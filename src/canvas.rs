use std::{os::raw::c_void, path::PathBuf};

use cocoa::{
    appkit::NSView,
    base::{id as cocoa_id, YES},
};
use core_graphics_types::geometry::CGSize;
use metal::{
    Buffer, CommandQueue, Device, Library, MTLOrigin, MTLRegion, MTLResourceOptions, MTLSize,
    MetalLayer, RenderPassDescriptor, RenderPassDescriptorRef, RenderPipelineDescriptor,
    RenderPipelineState, TextureDescriptor, TextureRef,
};
use winit::{
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::Window,
};

use crate::shader_bindings::{
    TextureIndex_TextureIndexBaseColor, TexturedVertex, VertexInputIndex_VertexInputIndexVertices,
    VertexInputIndex_VertexInputIndexViewportSize,
};

pub trait Canvas {
    fn draw(&self);
}

pub struct MetalCanvas {
    device: Device,
    library: Library,
    render_pipeline: RenderPipelineState,
    layer: MetalLayer,
    command_queue: CommandQueue,
}

impl Canvas for MetalCanvas {
    fn draw(&self) {}
}

impl MetalCanvas {
    pub fn new() -> Self {
        let device = Device::system_default().expect("No device found");
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);
        let library = device
            .new_library_with_file(
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("shaders.metallib"),
            )
            .unwrap();

        let pipeline_state_descriptor = RenderPipelineDescriptor::new();

        let vertex_shader = library.get_function("quad_vertex", None).unwrap();
        let fragment_shader = library.get_function("sampling_shader", None).unwrap();

        pipeline_state_descriptor.set_vertex_function(Some(&vertex_shader));
        pipeline_state_descriptor.set_fragment_function(Some(&fragment_shader));

        pipeline_state_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap()
            .set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);

        let render_pipeline = device
            .new_render_pipeline_state(&pipeline_state_descriptor)
            .unwrap();

        let command_queue = device.new_command_queue();

        Self {
            device,
            library,
            render_pipeline,
            layer,
            command_queue,
        }
    }

    fn texture(&self, img: *mut c_void, stride: u64, height: u64, width: u64) -> metal::Texture {
        let img_slice =
            unsafe { std::slice::from_raw_parts(img as *const u8, (stride * height) as usize) };
        let texture_desc = TextureDescriptor::new();
        texture_desc.set_width(width);
        texture_desc.set_height(height);
        texture_desc.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);

        let texture = self.device.new_texture(&texture_desc);
        texture.replace_region(
            MTLRegion {
                origin: MTLOrigin { x: 0, y: 0, z: 0 },
                size: MTLSize {
                    width,
                    height,
                    depth: 1,
                },
            },
            0,
            img_slice.as_ptr() as _,
            stride,
        );
        texture
    }

    pub fn redraw(&self, img: *mut c_void, stride: u64, height: u64, width: u64) {
        let texture = self.texture(img, stride, height, width);

        let drawable = match self.layer.next_drawable() {
            Some(drawable) => drawable,
            None => return,
        };

        let render_pass_desc = RenderPassDescriptor::new();
        prepare_render_pass_descriptor(&render_pass_desc, drawable.texture());
        let command_buffer = self.command_queue.new_command_buffer();
        let encoder = command_buffer.new_render_command_encoder(&render_pass_desc);
        encoder.set_render_pipeline_state(&self.render_pipeline);

        let vertex_data = vertices(width, height);
        let vertex_buffer = self.device.new_buffer_with_data(
            vertex_data.as_ptr() as *const _,
            (vertex_data.len() * std::mem::size_of::<TexturedVertex>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        );
        encoder.set_vertex_buffer(
            VertexInputIndex_VertexInputIndexVertices as u64,
            Some(&vertex_buffer),
            0,
        );

        let viewport_size_buffer = self.device.new_buffer(
            8,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        );

        let viewport_size = (width as u32, height as u32);

        update_viewport_size_buffer(&viewport_size_buffer, viewport_size);

        encoder.set_vertex_buffer(
            VertexInputIndex_VertexInputIndexViewportSize as u64,
            Some(&viewport_size_buffer),
            0,
        );
        encoder.set_fragment_texture(TextureIndex_TextureIndexBaseColor as u64, Some(&texture));

        encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, 0, 6);
        encoder.end_encoding();
        command_buffer.present_drawable(&drawable);
        command_buffer.commit();
    }

    pub fn set_window(&self, window: &Window) {
        self.layer.set_drawable_size(CGSize::new(
            window.inner_size().width as f64,
            window.inner_size().height as f64,
        ));
        unsafe {
            if let Ok(RawWindowHandle::AppKit(rw)) = window.window_handle().map(|wh| wh.as_raw()) {
                let view = rw.ns_view.as_ptr() as cocoa_id;
                view.setWantsLayer(YES);
                view.setLayer(std::mem::transmute(self.layer.as_ref()));
            }
        }
    }
}

fn update_viewport_size_buffer(viewport_size_buffer: &Buffer, size: (u32, u32)) {
    let contents = viewport_size_buffer.contents();
    let viewport_size: [u32; 2] = [size.0, size.1];
    let byte_count = (viewport_size.len() * std::mem::size_of::<u32>()) as usize;

    unsafe {
        std::ptr::copy(viewport_size.as_ptr(), contents as *mut u32, byte_count);
    }
    viewport_size_buffer.did_modify_range(metal::NSRange::new(0, byte_count as u64));
}

fn textured_vertex(position: [f32; 2], texture_coord: [f32; 2]) -> TexturedVertex {
    unsafe {
        // The metal shader is expecting two floats, but the rust-bindgen generated
        // type is a u64.
        //
        // So, we transmute the 2 floats into a u64 so that when the shader receives
        // these 64 bits they'll be interpreted as a `vector_float2`.
        TexturedVertex {
            position: std::mem::transmute(position),
            texture_coord: std::mem::transmute(texture_coord),
        }
    }
}

fn vertices(w: u64, h: u64) -> [TexturedVertex; 6] {
    let x = w as f32 / 2.;
    let y = h as f32 / 2.;
    [
        textured_vertex([-x, -y], [0., 1.]),
        textured_vertex([x, -y], [1., 1.]),
        textured_vertex([x, y], [1., 0.]),
        textured_vertex([-x, -y], [0., 1.]),
        textured_vertex([x, y], [1., 0.]),
        textured_vertex([-x, y], [0., 0.]),
    ]
}

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(metal::MTLLoadAction::Clear);
    color_attachment.set_clear_color(metal::MTLClearColor::new(0.2, 0.5, 0.8, 1.0));
    color_attachment.set_store_action(metal::MTLStoreAction::Store);
}

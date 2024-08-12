#include "channel.h"

MainChannel *new_main_channel(Session *session)
{
    MainChannel *main_channel = malloc(sizeof(MainChannel));
    SpiceChannel *spice_chanel = spice_channel_new(session->session, SPICE_CHANNEL_MAIN, 1);
    SpiceMainChannel *spice_main_channel = SPICE_MAIN_CHANNEL(spice_chanel);
    main_channel->main_channel = spice_main_channel;
    return main_channel;
}

void on_gl_draw(SpiceDisplayChannel *display_channel, guint x, guint y, guint width, guint height, gpointer user_data) {
    g_message("GL draw");
}

DisplayChannel *new_display_channel(Session *session)
{
    DisplayChannel *display_channel = malloc(sizeof(DisplayChannel));
    SpiceChannel *spice_channel = spice_channel_new(session->session, SPICE_CHANNEL_DISPLAY, 2);
    display_channel->display_channel = SPICE_DISPLAY_CHANNEL(spice_channel);
    g_signal_connect(display_channel->display_channel, "gl-draw", G_CALLBACK(on_gl_draw), NULL);
    return display_channel;
}

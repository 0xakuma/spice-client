#include "connection.h"
#include "channel-inputs.h"
#include "channel-main.h"
#include "glib-object.h"
#include "glib.h"
#include "glibconfig.h"
#include "spice/enums.h"

void main_channel_events(SpiceChannel *channel, SpiceChannelEvent event,
                         gpointer user_data) {
  g_message("Channel event");

  switch (event) {
  case SPICE_CHANNEL_OPENED:
    g_print("Channel opened: %s\n", G_OBJECT_TYPE_NAME(channel));
    break;
  case SPICE_CHANNEL_CLOSED:
    g_print("Channel closed: %s\n", G_OBJECT_TYPE_NAME(channel));
    break;
  case SPICE_CHANNEL_ERROR_CONNECT:
    g_print("Channel connect error: %s\n", G_OBJECT_TYPE_NAME(channel));
    break;
  case SPICE_CHANNEL_ERROR_TLS:
    g_print("Channel TLS error: %s\n", G_OBJECT_TYPE_NAME(channel));
    break;
  default:
    g_print("Channel event: %d\n", event);
    break;
  }

  if (event == SPICE_CHANNEL_ERROR_CONNECT) {
    g_error("SPICE_CHANNEL Connection failed");
  }
}

void primary_display_event(SpiceDisplayChannel *display, gint format,
                           gint width, gint height, gint stride, gint shmid,
                           gpointer imgdata, gpointer user_data) {
  SpiceConnection *connection = (SpiceConnection *)user_data;
  g_message("Display channel called");
  connection->callback();
}

void main_agent_update(SpiceMainChannel *channel, gpointer user_data) {
  g_message("Spice channel connected");
}

void on_display_invalidate(SpiceDisplayChannel *display, gint x, gint y,
                           gint width, gint height, gpointer user_data) {
  g_message("Display invalidate");
}

void on_fd_open(SpiceChannel *channel, gint with_tls, gpointer user_data) {
  g_message("fd opened");
}

void new_channel(SpiceSession *session, SpiceChannel *channel,
                 gpointer user_data) {
  int chid;
  SpiceConnection *connection = (SpiceConnection *)user_data;

  g_signal_connect(channel, "open-fd", G_CALLBACK(on_fd_open), user_data);

  g_object_get(channel, "channel-id", &chid, NULL);
  if (SPICE_IS_MAIN_CHANNEL(channel)) {
    g_message("MAIN channel");
    if (channel->priv != NULL) {
      SpiceMainChannel *main_channel =
          g_object_ref(SPICE_MAIN_CHANNEL(channel));
      g_signal_connect(channel, "channel-event",
                       G_CALLBACK(main_channel_events), NULL);
      g_signal_connect(channel, "notify::agent-connected",
                       G_CALLBACK(main_agent_update), NULL);
      // connection->display_channel = new_display_channel(connection->session);
    }
  }

  if (SPICE_IS_DISPLAY_CHANNEL(channel)) {
    gint channel_id;
    SpiceDisplayPrimary primary_display;
    g_object_get(channel, "channel-id", &channel_id, NULL);
    g_message("Display channel %d", channel_id);
    SpiceDisplayChannel *display_channel = SPICE_DISPLAY_CHANNEL(channel);

    g_signal_connect(display_channel, "display-primary-create",
                     G_CALLBACK(primary_display_event), connection);
    g_signal_connect(display_channel, "display-invalidate",
                     G_CALLBACK(on_display_invalidate), NULL);
    if (spice_display_channel_get_primary(channel, 0, &primary_display)) {
      g_message("Primary display");
    }
    spice_channel_connect(channel);
  }

  if (SPICE_IS_CURSOR_CHANNEL(channel)) {
    g_message("Cursor channel");
  }

  if (SPICE_IS_INPUTS_CHANNEL(channel)) {
    g_message("Input channel");
  }

  if (SPICE_IS_PORT_CHANNEL(channel)) {
    g_message("PORT channel");
  }
}

void destroy_channel(SpiceSession *session, SpiceChannel *channel,
                     gpointer user_data) {}

SpiceConnection *new_connection(gchar *host, gchar *port, Callback callback) {
  SpiceConnection *connection = malloc(sizeof(SpiceConnection));

  connection->session = spice_session_new();
  spice_set_session_option(connection->session);
  connection->callback = callback;
  static gchar *port_str;
  static gchar *tls_port_str;
  static gchar *uri;

  port_str = g_strdup_printf("%d", 5930);
  tls_port_str = g_strdup_printf("%d", 0);

  g_object_set(connection->session, "port", port_str, NULL);
  g_object_set(connection->session, "enable-usbredir", FALSE, NULL);
  g_object_set(connection->session, "client-sockets", TRUE, NULL);

  g_signal_connect(connection->session, "channel-new", G_CALLBACK(new_channel),
                   connection);
  g_signal_connect(connection->session, "channel-destroy",
                   G_CALLBACK(destroy_channel), NULL);

  // connection->main_channel = SPICE_MAIN_CHANNEL(
  //     spice_channel_new(connection->session, SPICE_CHANNEL_MAIN, 0));
  // connection->display_channel = SPICE_DISPLAY_CHANNEL(
  //     spice_channel_new(connection->session, SPICE_CHANNEL_DISPLAY, 2));
  // if (!connection->display_channel) {
  //   g_print("Failed to create display channel\n");
  // }

  // connection->input_channel = SPICE_INPUTS_CHANNEL(
  //     spice_channel_new(connection->session, SPICE_CHANNEL_INPUTS, 3));
  // if (!connection->input_channel) {
  //   g_print("Failed to create inputs channel\n");
  // }

  return connection;
}

gboolean channel_connect(SpiceConnection *connection) {
  return spice_session_connect(connection->session);
}

#include "connection.h"
#include "glib.h"
#include "spice-util.h"

void callback() {
    g_message("Callback");
}

int main()
{
    spice_util_set_debug(TRUE);
    GMainContext *ctx = g_main_context_new();
    GMainLoop *loop = g_main_loop_new(ctx, FALSE);

    gchar *port = "5930";
    gchar *host = "localhost";

    SpiceConnection *connection = new_connection(host, port, callback);
    gboolean rt = channel_connect(connection);
    GList* list = spice_session_get_channels(connection->session);
    guint l = g_list_length(list);
    g_message("glist length %d", l);
    if (rt)
    {
        g_message("Connection success");
    }
    else
    {
        g_message("connection failed");
    }

    g_main_loop_run(loop);
    g_main_loop_unref(loop);
    return 0;
}

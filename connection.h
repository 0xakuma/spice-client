#include "spice-client.h"
#include "glib.h"
#include "channel.h"

struct _SpiceConnection
{
    MainChannel main_channel;
    Session *session;
} typedef SpiceConnection;

SpiceConnection new_connection(gchar *uri);
gboolean channel_connect(SpiceConnection *connection);
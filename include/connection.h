#include "channel-inputs.h"
#include "channel-main.h"
#include "spice-client.h"
#include "glib.h"
#include "channel.h"

typedef void (*Callback)();

struct _SpiceConnection
{
    SpiceMainChannel *main_channel;
    SpiceSession *session;
    SpiceDisplayChannel *display_channel;
    SpiceInputsChannel *input_channel;
    Callback callback;
} typedef SpiceConnection;

SpiceConnection *new_connection(gchar *host, gchar *port, Callback);
gboolean channel_connect(SpiceConnection *connection);

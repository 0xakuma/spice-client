#include "connection.h"
#include "session.h"

SpiceConnection new_connection(gchar *uri)
{
    SpiceConnection connection;
    Session session = new_session();
    MainChannel channel = new_main_channel(&session);
    connection.main_channel = channel;
    connection.session = &session;
    return connection;
}

gboolean connect(SpiceConnection *connection)
{
    return session_connect(connection->session);
}
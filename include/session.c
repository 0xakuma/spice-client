#include "session.h"
#include "glib-object.h"

Session *new_session()
{
    Session *session = malloc(sizeof(Session));
    session->session = spice_session_new();
    return session;
}

void set_host(Session *session, gchar *host)
{
    g_object_set(session->session, "host", host, NULL);
}

void set_port(Session *session, gchar *port)
{
    g_object_set(session->session, "port", port, NULL);
}

gboolean session_connect(Session *session)
{
    return spice_session_connect(session->session);
}

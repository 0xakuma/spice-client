#include "session.h"

Session new_session()
{
    Session session;
    SpiceSession *spice_session = spice_session_new();
    session.session = spice_session;
    return session;
}

void set_uri(Session *session, gchar *uri)
{
    g_object_set(session->session, "uri\0", uri, NULL);
}

gboolean session_connect(Session *session)
{
    return spice_session_connect(session->session);
}
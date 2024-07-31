#ifndef SESSION_H
#define SESSION_H

#include "spice-client.h"
#include "string.h"
#include "glib.h"
#include "gobject/gobject.h"

struct _Session
{
    SpiceSession *session;
} typedef Session;

Session new_session();
void set_uri(Session *, gchar *);
gboolean session_connection(Session *session);

#endif // SESSION_H
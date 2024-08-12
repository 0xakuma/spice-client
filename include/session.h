#ifndef SESSION_H
#define SESSION_H

#include "spice-client.h"
#include "glib.h"
#include "gobject/gobject.h"

struct _Session
{
    SpiceSession *session;
} typedef Session;

Session *new_session();
void set_host(Session *, gchar *);
void set_port(Session *, gchar *);
gboolean session_connect(Session *session);

#endif

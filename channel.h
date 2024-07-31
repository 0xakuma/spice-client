#ifndef CHANNEL_H
#define CHANNEL_H

#include "spice-client.h"
#include "session.h"

struct _MainChannel
{
    SpiceMainChannel *main_channel;
} typedef MainChannel;

struct _DisplayChannel
{
    SpiceDisplayChannel *display_channel;
} typedef DisplayChannel;

MainChannel new_main_channel(MainChannel *main_channel);
DisplayChannel new_display_channel(Session *session);

#endif
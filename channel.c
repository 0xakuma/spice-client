#include "channel.h"

MainChannel new_main_channel(Session *session)
{
    MainChannel main_channel;
    SpiceChannel *spice_chanel = spice_channel_new(session->session, SPICE_CHANNEL_MAIN, 1);
    SpiceMainChannel *spice_main_channel = SPICE_MAIN_CHANNEL(spice_chanel);
    main_channel.main_channel = spice_main_channel;
    return main_channel;
}

DisplayChannel new_display_channel(Session *session)
{
    DisplayChannel display_channel;
    SpiceChannel *spice_channel = spice_channel_new(session->session, SPICE_CHANNEL_DISPLAY, 2);
    SpiceDisplayChannel *spice_display_channel = SPICE_DISPLAY_CHANNEL(spice_channel);
    display_channel.display_channel = spice_display_channel;
    return display_channel;
}
#include <stdio.h>
#include "channel.h"

int main()
{
    printf("DEBUG");
    Session session = new_session();
    MainChannel channel = new_main_channel(&session);
    return 0;
}
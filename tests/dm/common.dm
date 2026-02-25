#define ICONFORGE_RS world.GetConfig("env", "ICONFORGE_RS")
#include "iconforge_rs.dm"

/world/New()
    for(var/func in typesof(/test/proc))
        log << "[func] [copytext("------------------------------------------------------------------------", length("[func]"))]"
        call(new /test, func)()
    del src

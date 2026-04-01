#define ICONFORGE world.GetConfig("env", "ICONFORGE")
#include "iconforge.dm"

/world/New()
    for(var/func in typesof(/test/proc))
        log << "[func] [copytext("------------------------------------------------------------------------", length("[func]"))]"
        call(new /test, func)()
    del src

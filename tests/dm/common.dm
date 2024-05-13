#define RUST_UTILS world.GetConfig("env", "RUST_UTILS")
#include "../../target/rust_utils.dm"

/world/New()
    world.log << RUST_UTILS
    for(var/func in typesof(/test/proc))
        log << "[func] [copytext("------------------------------------------------------------------------", length("[func]"))]"
        call(new /test, func)()
    del src

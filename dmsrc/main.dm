#ifndef RUST_UTILS

/* This comment bypasses grep checks */ /var/__rust_utils

/proc/__detect_rust_utils()
	if (world.system_type == UNIX)
		if (fexists("./librust_utils.so"))
			// No need for LD_LIBRARY_PATH badness.
			return __rust_utils = "./librust_utils.so"
		else if (fexists("./rust_utils"))
			// Old dumb filename.
			return __rust_utils = "./rust_utils"
		else if (fexists("[world.GetConfig("env", "HOME")]/.byond/bin/rust_utils"))
			// Old dumb filename in `~/.byond/bin`.
			return __rust_utils = "rust_utils"
		else
			// It's not in the current directory, so try others
			return __rust_utils = "librust_utils.so"
	else
		return __rust_utils = "librust_utils"

#define RUST_UTILS (__rust_utils || __detect_rust_utils())
#endif

// Handle 515 call() -> call_ext() changes
#if DM_VERSION >= 515
#define CALL_LIB call_ext
#else
#define CALL_LIB call
#endif

/// Gets the version of rust_utils
/proc/rust_utils_get_version() return CALL_LIB(RUST_UTILS, "get_version")()

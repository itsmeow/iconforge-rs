// iconforge.dm - DM API for iconforge extension library
//
// To configure, create a `iconforge.config.dm` and set what you care about from
// the following options:
//
// #define ICONFORGE "path/to/iconforge"
// Override the .dll/.so detection logic with a fixed path or with detection
// logic of your own.

#ifndef ICONFORGE
// Default automatic ICONFORGE detection.
// On Windows, looks in the standard places for `iconforge.dll`.
// On Linux, looks in `.`, `$LD_LIBRARY_PATH`, and `~/.byond/bin` for either of
// `libiconforge.so` (preferred) or `iconforge` (old).

/* This comment bypasses grep checks */ /var/__iconforge_binary

/proc/__detect_iconforge()
	var/arch_suffix = null
	#ifdef OPENDREAM
	arch_suffix = "64"
	#endif
	if (world.system_type == UNIX)
		if (fexists("./libiconforge[arch_suffix].so"))
			// No need for LD_LIBRARY_PATH badness.
			return __iconforge_binary = "./libiconforge[arch_suffix].so"
		else if (fexists("./iconforge[arch_suffix]"))
			// Old dumb filename.
			return __iconforge_binary = "./iconforge[arch_suffix]"
		else if (fexists("[world.GetConfig("env", "HOME")]/.byond/bin/iconforge[arch_suffix]"))
			// Old dumb filename in `~/.byond/bin`.
			return __iconforge_binary = "iconforge[arch_suffix]"
		else
			// It's not in the current directory, so try others
			return __iconforge_binary = "libiconforge[arch_suffix].so"
	else
		return __iconforge_binary = "iconforge[arch_suffix]"

#define ICONFORGE (__iconforge_binary || __detect_iconforge())
#endif

// Handle 515 call() -> call_ext() changes
#if DM_VERSION >= 515
#define ICONFORGE_CALL call_ext
#else
#define ICONFORGE_CALL call
#endif

/// Gets the version of iconforge
/proc/iconforge_get_version() return ICONFORGE_CALL(ICONFORGE, "get_version")()

#define ICONFORGE_JOB_NO_RESULTS_YET "NO RESULTS YET"
#define ICONFORGE_JOB_NO_SUCH_JOB "NO SUCH JOB"
#define ICONFORGE_JOB_ERROR "JOB PANICKED"

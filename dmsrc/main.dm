// iconforge_rs.dm - DM API for iconforge_rs extension library
//
// To configure, create a `iconforge_rs.config.dm` and set what you care about from
// the following options:
//
// #define ICONFORGE_RS "path/to/iconforge_rs"
// Override the .dll/.so detection logic with a fixed path or with detection
// logic of your own.

#ifndef ICONFORGE_RS
// Default automatic ICONFORGE_RS detection.
// On Windows, looks in the standard places for `iconforge_rs.dll`.
// On Linux, looks in `.`, `$LD_LIBRARY_PATH`, and `~/.byond/bin` for either of
// `libiconforge_rs.so` (preferred) or `iconforge_rs` (old).

/* This comment bypasses grep checks */ /var/__iconforge_rs

/proc/__detect_iconforge_rs()
	var/arch_suffix = null
	#ifdef OPENDREAM
	arch_suffix = "64"
	#endif
	if (world.system_type == UNIX)
		if (fexists("./libiconforge_rs[arch_suffix].so"))
			// No need for LD_LIBRARY_PATH badness.
			return __iconforge_rs = "./libiconforge_rs[arch_suffix].so"
		else if (fexists("./iconforge_rs[arch_suffix]"))
			// Old dumb filename.
			return __iconforge_rs = "./iconforge_rs[arch_suffix]"
		else if (fexists("[world.GetConfig("env", "HOME")]/.byond/bin/iconforge_rs[arch_suffix]"))
			// Old dumb filename in `~/.byond/bin`.
			return __iconforge_rs = "iconforge_rs[arch_suffix]"
		else
			// It's not in the current directory, so try others
			return __iconforge_rs = "libiconforge_rs[arch_suffix].so"
	else
		return __iconforge_rs = "iconforge_rs[arch_suffix]"

#define ICONFORGE_RS (__iconforge_rs || __detect_iconforge_rs())
#endif

// Handle 515 call() -> call_ext() changes
#if DM_VERSION >= 515
#define ICONFORGE_RS_CALL call_ext
#else
#define ICONFORGE_RS_CALL call
#endif

/// Gets the version of iconforge_rs
/proc/iconforge_rs_get_version() return ICONFORGE_RS_CALL(ICONFORGE_RS, "get_version")()

#define ICONFORGE_RS_JOB_NO_RESULTS_YET "NO RESULTS YET"
#define ICONFORGE_RS_JOB_NO_SUCH_JOB "NO SUCH JOB"
#define ICONFORGE_RS_JOB_ERROR "JOB PANICKED"

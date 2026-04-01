#[cfg(feature = "ffi")]
use std::{
	fs,
	path::Path,
	process::{Command, Output},
};

#[cfg(feature = "ffi")]
pub mod iconforge;

/**
 * Find a valid BYOND bin path on the system.
 */
#[cfg(feature = "ffi")]
fn find_byond() -> String {
	match std::env::var("BYOND_BIN") {
		Ok(bin) => bin,
		Err(_) => {
			let paths = vec![
				"C:/Program Files (x86)/BYOND/bin",
				"C:/Program Files/BYOND/bin",
			];
			let mut found_path = None;
			for path in paths {
				if let Ok(exists) = fs::exists(Path::new(path)) {
					if exists {
						found_path = Some(path.to_string());
						break;
					} else {
						continue;
					}
				} else {
					continue;
				}
			}
			found_path.expect(
				"Could not find environment variable BYOND_BIN, or any valid installation path \
				 for BYOND.",
			)
		}
	}
}

#[cfg(feature = "ffi")]
fn use_byond_executable<F>(byond_bin: &str, windows: &str, linux: &str, command: F) -> Output
where
	F: Fn(&mut Command) -> &mut Command,
{
	if cfg!(target_os = "linux") {
		let byondexec = format!("{byond_bin}/byondexec");
		let linux_full = format!("{byond_bin}/{linux}");
		command(Command::new("bash").arg(&byondexec).arg(&linux_full))
			.output()
			.unwrap()
	} else {
		let windows_full = format!("{byond_bin}/{windows}");
		command(&mut Command::new(&windows_full)).output().unwrap()
	}
}

#[cfg(feature = "ffi")]
fn compile_and_run_dme(name: &str, iconforge_rs_lib_path: &str, chdir: Option<&str>) -> Output {
	let byond_bin = find_byond();

	let dme = format!("tests/dm/{name}.dme");
	let dmb = format!("tests/dm/{name}.dmb");

	let output = use_byond_executable(&byond_bin, "dm", "DreamMaker", |c| c.arg(&dme));
	dump(&output);
	generic_check(&output);

	let output = use_byond_executable(&byond_bin, "dd", "DreamDaemon", |c| {
		c.arg(&dmb)
			.arg("-trusted")
			.arg("-cd")
			.arg(chdir.unwrap_or("."))
			.env("ICONFORGE", iconforge_rs_lib_path)
	});

	// Cleanup
	let _ = std::fs::remove_file(&dmb);
	let _ = std::fs::remove_file(format!("tests/dm/{name}.rsc"));
	let _ = std::fs::remove_file(format!("tests/dm/{name}.dyn.rsc"));
	let _ = std::fs::remove_file(format!("tests/dm/{name}.lk"));
	let _ = std::fs::remove_file(format!("tests/dm/{name}.int"));

	dump(&output);
	generic_check(&output);
	output
}

/**
 * Find the iconforge binary and DMSRC and copy them into the test run
 * directory
 */
#[cfg(feature = "ffi")]
fn find_and_copy_iconforge_lib() -> (String, &'static str, &'static str) {
	let target_dir = if cfg!(target_os = "linux") {
		"i686-unknown-linux-gnu"
	} else {
		"i686-pc-windows-msvc"
	};
	let profile = if cfg!(debug_assertions) {
		"debug"
	} else {
		"release"
	};
	let iconforge_rs_lib_fname = if cfg!(target_os = "linux") {
		"libiconforge.so"
	} else {
		"iconforge.dll"
	};
	let iconforge_rs_lib_source_path =
		format!("target/{target_dir}/{profile}/{iconforge_rs_lib_fname}");
	println!("Source ICONFORGE path: {iconforge_rs_lib_source_path}");
	match fs::exists(Path::new(&iconforge_rs_lib_source_path)) {
		Ok(exists) => {
			if !exists {
				panic!(
					"Source ICONFORGE path does not exist! Try rebuilding the project with the \
					 corresponding target and debug or release mode."
				)
			}
		}
		Err(err) => panic!("Error accessing source iconforge path! {err}"),
	}
	let iconforge_rs_lib_path = format!("tests/dm/{iconforge_rs_lib_fname}");
	let _ = fs::copy(&iconforge_rs_lib_source_path, &iconforge_rs_lib_path);
	let iconforge_rs_dm_path = "tests/dm/iconforge.dm";
	let _ = fs::copy("target/iconforge.dm", iconforge_rs_dm_path);
	(
		iconforge_rs_lib_path,
		iconforge_rs_lib_fname,
		iconforge_rs_dm_path,
	)
}

#[cfg(feature = "ffi")]
fn run_dm_tests(name: &str, use_repo_root: bool) {
	let (iconforge_rs_lib_path, iconforge_rs_lib_fname, iconforge_rs_dm_path) =
		find_and_copy_iconforge_lib();

	let output = compile_and_run_dme(
		name,
		if use_repo_root {
			&iconforge_rs_lib_path
		} else {
			iconforge_rs_lib_fname
		},
		if use_repo_root {
			Some(env!("CARGO_MANIFEST_DIR"))
		} else {
			None
		},
	);
	runtime_check(&output);

	// Cleanup
	let _ = std::fs::remove_file(&iconforge_rs_lib_path);
	let _ = std::fs::remove_file(iconforge_rs_dm_path);
}

#[cfg(feature = "ffi")]
fn dump(output: &Output) {
	print!("{}", String::from_utf8_lossy(&output.stdout));
	eprint!("{}", String::from_utf8_lossy(&output.stderr));
}

#[cfg(feature = "ffi")]
fn generic_check(output: &Output) {
	if !output.status.success() {
		panic!("process exited with {:?}", output.status);
	}
}

#[cfg(feature = "ffi")]
fn runtime_check(output: &Output) {
	for line in output.stderr.split(|&c| c == b'\n') {
		if line.starts_with(b"runtime error: ") {
			panic!("{}", String::from_utf8_lossy(line));
		}
	}
}

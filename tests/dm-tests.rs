use std::{
	fs,
	path::Path,
	process::{Command, Output},
};

pub mod iconforge;

/**
 * Find a valid BYOND bin path on the system.
 */
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

fn compile_and_run_dme(name: &str, iconforge_rs_lib_path: &str, chdir: Option<&str>) -> Output {
	let byond_bin = find_byond();
	let dream_maker = format!("{byond_bin}/dm");
	let dream_daemon = format!("{byond_bin}/dd");

	let dme = format!("tests/dm/{name}.dme");
	let dmb = format!("tests/dm/{name}.dmb");

	let output = Command::new(&dream_maker).arg(&dme).output().unwrap();
	dump(&output);
	generic_check(&output);

	let output = Command::new(&dream_daemon)
		.arg(&dmb)
		.arg("-trusted")
		.arg("-cd")
		.arg(chdir.unwrap_or("."))
		.env("ICONFORGE_RS", iconforge_rs_lib_path)
		.output()
		.unwrap();

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
 * Find the iconforge_rs binary and DMSRC and copy them into the test run
 * directory
 */
fn find_and_copy_iconforge_rs_lib() -> (String, &'static str, &'static str) {
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
		"libiconforge_rs.so"
	} else {
		"iconforge_rs.dll"
	};
	let iconforge_rs_lib_source_path =
		format!("target/{target_dir}/{profile}/{iconforge_rs_lib_fname}");
	println!("Source ICONFORGE_RS path: {iconforge_rs_lib_source_path}");
	match fs::exists(Path::new(&iconforge_rs_lib_source_path)) {
		Ok(exists) => {
			if !exists {
				panic!(
					"Source ICONFORGE_RS path does not exist! Try rebuilding the project with the \
					 corresponding target and debug or release mode."
				)
			}
		}
		Err(err) => panic!("Error accessing source iconforge_rs path! {err}"),
	}
	let iconforge_rs_lib_path = format!("tests/dm/{iconforge_rs_lib_fname}");
	let _ = fs::copy(&iconforge_rs_lib_source_path, &iconforge_rs_lib_path);
	let iconforge_rs_dm_path = "tests/dm/iconforge_rs.dm";
	let _ = fs::copy("target/iconforge_rs.dm", iconforge_rs_dm_path);
	(
		iconforge_rs_lib_path,
		iconforge_rs_lib_fname,
		iconforge_rs_dm_path,
	)
}

fn run_dm_tests(name: &str, use_repo_root: bool) {
	let (iconforge_rs_lib_path, iconforge_rs_lib_fname, iconforge_rs_dm_path) =
		find_and_copy_iconforge_rs_lib();

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

fn dump(output: &Output) {
	print!("{}", String::from_utf8_lossy(&output.stdout));
	eprint!("{}", String::from_utf8_lossy(&output.stderr));
}

fn generic_check(output: &Output) {
	if !output.status.success() {
		panic!("process exited with {:?}", output.status);
	}
}

fn runtime_check(output: &Output) {
	for line in output.stderr.split(|&c| c == b'\n') {
		if line.starts_with(b"runtime error: ") {
			panic!("{}", String::from_utf8_lossy(line));
		}
	}
}

//! Buildscript which will save a `iconforge.dm` with the DLL's public API.

use std::{env, fs::File, io::Write, path::Path};

macro_rules! feature_dm_file {
	($name:expr) => {
		&"dmsrc/{}.dm".replace("{}", $name)
	};
}

fn main() {
	let target_dir = env::var("DM_OUT_DIR")
		.unwrap_or_else(|_| env::var("OUT_DIR").unwrap_or_else(|_| String::from(".")));
	if target_dir.trim().is_empty() {
		return;
	}
	let dm_path = Path::new(&target_dir).join("iconforge.dm");
	let mut f = File::create(&dm_path).unwrap_or_else(|_| {
		panic!(
			"Couldn't open `{}` for writing iconforge-rs DM headers. Set DM_OUT_DIR to an empty \
			 string to disable writing headers or to an absolute path you want to write headers \
			 to.",
			dm_path.display()
		)
	});

	// header
	writeln!(
		f,
		"{}",
		std::fs::read_to_string(feature_dm_file!("main")).unwrap()
	)
	.unwrap();

	writeln!(
		f,
		"{}",
		std::fs::read_to_string(feature_dm_file!("iconforge")).unwrap()
	)
	.unwrap();

	writeln!(
		f,
		"{}",
		std::fs::read_to_string(feature_dm_file!("universal_icon")).unwrap()
	)
	.unwrap();
}

//! Buildscript which will save a `iconforge_rs.dm` with the DLL's public API.

use std::{fs::File, io::Write};

macro_rules! feature_dm_file {
    ($name:expr) => {
        &"dmsrc/{}.dm".replace("{}", $name)
    };
}

fn main() {
    let mut f = File::create("target/iconforge_rs.dm").unwrap();

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

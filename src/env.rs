use std::{path::PathBuf, sync::OnceLock};

use colored::Colorize;

static EXTRA_PATH: OnceLock<Vec<PathBuf>> = OnceLock::new();

pub fn prepere_deps() {
    #[cfg(target_os = "windows")]
    windows();
}

#[cfg(target_os = "windows")]
fn windows() {
    let mut mysys2_root = PathBuf::from("C:\\msys64");
    if let Some(p) = std::env::var_os("MSYS2_ROOT") {
        mysys2_root = PathBuf::from(p);
    }

    if !mysys2_root.exists() {
        println!("{}", "MSYS2 not found!".yellow());
        return;
    }

    println!("{}", "MSYS2 found!".green());

    let ucrt64 = mysys2_root.join("ucrt64/bin");
    let mingw64 = mysys2_root.join("mingw64/bin");
    let usr_bin = mysys2_root.join("usr/bin");

    let mut path = vec![usr_bin];

    if ucrt64.join("qemu-system-x86_64.exe").exists() {
        path.push(ucrt64);
    } else if mingw64.join("qemu-system-x86_64.exe").exists() {
        path.push(mingw64);
    } else {
        println!("{}", "QEMU not found!".yellow());
    }

    let _ = EXTRA_PATH.set(path);
}

pub fn get_extra_path() -> Vec<PathBuf> {
    EXTRA_PATH.get_or_init(Vec::new).clone()
}

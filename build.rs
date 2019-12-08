use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out_dir.join("com.ld")).unwrap().write_all(include_bytes!("com.ld")).unwrap();
    Command::new("i386-elf-gcc")
        .args(&["startup.c", "-c", "-march=i386", "-m16", "-ffreestanding", "-fno-pie", "-o"])
        .arg(out_dir.join("startup.o"))
        .status().unwrap();
}

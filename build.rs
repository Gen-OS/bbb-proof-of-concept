// build.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.
//

extern crate gcc;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let dst = env::var_os("OUT_DIR").unwrap();
    let src_path = Path::new("src/");
    let dest_path = Path::new(&dst);

    let _ = fs::copy(src_path.join("cpu/am335x/linker.ld"),
                     dest_path.join("linker.ld"));
    let _ = fs::copy(src_path.join("arch/thumbv7a/arch_universal.ld"),
                     dest_path.join("arch_universal.ld"));

    gcc::Config::new()
        .compiler("arm-none-eabi-gcc")
        .flag("-mcpu=cortex-a8")
        .flag("-mfpu=neon")
        .flag("-mfloat-abi=softfp")
        .file("src/arch/thumbv7a/stage0.S")
        .include("src/arch/thumbv7a")
        .compile("libstage0.a");

    gcc::Config::new()
        .compiler("arm-none-eabi-gcc")
        .flag("-mcpu=cortex-a8")
        .flag("-mfpu=neon")
        .flag("-mfloat-abi=softfp")
        .file("src/arch/thumbv7a/libc_stubs/memcmp.S")
        .file("src/arch/thumbv7a/libc_stubs/memcpy_arm.S")
        .file("src/arch/thumbv7a/libc_stubs/memmove.S")
        .file("src/arch/thumbv7a/libc_stubs/memset.S")
        .include("src/arch/thumbv7a/libc_stubs")
        .compile("libc_stubs.a");

}
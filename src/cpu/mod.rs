// src/cpu/mods.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.

// CPU-specific modules
#[cfg(all(target_arch = "thumbv7a", feature = "cpu-am335x"))]
pub use self::am335x::*;

pub mod am335x;
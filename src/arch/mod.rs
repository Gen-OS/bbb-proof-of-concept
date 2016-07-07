// src/arch/mods.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.

// Platform-specific stuff
#[cfg(target_arch = "thumbv7a")]
pub use self::thumbv7a::*;

// Platform modules:
pub mod thumbv7a; // - ARMv7-A ISA in Thumb mode

// src/cpu/generic/mod.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.

use core::intrinsics::{volatile_load, volatile_store};

pub struct Register<T> {
    pub reg: *mut T, /* This is public, but only because it has to be. Consider it an implementation detail. */
}

impl<T> Register<T> {
    pub fn get(self) -> T {
        unsafe { volatile_load(self.reg) }
    }

    pub fn set(self, val: T) {
        unsafe { volatile_store(self.reg, val) }
    }
}

#[macro_export]
macro_rules! register_map {
    (pub $prefix:ident => from $base:expr, $size:ty: $($name:ident => $offset:expr)+) => ( interpolate_idents! {
        #[allow(non_camel_case_types)]
        pub struct [$prefix _T] {
            $(pub $name : register_map::Register<$size>,)+
        }

        #[allow(non_upper_case_globals)]
        pub const $prefix : [$prefix _T] = [$prefix _T] {
            $($name: register_map::Register{reg: (($base + $offset) as *mut $size)},)+
        };
    } );
    ($prefix:ident => from $base:expr, $size:ty: $($name:ident => $offset:expr)+) => ( interpolate_idents! {
        #[allow(non_camel_case_types)]
        struct [$prefix _T] {
            $(pub $name : register_map::Register<$size>,)+
        }

        #[allow(non_upper_case_globals)]
        const $prefix : [$prefix _T] = [$prefix _T] {
            $($name: register_map::Register{reg: (($base + $offset) as *mut $size)},)+
        };
    } );
}

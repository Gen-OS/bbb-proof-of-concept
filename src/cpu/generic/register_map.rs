// src/cpu/generic/mod.rs
// Copyright 2016 Alexis Williams
//
// Licensed under the MIT License <http://opensource.org/licenses/MIT>.


#[macro_export]
macro_rules! register_map {
    ($prefix:ident => from $base:expr, $size:ty: $($name:ident => $offset:expr)+) => ( interpolate_idents! {
        #[allow(non_camel_case_types)]
        struct [$prefix _T] {
            $($name : *const $size,)+
        }

        const $prefix : [$prefix _T] = [$prefix _T] {
            $($name: ($base + $offset) as *const $size,)+
        };
    } )
}
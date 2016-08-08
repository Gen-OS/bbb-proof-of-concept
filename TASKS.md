#TODO

- [x] Get Cargo to be able to properly link in the stage0 loader on its own. (This will involve working with the Cargo developers to get them to add the ability to configure the toolchain (global linkerflags) in a crate.)
    - Turns out that it was so much easier than that. However, with how fast Rust moves I don't know if it was when I wrote that. Oh well.
- [x] Figure out how to actually get Rust to emit hardfp code/use the floating point registers.
    - I think that this has always been happening, rustc just doesn't talk about it in the ELF header like it should.
- [x] Figure out how to tell GNU as that the code follows the right conventions for the hard FP ABI/get it to add the right flags to the ELF objects it outputs.
    - Not technically done, but it's not needed now that `compiler-rt` is being used instead of trying to force `libgcc` into a square hole. 

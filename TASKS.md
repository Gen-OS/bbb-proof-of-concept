#TODO

- [ ] Get Cargo to be able to properly link in the stage0 loader on its own. (This will involve working with the Cargo developers to get them to add the ability to configure the toolchain (global linkerflags) in a crate.)
- [ ] Figure out how to actually get Rust to emit hardfp code/use the floating point registers.
- [ ] Figure out how to tell GNU as that the code follows the right conventions for the hard FP ABI/get it to add the right flags to the ELF objects it outputs.


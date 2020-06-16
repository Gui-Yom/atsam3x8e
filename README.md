# atsam3x8e
Rust support crate for the atsam3x8e chip.

Could also probably work with near variants, but not no guaranties.
The main goal is to support the Arduino Due cpu.

Generated with svd2rust 0.17.0 (714ed98 2020-06-09) and then patched by hand.

#### Manual Patches
Patch made are documented here and in the source. The patches are made to allow
the crate to compile *(essential)* and to prevent *(warnings)* during compilation.
 - (`cargo fmt`)
 - `src/chipid/cidr.rs` : ARCH_A enum contained some values
 with the same number. `SAM3XXE` has been preserved while other conflicts
 have been set to dummy values. *(essential)*
 - `src/chipid/cidr.rs` : added `#[allow(unreachable_patterns)]` *(warnings)*
 - `src/lib.rs` : added `#![allow(unused_braces)]` *(warnings)*
 - `src/piob.rs` : added reset values to some writable registers, *(essential)*
 in order to write to them.

// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(phase, lang_items)]
#![no_std]

#[phase(plugin, link)]
extern crate core;
extern crate libc;

#[phase(plugin, link)]
extern crate collections;

use core::option::Option::Some;
use core::slice::SliceExt;
use collections::vec::Vec;

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

// Issue #16806

#[start]
fn start(_argc: int, _argv: *const *const u8) -> int {
    let x: Vec<u8> = vec![0, 1, 2];
    match x.last() {
        Some(&2) => (),
        _ => panic!(),
    }
    0
}

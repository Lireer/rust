// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate collections;
//~^ NOTE previous import of `collections` here

mod collections {
//~^ ERROR `collections` has already been imported in this module [E0260]
//~| NOTE `collections` already imported
    pub trait MyTrait {
        fn do_something();
    }
}

fn main() {}
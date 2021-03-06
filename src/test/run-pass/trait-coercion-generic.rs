// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unknown_features)]
#![feature(box_syntax)]

trait Trait<T> {
    fn f(&self, x: T);
}

#[derive(Copy)]
struct Struct {
    x: int,
    y: int,
}

impl Trait<&'static str> for Struct {
    fn f(&self, x: &'static str) {
        println!("Hi, {}!", x);
    }
}

pub fn main() {
    let a = Struct { x: 1, y: 2 };
    let b: Box<Trait<&'static str>> = box a;
    b.f("Mary");
    let c: &Trait<&'static str> = &a;
    c.f("Joe");
}


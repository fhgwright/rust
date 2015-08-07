// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Qiz {
  fn qiz();
}

struct Foo;
impl Qiz for Foo {
  fn qiz() {}
}

struct Bar {
//~^ ERROR E0038
  foos: &'static [&'static (Qiz + 'static)]
}

const FOO : Foo = Foo;
const BAR : Bar = Bar { foos: &[&FOO]};

fn main() { }

// This file is part of libfringe, a low-level green threading library.
// Copyright (c) whitequark <whitequark@whitequark.org>
// See the LICENSE file included in this distribution.
extern crate fringe;

use fringe::{Stack, OsStack};

#[test]
fn stack_accessible() {
  let stack = OsStack::new(4096).unwrap();
  // Make sure the topmost page of the stack, at least, is accessible.
  unsafe { *(stack.top().offset(-1)) = 0; }
}

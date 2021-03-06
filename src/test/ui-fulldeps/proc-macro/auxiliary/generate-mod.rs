// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
// no-prefer-dynamic

#![feature(proc_macro)]
#![crate_type = "proc-macro"]

extern crate proc_macro;
use proc_macro::*;

#[proc_macro]
pub fn check(_: TokenStream) -> TokenStream {
    "
    type Alias = FromOutside; // OK
    struct Outer;
    mod inner {
        type Alias = FromOutside; // `FromOutside` shouldn't be available from here
        type Inner = Outer; // `Outer` shouldn't be available from here
    }
    ".parse().unwrap()
}

#[proc_macro_attribute]
pub fn check_attr(_: TokenStream, _: TokenStream) -> TokenStream {
    "
    type AliasAttr = FromOutside; // OK
    struct OuterAttr;
    mod inner_attr {
        type Alias = FromOutside; // `FromOutside` shouldn't be available from here
        type Inner = OuterAttr; // `OuterAttr` shouldn't be available from here
    }
    ".parse().unwrap()
}

#[proc_macro_derive(CheckDerive)]
pub fn check_derive(_: TokenStream) -> TokenStream {
    "
    type AliasDerive = FromOutside; // OK
    struct OuterDerive;
    mod inner_derive {
        type Alias = FromOutside; // `FromOutside` shouldn't be available from here
        type Inner = OuterDerive; // `OuterDerive` shouldn't be available from here
    }
    ".parse().unwrap()
}

// ignore-tidy-linelength
// run-rustfix

#![feature(edition_macro_pats)]
#![deny(or_patterns_back_compat)]
#![allow(unused_macros)]
macro_rules! foo { ($x:pat | $y:pat) => {} } //~ ERROR the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
macro_rules! bar { ($($x:pat)+ | $($y:pat)+) => {} } //~ ERROR the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
macro_rules! baz { ($x:pat2015 | $y:pat2015) => {} } // should be ok
macro_rules! qux { ($x:pat2015 | $y:pat) => {} } // should be ok
macro_rules! ogg { ($x:pat | $y:pat2015) => {} } //~ ERROR the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
macro_rules! match_any {
    ( $expr:expr , $( $( $pat:pat )|+ => $expr_arm:expr ),+ ) => { //~ ERROR the meaning of the `pat` fragment specifier is changing in Rust 2021, which may affect this macro
        match $expr {
            $(
                $( $pat => $expr_arm, )+
            )+
        }
    };
}

fn main() {
    let result: Result<i64, i32> = Err(42);
    let int: i64 = match_any!(result, Ok(i) | Err(i) => i.into());
    assert_eq!(int, 42);
}

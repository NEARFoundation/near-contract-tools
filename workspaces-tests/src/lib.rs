#![allow(missing_docs)]
#![cfg(not(windows))]

#[macro_export]
macro_rules! predicate {
    () => {
        pub fn main() {}
    };
}

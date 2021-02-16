#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod knight;

#[skyline::main(name = "")]
pub fn main() {
    knight::install();
}
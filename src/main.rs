#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

mod quickcheck_extra;
// mod processor;
// mod decode;
// mod execute;

mod decoded;
mod decoder;
mod instruction;
mod error;

fn main() {
    println!("Hello, world!");
}

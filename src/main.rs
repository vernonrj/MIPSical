#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod quickcheck_extra;

mod decoded;
mod decoder;
mod instruction;
mod error;

fn main() {
    println!("Hello, world!");
}

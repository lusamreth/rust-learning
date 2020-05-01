#![allow(dead_code)]
mod ownership;
mod print;
mod structure;
mod interface;
mod lifetime;
use crate::structure::shop;
use std::collections::HashMap;
use std::io::{self};
fn main() {
    //Mod  || function
    // print::run();
    // ownership::testing();
    lifetime::runner();
    println!("Welcome to testing app!");
    // println!("Choose your option []");
    // structure::run();
    // interface::runner();
}

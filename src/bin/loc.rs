use std::env;
use std::fs::{self, File};

fn main() {
    for a in env::args() {
        build(&a);
    }
}

fn build(file: &str) {
    
}
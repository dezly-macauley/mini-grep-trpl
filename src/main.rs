// The first task is to make minigrep accept its two command line arguments:
// the file path and a string to search for.

// NOTE: How to run this program:

/*
    Example 1: Run without passing any arguments

    [src/main.rs:31:5] args = [
        "target/debug/mini-grep-trpl",
    ]

    This returned a Vector where the the the first string in the vector,
    where the String at index 0 is "target/debug/mini-grep-trpl" 

    Which is the directory where the binary executable is located

*/

/*

    Example 2: Run with arguements
    cargo run -- Europe Asia

    The two hypens indicate the following arguments
    are for our program rather than for cargo

    [src/main.rs:45:5] args = [
        "target/debug/mini-grep-trpl",
        "Europe",
        "Asia",
    ]

    This returned a Vector containing 3 items, starting from index 0
    to index 2

*/

// E.g. cargo run -- searchstring example_filename.txt

//=============================================================================

/*
    This lines tells Rust that you want to use the "env" module,
    which is part of "std". "std" is the standard library

    The env module provides functions for interacting with the environment,
    like reading command-line arguments or environment variables.
*/

use std::env;

fn main() {
    
    // env::arg() is a function that returns an iterator
    // over the command-line arguments passed to the program.
    // .collect() is a method that collects these arguments
    // into a data structure.
    // In this case that data structue is a String
    let args: Vec<String> = env::args().collect(); 

    // The dbg! macro is used to print the value of a variable 
    // (or an expression) for debugging purposes
    dbg!(args);

}

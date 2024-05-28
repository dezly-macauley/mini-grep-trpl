// NOTE: What is this program meant to do?

// The first task is to make minigrep accept its two command line arguments:
// the file path and a string to search for.

//=============================================================================

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

// NOTE: Potential errors that need to be handled:

// 1. The user could run the program without supplying any arguements,
// with only one arguement, or more than two arguements

// 2. The file the user wants to be searched, does not exist.

//=============================================================================

/*
    This lines tells Rust that you want to use the "env" module,
    which is part of "std". "std" is the standard library

    The env module provides functions for interacting with the environment,
    like reading command-line arguments or environment variables.
*/

use std::env;

// The "fs" module is used for filesystem operations such as reading or 
// writing from files
use std::fs;

fn main() {
    
    // NOTE: Step 1: Read all of the command-line arguments and save them
    // to a Vector

    // env::arg() is a function that returns an iterator
    // over the command-line arguments passed to the program.
    // .collect() is a method that collects these arguments
    // into a data structure.
    // In this case that data structue is a String
    let args: Vec<String> = env::args().collect(); 

    // The dbg! macro is used to print the value of a variable 
    // (or an expression) for debugging purposes
    // Remember to use &args so that the Debug macro does not take ownership
    // of the string.
    // dbg!(&args);

    // NOTE: Step 2: Create separate variables for each arguement
    // Two variables are needed at this point.

    // A variable to store the text that the user wants to search for,
    // and a second variable to store the name of the 
    // file that will be searched.

    // Remember that if you run this command: Cargo run -- pizza food_list.txt
    // This will create a String with 3 items.
    // index 0: "target/debug/mini-grep-trpl"
    // index 1: "pizza"
    // index 2: "food_list.txt"

    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("Searching for {}", query);
    println!("In the file {}", file_path);

    // NOTE: Step 3: Create a variable that will read the contents of the file

    // If the file does not exist the program will panic, crash, 
    // and display and error message.
    let contents: String = fs::read_to_string(file_path)
        .expect("The file you want to search either has an error,
        or does not exist");

    println!("With text:\n{contents}");

}

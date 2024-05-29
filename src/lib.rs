
// This function accepts the arguement:
// let args: Vec<String> = env::args().collect(); 
// This is what reads all of the command-line arguements and saves them into
// a Vector of Strings

/*
    After that, this function will go through that Vec<String>, and take
    the 2nd String (at index 1), and the 3rd String (at index 0) and
    place them into seperate variables.
*/

// let args: Vec<String> = env::args().collect(); 
// This function extracts the search_query and the file_path from args,
// puts them into a tuple and then returns this tuple

// Even though the variable type of args is Vec<String>, 
// you would use &[String] instead of &Vec<String> because this make 
// the function more flexible
// You can pass either the entire vector or just a part of it.


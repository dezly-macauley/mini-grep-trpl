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

pub fn parse_config(args: &[String]) -> (&str, &str) {

    let search_query: &String = &args[1];
    let file_path: &String = &args[2];
   
    // This function then creates a tuple that contains string slices
    // When creating a function in Rust that would return &String 
    // and work fine, it is actually better to have this function return &str,
    // using the .as_str conversion.
    // This is because &str is more flexible and lightweight than &String
    let search_details: (&str, &str) = (search_query.as_str(), file_path.as_str());

    // In Rust the last line of a function that does not have a ; 
    // is implicitly returned. If you wanted to explicitly return the do this:
    // return search_details;
    search_details

}

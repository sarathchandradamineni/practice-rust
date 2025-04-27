fn main() {

    // Creating a new string using the function "to_string"
    let mut s = String::new();
    let data = "Sarath Chandra Damineni";
    s = data.to_string();
    println!("string is {s}");

    // Creating a new string  with String::from
    let mut s2 = String::from("initial contents");
    println!("string s2 is {s2}");

    // Updating the strings with push_str method
    let mut s3 = String::from("Sarath");
    let s4 = "chandra";
    s3.push_str(s4);
    println!("s3 is {s3}");

    // Updating strings with '+' -> This uses 'add' method in the background
    let s4 = String::from("Sarath");
    let s5 = String::from("Chandra");
    let s6 = s4 + &s5;
    println!("String s6 is {s6}");

    // Indexing a string in rust is not simple 
    // Rust handles the string in UTF-8 format so each character in a string can be more than 1 byte
    for c in "नमस्ते".chars()
    {
        println!("{c}");
    }

    for c in "नमस्ते".bytes()
    {
        println!("{c}");
    }
    }

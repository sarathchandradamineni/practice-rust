fn main() {
    // Vector is a collection data type
    // Allows to store multiple value of same data structure
    // Puts all the values next to each other in the memory

    // Declaring a new vector
    let v: Vec<i32> = Vec::new();

    // Default initialization of vector using vec! macro
    let v2 = vec![1, 2, 3];

    // Pushing elements in the vector
    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(27);

    // Reading the elements in the vector
    // Reading can be done in two ways 
    // 1. Directly from the index
    // 2. Using the get method with the help of match, Some, None [This is a better handling way]
    let v4 = vec![1, 2, 3, 4];
    let third: &i32 = &v4[2];
    // This way is error prone and can lead to the error-> Index out of the bound problem
    println!("Element in the third index is {third}");

    let thirdOption: Option<&i32> = v4.get(21);
    // Second way to read the elements in the vector
    match thirdOption{
        Some(thirdOption) => println!("The third element is {thirdOption}"),
        None => println!("There is no third element"),
    }

}

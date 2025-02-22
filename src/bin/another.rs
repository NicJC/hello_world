fn main() {
    // Create a new empty vector
    let mut list: Vec<i32> = Vec::new();

    // Add elements to the vector
    list.push(1);
    list.push(2);
    list.push(3);

    // Print the elements of the vector using a for loop
    for element in &list {
        println!("{}", element);
    }
    
}

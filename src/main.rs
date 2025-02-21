fn main() {
    //  empty vector
    let mut list: Vec<i32> = Vec::new();

    // elements to vector
    list.push(1);
    list.push(2);
    list.push(3);

    // print
    for element in &list {
        println!("{}", element);
    }
}

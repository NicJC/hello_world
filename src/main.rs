mod vectors; 
mod math;

fn main() {
    let mut numbers = vec![12, 2, 13, 4, 35, 18];

    // Using functions from the nested modules
    vectors::operations::add_element(&mut numbers, 6);
    println!("Some vector after adding: {:?}", numbers);

    let sum = vectors::operations::sum_elements(&numbers);
    println!("Sum of numbers: {}", sum);

    let reversed = vectors::utils::reverse_vector(&numbers);
    println!("Reversed Vector: {:?}", reversed);

    let sum = math::add(10, 20);
    println!("Sum: {}", sum);

    let diff = math::subtract(56, 15);
    println!("The difference is: {}", diff);
}
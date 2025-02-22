pub fn add_element(vec: &mut Vec<i32>, value: i32) {
    vec.push(value);
}

pub fn sum_elements(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

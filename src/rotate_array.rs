fn rotate(nums: &mut Vec<i32>, k: i32) {
    let mut key = k;
    while key > 0 {
        nums.rotate_right(1);
        key = key - 1;
    }
}

pub fn rotate_array_test() {
    let mut input: Vec<i32> = Vec::from([1, 2, 3, 4, 5, 6, 7]);
    let k_value: i32 = 3;
    let expected: Vec<i32> = Vec::from([5, 6, 7, 1, 2, 3, 4]);
    rotate(&mut input, k_value);
    println!("expected: {:?}\nactual: {:?}", expected, input);

    let mut input_2: Vec<i32> = Vec::from([-1]);
    let k_value_2: i32 = 2;
    let expected_2: Vec<i32> = Vec::from([-1]);
    rotate(&mut input_2, k_value_2);
    println!("expected: {:?}\nactual: {:?}", expected_2, input_2);

    let mut input_3: Vec<i32> = Vec::from([1, 2]);
    let k_value_3: i32 = 3;
    let expected_3: Vec<i32> = Vec::from([2, 1]);
    rotate(&mut input_3, k_value_3);
    println!("expected: {:?}\nactual: {:?}", expected_3, input_3);
}

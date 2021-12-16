// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
// Note that you must do this in-place without making a copy of the array.
// Example 1:
// Input: nums = [0,1,0,3,12]
// Output: [1,3,12,0,0]
// Example 2:
// Input: nums = [0]
// Output: [0]
// Constraints:
// 1 <= nums.length <= 104
// -231 <= nums[i] <= 231 - 1
// Follow up: Could you minimize the total number of operations done?

fn move_zeroes(nums: &mut Vec<i32>) {
    // keep a count of the zeroes encountered
    // when hit a non-zero number move that value the count of the zero, spaces left.
    // put the zero in the place of the current value
    let mut zero_count: usize = 0; // 1
    for i in 0..nums.len() {
        if nums[i] == 0 {
            zero_count = zero_count + 1;
        } else {
            nums[i - zero_count] = nums[i];
            if zero_count != 0 {
              nums[i] = 0;
            }
        }
    }
}

pub fn move_zeroes_test() {
    let mut nums: Vec<i32> = Vec::from([0, 1, 0, 3, 12]);
    let expected: Vec<i32> = Vec::from([1, 3, 12, 0, 0]);
    move_zeroes(&mut nums);
    println!("expected: {:?}\nactual: {:?}\n==============", expected, nums);

    let mut nums_2: Vec<i32> = Vec::from([0]);
    let expected_2: Vec<i32> = Vec::from([0]);
    move_zeroes(&mut nums_2);
    println!("expected: {:?}\nactual: {:?}\n==============", expected_2, nums_2);

    let mut nums_3: Vec<i32> = Vec::from([1]);
    let expected_3: Vec<i32> = Vec::from([1]);
    move_zeroes(&mut nums_3);
    println!("expected: {:?}\nactual: {:?}\n==============", expected_3, nums_3);
}

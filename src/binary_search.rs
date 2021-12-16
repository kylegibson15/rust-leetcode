fn search(nums: &Vec<i32>, target: i32) -> i32 {
  let mut answer: i32 = -1;
  let mut high_ptr: usize = nums.len();
  let mut low_ptr: usize = 0;
  loop {
      if target > nums[nums.len()-1] || target < nums[0] {
          break;
      }
      let mid_ptr = ((high_ptr - low_ptr) / 2) + low_ptr;
      let value = nums[mid_ptr];
      if (high_ptr - low_ptr) == 1 && target > value {
          break;
      }
      if value == target {
          answer = mid_ptr as i32;
          break;
      } else if value < target {
          low_ptr = mid_ptr;
      } else {
          high_ptr = mid_ptr;
      }
  }
  answer
}

pub fn binary_search_test() {
    println!("Running Binary Search Tests");
  let arr: Vec<i32> = Vec::from([-1,0,3,5,9,12]);
  println!("input array: {:?}", arr);
  println!("{} should be found: {}", -1, search(&arr, -1));
  println!("{} should be found: {}", 0, search(&arr, 0));
  println!("{} should be found: {}", 3, search(&arr, 3));
  println!("{} should be found: {}", 5, search(&arr, 5));
  println!("{} should be found: {}", 9, search(&arr, 9));
  println!("{} should be found: {}", 12, search(&arr, 12));
  println!("========");
  println!("{} should be NOT found: {}", -2, search(&arr, -2));
  println!("{} should be NOT found: {}", 1, search(&arr, 1));
  println!("{} should be NOT found: {}", 2, search(&arr, 2));
  println!("{} should be NOT found: {}", 6, search(&arr, 6));
  println!("{} should be NOT found: {}", 11, search(&arr, 11));
  println!("{} should be NOT found: {}", 15, search(&arr, 15));
  println!("Finished Binary Search Tests\n===========================\n");
}
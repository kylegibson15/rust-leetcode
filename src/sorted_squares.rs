fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
  // O(n) solution
  // array is non-decreasing with negatives
  // the negative values are the ones we need to worry about inserting
  // while iterating store the current negative value in a new array
  // while iterating the positive values check and see if the negative should be put into the new arr
  let mut negatives: Vec<i32> = Vec::new();
  let mut answer: Vec<i32> = Vec::new();
  
  for value in &nums {
      if value < &0 {
          let sq_value: i32 = value.pow(2);
          negatives.push(sq_value);
      } else {
          let sq_value: i32 = value.pow(2);
          while negatives.len() > 0 {
              let last_neg_value: i32 = negatives[negatives.len()-1];

              if last_neg_value <= sq_value {
                  answer.push(negatives.pop().unwrap());
              } else {
                  break;
              }
          }
          answer.push(sq_value);
      }
  }  

  if negatives.len() > 0 {
      for value in negatives.iter().rev() {
          answer.push(*value);
      }
  }
  answer
}

pub fn sorted_squares_test() {
  println!("Running Sorted Squares Tests");
  let nums: Vec<i32> = Vec::from([-4,-1,0,3,10]);
  let expected: Vec<i32> = Vec::from([0,1,9,16,100]);
  let actual = sorted_squares(nums);
  println!("expected: {:?}, actual: {:?}", expected, actual);
  let nums_2: Vec<i32> = Vec::from([-1]);
  let expected_2: Vec<i32> = Vec::from([1]);
  let actual_2 = sorted_squares(nums_2);
  println!("expected: {:?}, actual: {:?}", expected_2, actual_2);
  let nums_3: Vec<i32> = Vec::from([-5,-3,-2,-1]);
  let expected_3: Vec<i32> = Vec::from([1,4,9,25]);
  let actual_3 = sorted_squares(nums_3);
  println!("expected: {:?}, actual: {:?}", expected_3, actual_3);
  let nums_4: Vec<i32> = Vec::from([-10000,-9999,-7,-5,0,0,10000]);
  let expected_4: Vec<i32> = Vec::from([0,0,25,49,99980001,100000000,100000000]);
  let actual_4 = sorted_squares(nums_4);
  println!("expected: {:?}, actual: {:?}", expected_4, actual_4);
  println!("Finished Sorted Squares Tests\n===========================\n");
}

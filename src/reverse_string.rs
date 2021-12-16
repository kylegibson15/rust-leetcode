fn reverse_string(s: &mut Vec<char>) {
  s.reverse()
}

pub fn reverse_string_test() {
  let mut chars: Vec<char> = Vec::from(['h','e','l','l','o']);
  reverse_string(&mut chars);
  println!("end result: {:?}", chars);
}
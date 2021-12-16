// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

}

fn list_length(list: &Option<Box<ListNode>>) -> u32 {
  let mut length: u32 = 0;
  let mut current: &Option<Box<ListNode>> = list;
  loop {
    println!("{:?}", current);
    current = current.unwrap()
  }
  length
}

pub fn middle_of_linked_list_test() {}

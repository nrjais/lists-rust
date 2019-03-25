use std::mem;

use crate::first::Link::{Empty, More};

#[allow(dead_code)]
pub struct List {
  head: Link,
}

enum Link {
  Empty,
  More(Box<Node>),
}

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    List { head: Link::Empty }
  }

  pub fn push(&mut self, e: i32) {
    let node = Node {
      elem: e,
      next: mem::replace(&mut self.head, Link::Empty),
    };

    self.head = Link::More(Box::new(node));
  }

  pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Link::Empty) {
      Empty => Option::None,
      More(node) => {
        self.head = node.next;
        Option::Some(node.elem)
      }
    }
  }
}

impl Drop for List {
  fn drop(&mut self) {
    let mut cur_link = mem::replace(&mut self.head, Link::Empty);

    while let Link::More(mut boxed_node) = cur_link {
      cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
    }
  }
}


#[cfg(test)]
mod tests {
  use crate::first::List;

  #[test]
  fn should_add_given_element_to_list() {
    let mut list = List::new();
    list.push(5);

    assert_eq!(list.pop(), Some(5));
  }

  #[test]
  fn should_push_given_element_to_list() {
    let mut list = List::new();
    list.push(5);
    list.push(3);
    list.push(2);

    assert_eq!(list.pop(), Some(2));
  }

  #[test]
  fn basics() {
    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }
}



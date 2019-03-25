use std::mem;

#[allow(dead_code)]
pub struct List {
  head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
      elem,
      next: self.head.take(),
    });

    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<i32> {
    self.head.take().map(|n| {
      self.head = n.next;
      n.elem
    })
  }
}

impl Drop for List {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();
    while let Some(mut boxed_node) = cur_link {
      cur_link = boxed_node.next.take();
    }
  }
}


#[cfg(test)]
mod tests {
  use crate::second::List;

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



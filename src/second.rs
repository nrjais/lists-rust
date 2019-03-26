use std::mem;

#[allow(dead_code)]
pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(Node {
      elem,
      next: self.head.take(),
    });

    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|n| {
      self.head = n.next;
      n.elem
    })
  }

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| {
      &node.elem
    })
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|n| {
      &mut n.elem
    })
  }
}

impl<T> Drop for List<T> {
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

  #[test]
  fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
    list.peek_mut().map(|value| {
      *value = 42
    });

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
  }
}



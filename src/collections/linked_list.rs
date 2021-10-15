#[derive(Debug)]
struct Node<T> {
	value: T,
	next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
	fn new(value: T) -> Self {
		Node {
			value: value,
			next: None,
		}
	}
}

#[derive(Debug)]
pub struct LinkedList<T> {
	head: Option<Box<Node<T>>>,
	len: usize
}

impl<T> LinkedList<T> {
	pub fn new() -> Self {
		return LinkedList {
			head: None,
			len: 0,
		}
	}

	pub fn push(&mut self, value: T) {
		let mut new_node = Box::new(Node::new(value));
		match self.head.take() {
			Some(old_head) => {
				new_node.next = Some(old_head);
				self.head = Some(new_node);
			},
			None => {
				self.head = Some(new_node);
			}
		}
		self.len += 1;
	}
}

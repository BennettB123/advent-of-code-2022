#[derive(Debug)]
pub struct Stack<T> {
	data: Vec<T>
}

impl<T> Stack<T> {
	pub fn new() -> Stack<T> {
		Stack {
			data: Vec::new(),
		}
	}

	pub fn push(&mut self, x: T) {
		self.data.push(x);
	}

	pub fn pop(&mut self) -> Option<T> {
		self.data.pop()
	}

	pub fn peek(&self) -> Option<&T> {
		self.data.last()
	}
}
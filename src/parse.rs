use crate::{
	parse::Type::*,
	tokenise::token::Token::{self, *},
};
#[derive(Debug, Clone)]
enum Type {
	Tree,
	Group,
	// Root,
	// LShift,
	// RShift,
	Add,
	// Sub,
	// Div,
	// Mul,
	// Exp,
	// Mod,
	Num(f64),
}
#[derive(Debug, Clone)]
pub struct Node(Type, Vec<Node>);
impl Node {
	fn new(r#type: Type) -> Self {
		Self(r#type, vec![])
	}
	fn push(&mut self, node: Self) {
		self.1.push(node)
	}
}
pub(super) fn parse(tokens: Vec<Token>) -> Result<Node, ()> {
	let mut stack: Vec<Node> = vec![Node::new(Tree)];
	for t in tokens {
		match t {
			Number(n) => {
				if let Some(last) = stack.last_mut() {
					last.push(Node::new(Num(n)))
				}
			}
			Sign(s) => {
				let _node;
				let s = s.as_str();
				if ["+", "-", "*", "/", "%", "^", "//", "<<", ">>"].contains(&s)
				{
					match s {
						"+" => _node = Node::new(Add),
						// "-" => _node = Node::new(Sub),
						// "*" => _node = Node::new(Mul),
						// "/" => _node = Node::new(Div),
						// "%" => _node = Node::new(Mod),
						// "^" => _node = Node::new(Exp),
						// "//" => _node = Node::new(Root),
						// "<<" => _node = Node::new(LShift),
						// ">>" => _node = Node::new(RShift),
						_ => unreachable!(),
					}
				} else {
					panic!("'{s}' is an invalid sign!")
				}
				let Node(t, c) = stack.last().unwrap();
				println!("{t:?}, {c:?}")
			}
			Open => stack.push(Node::new(Group)),
			Close => {
				if let Some(node) = stack.pop() {
					if let Some(last) = stack.last_mut() {
						last.push(node)
					}
				}
			}
		}
	}
	// println!("Stack: {stack:#?}");
	assert!(
		!stack.is_empty(),
		"The stack is too small! ({len})",
		len = stack.len()
	);
	let tree = stack.last().unwrap();
	println!("Tree: {tree:#?}");
	assert!(stack.len() == 1);
	Ok(tree.clone())
}

/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/


#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		if self.size > 0 {
            self.size -= 1;
            let res_ref = self.data.remove(self.size);
            Some(res_ref)
        } else {
            None
        }
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool
{
	const LEFT_BRACH: char = '{';
    const RIGHT_BRACH: char = '}';
    const LEFT_SQBRACH: char = '[';
    const RIGHT_SQBRACH: char = ']';
    const LEFT_SMBRACH: char = '(';
    const RIGHT_SMBRACH: char = ')';

    const PUSH_STACK_CHARS: [char; 3] = [LEFT_BRACH, LEFT_SMBRACH, LEFT_SQBRACH];
    const POP_STACK_CHARS: [char; 3] = [RIGHT_BRACH, RIGHT_SMBRACH, RIGHT_SQBRACH];
    let mut bracket_stack = Stack::new();
    let mut is_match = true;
    for c in bracket.chars() {
        if PUSH_STACK_CHARS.contains(&c) {
            bracket_stack.push(c);
        } else if POP_STACK_CHARS.contains(&c) {
            let bracket_peek = bracket_stack.peek();
            match bracket_peek {
                Some(peek_char) => match peek_char {
                    &LEFT_BRACH => {
                        if RIGHT_BRACH.eq(&c) {
                           let _= bracket_stack.pop();
                            continue;
                        } else {
                            is_match = false;
                            break;
                        }
                    }
                    &LEFT_SMBRACH => {
                        if RIGHT_SMBRACH.eq(&c) {
                            let _= bracket_stack.pop();
                            continue;
                        } else {
                            is_match = false;
                            break;
                        }
                    }
                    &LEFT_SQBRACH => {
                        if RIGHT_SQBRACH.eq(&c) {
                            let _= bracket_stack.pop();
                            continue;
                        } else {
                            is_match = false;
                            break;
                        }
                    }
                    _ => {}
                },
                None => {
                    is_match = false;
                    break;
                }
            }
        }
    }
    is_match && bracket_stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}
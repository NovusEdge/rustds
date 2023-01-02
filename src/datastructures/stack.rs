#[allow(dead_code)]
#[derive(Clone)]
pub struct Stack<T> {
    length: usize,
    elements: Vec<T>,
}

#[derive(Debug)]
pub enum StackError {
    StackOverflow,
    StackUnderflow,
    MemoryError,
}

/// stack::new returns a stack of a fixed size: <length> from the provided
/// elements. The elements vector is truncated to fit the length parameter of
/// the stack.
pub fn new<T: Clone>(length: usize, elements: Vec<T>) -> Stack<T> {
    let mut temp = elements.to_vec();
    temp.truncate(length);

    Stack {
        length,
        elements: temp,
    }
}

/// Implementation for default methods for the stack data structure.
impl<T> Stack<T> {
    pub fn push(&mut self, v: T) -> Result<(), StackError> {
        match self.length >= self.elements.len() {
            true => Err(StackError::StackOverflow),
            false => { 
                self.length += 1;
                Ok(self.elements.push(v))
            },
        }
    }
    
    pub fn pop(&mut self) -> Result<Option<T>, StackError> {
        match self.length == 0 {
            true => Err(StackError::StackUnderflow),
            false => { 
                self.length -= 1;
                Ok(self.elements.pop())
            },
        }
    }

    pub fn size(&self) -> usize { self.length }
}

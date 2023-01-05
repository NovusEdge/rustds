
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Queue<T> {
    pub len: usize,
    elements: Vec<T>
}

impl<T: Copy> Queue<T> {
    pub fn new() -> Self {
        Queue { 
            len: 0,
            elements: Vec::new(), 
        }
    }

    pub fn enqueue(&mut self, element: T) {
        self.len += 1;
        self.elements.push(element);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.len -= 1;
        self.elements.pop()
    }
}

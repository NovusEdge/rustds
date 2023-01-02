use std::rc::Rc;

/// TODO: write some documentation for this...
type SingleLink<T> = Option<Rc<SinglyLinkedList<T>>>;
type DoubleLink<T> = Option<Rc<DoublyLinkedList<T>>>;


/// TODO: Write some documentation for this...
#[allow(dead_code)]
#[derive(Clone)]
pub struct SinglyLinkedList<T> {
    pub element: T,
    next: SingleLink<T>,
}


/// TODO: Write some documentation for this...
#[allow(dead_code)]
#[derive(Clone)]
pub struct DoublyLinkedList<T> {
    pub element: T,
    next: DoubleLink<T>,
    prev: DoubleLink<T>,
}

impl<T: Copy> SinglyLinkedList<T> {
    pub fn new(initial_element: T) -> Self {
        SinglyLinkedList { element: initial_element, next: None }
    }

    pub fn prepend(&self, element: T) -> SinglyLinkedList<T> {
        SinglyLinkedList {
            element,
            next: Some(Rc::new(SinglyLinkedList { 
                element: self.element, 
                next: self.next.clone(),
            })),
        }
    }

    pub fn tail(&self) -> SinglyLinkedList<T> {
        SinglyLinkedList {
            element: self.element,
            next: self.next.as_ref().and_then(|node| node.next.clone()) 
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.next.as_ref().map(|node| &node.element)
    }

}

impl<T> DoublyLinkedList<T> {

}

#[allow(unused_variables)]
impl<T> std::fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!();
    }
}

#[allow(unused_variables)]
impl<T> std::fmt::Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!();
    }
}


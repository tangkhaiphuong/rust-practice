use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

struct Node<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
    element: T,
}

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

pub struct Iter<'a, T: 'a> {
    curr: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            element: value,
            next: None,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.len += 1;
        let next = Rc::new(RefCell::new(Node::new(value)));

        if let None = self.head {
            self.head = Some(next.clone());
        }

        match &self.tail {
            None => self.tail = Some(next.clone()),
            Some(ref tail) => {
                tail.borrow_mut().next = Some(next.clone());
                self.tail = Some(next.clone());
            }
        }

        return;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            curr: self.head.clone(),
            len: self.len,
            marker: PhantomData,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.curr {
            None => None,
            Some(curr) => {
                let result = curr.as_ptr();

                let next = curr.borrow().next.clone();

                self.curr = next;

                self.len -= 1;

                unsafe { Some(&(*result).element) }
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

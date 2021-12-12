use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut h = &self.head;
        while let Some(n) = h {
            count += 1;
            h = &n.next;
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(n) => {
                self.head = n.next;
                Some(n.data)
            },
            None => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(n) => Some(&n.data),
            None => None
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        let mut h = self.head;
        while let Some(n) = h {
            rev_list.push(n.data);
            h = n.next;
        }
        rev_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut res = SimpleLinkedList::new();
        for i in _iter {
            res.push(i);
        }
        res
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut res = Vec::new();
        let mut h = self.head;
        while let Some(n) = h {
            res.push(n.data);
            h = n.next;
        }
        res.reverse();
        res
    }
}

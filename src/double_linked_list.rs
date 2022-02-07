use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct DbNode<T> {
    data: T,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
    next: Option<Rc<RefCell<DbNode<T>>>>,
}

struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbList<T> {
    fn new() -> Self {
        DbList {
            last: None,
            first: None,
        }
    }

    fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }

    fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: Some(r.clone()),
                    next: None,
                }));
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(new_back));
                m.next = Some(new_back);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DbList;
    #[test]
    fn test0() {
        let mut dl = DbList::new();
        dl.push_front(6);
        dl.push_back(11);
        dl.push_front(5);
        dl.push_back(4);
    }
}

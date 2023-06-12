use std::cell::RefCell;
use std::fmt::*;
use std::rc::Rc;

/*

Basic i32 linked-list implementation

1. Option<T> allows to specify null-values (like nullptr)
2. Rc<T> allows multiple ownership of the object (ie. multiple vars referring to same object)
3. RefCell<T> allows for interior mutability. Rc<X> itself doesn't allow mutation of X

*/
#[allow(dead_code)]
struct Int32LinkedListNode {
    val: i32,
    next: Option<Rc<RefCell<Int32LinkedListNode>>>,
}

pub struct Int32LinkedList {
    len: usize,
    head: Option<Rc<RefCell<Int32LinkedListNode>>>,
    tail: Option<Rc<RefCell<Int32LinkedListNode>>>,
}

#[allow(dead_code)]
impl Int32LinkedList {
    pub fn new() -> Int32LinkedList {
        Int32LinkedList {
            len: 0 as usize,
            head: None,
            tail: None,
        }
    }

    pub fn empty(&self) -> bool {
        self.len == 0
    }

    pub fn add_to_back(&mut self, to_add: i32) {
        // Make the new node and increase size
        let new_node_rc = Rc::new(RefCell::new(Int32LinkedListNode {
            val: to_add,
            next: None,
        }));

        if self.empty() {
            let new_node_rc_tail = new_node_rc.clone();
            self.head = Some(new_node_rc);
            self.tail = Some(new_node_rc_tail);
            self.len += 1;
            return;
        }

        let tail_node = self.tail.take().unwrap();
        tail_node.borrow_mut().next = Some(new_node_rc.clone());
        self.tail = Some(new_node_rc);
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn size(&self) -> usize {
        self.len()
    }

    pub fn get_head(&self) -> Option<i32> {
        let ret = match &self.head {
            None => None,
            Some(v_cell_ptr) => Some(v_cell_ptr.borrow().val),
        };
        ret
    }

    pub fn get_tail(&self) -> Option<i32> {
        let ret = match &self.tail {
            None => None,
            Some(v_cell_ptr) => Some(v_cell_ptr.borrow().val),
        };
        ret
    }
}

use std::fmt;

impl std::fmt::Display for Int32LinkedList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.empty() {
            write!(f, "<empty>")?;
            return Ok(());
        }
        let mut ptr = self.head.clone();
        let mut cnt = 0;
        loop {
            match ptr {
                None => break,
                Some(v) => {
                    cnt += 1;
                    write!(
                        f,
                        "{}",
                        v.borrow().val
                    )?;
                    if cnt != self.size() {
                        write!(f, ", ")?;
                    }
                    ptr = v.borrow().next.clone();
                }
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! make_i32_ll {
    ( $( $x:expr ),* ) => {
        {
            let mut ret = Int32LinkedList::new();
            $(
                ret.add_to_back($x);
            )*
            ret
        }
    };
}

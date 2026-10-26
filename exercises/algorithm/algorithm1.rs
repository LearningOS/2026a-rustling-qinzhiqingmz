/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        if index < 0 {
            return None;
        }
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn merge(mut list_a: Self, mut list_b: Self) -> Self
    where
        T: Ord,
    {
        let length = list_a.length.checked_add(list_b.length).expect("List length overflow");
        let mut merged = Self::new();
        while let (Some(a), Some(b)) = (list_a.start, list_b.start) {
            // SAFETY: Both heads point to live nodes owned by their input lists.
            let take_a = unsafe { a.as_ref().val <= b.as_ref().val };
            let source = if take_a { &mut list_a } else { &mut list_b };
            let mut node = source.start.take().unwrap();
            // SAFETY: Detach the uniquely owned source head before appending it.
            unsafe {
                source.start = node.as_ref().next;
                node.as_mut().next = None;
            }
            source.length -= 1;
            if source.start.is_none() {
                source.end = None;
            }
            if let Some(mut tail) = merged.end {
                // SAFETY: merged owns its tail, and node is detached from source.
                unsafe { tail.as_mut().next = Some(node) };
            } else {
                merged.start = Some(node);
            }
            merged.end = Some(node);
        }
        let rest = if list_a.start.is_some() { &mut list_a } else { &mut list_b };
        if let Some(mut tail) = merged.end {
            // SAFETY: Transfer the remaining owned chain into the merged list.
            unsafe { tail.as_mut().next = rest.start.take() };
        } else {
            merged.start = rest.start.take();
        }
        if rest.end.is_some() {
            merged.end = rest.end.take();
        }
        rest.length = 0;
        merged.length = length;
        merged
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.start.take() {
            // SAFETY: Each node came from Box::into_raw and is reclaimed once.
            let owned = unsafe { Box::from_raw(node.as_ptr()) };
            self.start = owned.next;
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}
#[cfg(test)]
mod edge_tests {
    use super::*;
    #[test]
    fn empty_duplicate_and_owned_values() {
        let mut a = LinkedList::new();
        a.add(String::from("a"));
        a.add(String::from("a"));
        let mut b = LinkedList::new();
        b.add(String::from("a"));
        b.add(String::from("z"));
        let mut merged = LinkedList::merge(a, b);
        assert_eq!(merged.length, 4);
        for (i, expected) in ["a", "a", "a", "z"].iter().enumerate() {
            assert_eq!(merged.get(i as i32).map(String::as_str), Some(*expected));
        }
        assert_eq!(merged.get(-1), None);
        assert_eq!(merged.get(4), None);
        merged.add(String::from("zz"));
        assert_eq!(merged.get(4).map(String::as_str), Some("zz"));
        let mut empty = LinkedList::<String>::merge(LinkedList::new(), LinkedList::new());
        assert_eq!(empty.length, 0);
        assert_eq!(empty.get(0), None);
        let mut one = LinkedList::merge(LinkedList::new(), merged);
        assert_eq!(one.length, 5);
        assert_eq!(one.get(0).map(String::as_str), Some("a"));
    }
}

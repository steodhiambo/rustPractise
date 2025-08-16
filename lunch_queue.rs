#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        let new_person = Box::new(Person {
            name,
            discount,
            next_person: self.node.take(),
        });
        self.node = Some(new_person);
    }

    pub fn invert_queue(&mut self) {
        let mut prev = None;
        let mut current = self.node.take();

        while let Some(mut node) = current {
            let next = node.next_person.take();
            node.next_person = prev;
            prev = Some(node);
            current = next;
        }

        self.node = prev;
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        // Find the last person in the queue (FIFO - first in, first out)
        if self.node.is_none() {
            return None;
        }

        // If there's only one person
        if self.node.as_ref().unwrap().next_person.is_none() {
            let person = self.node.take().unwrap();
            return Some((person.name, person.discount));
        }

        // Find the second-to-last person
        let mut current = self.node.as_mut().unwrap();
        while current.next_person.as_ref().unwrap().next_person.is_some() {
            current = current.next_person.as_mut().unwrap();
        }

        // Remove the last person
        let last_person = current.next_person.take().unwrap();
        Some((last_person.name, last_person.discount))
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = &self.node;

        while let Some(person) = current {
            if person.name == name {
                return Some((person.name.clone(), person.discount));
            }
            current = &person.next_person;
        }

        None
    }
}

fn main() {
    let mut list = Queue::new();
    list.add(String::from("Marie"), 20);
    list.add(String::from("Monica"), 15);
    list.add(String::from("Ana"), 5);
    list.add(String::from("Alice"), 35);
    println!("{:?}", list);

    println!("{:?}", list.search("Marie"));
    println!("{:?}", list.search("Alice"));
    println!("{:?}", list.search("someone"));

    println!("removed {:?}", list.rm());
    println!("list {:?}", list);
    list.invert_queue();
    println!("invert {:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_queue() {
        let queue = Queue::new();
        assert!(queue.node.is_none());
    }

    #[test]
    fn test_add_single_person() {
        let mut queue = Queue::new();
        queue.add("Alice".to_string(), 10);
        
        assert!(queue.node.is_some());
        assert_eq!(queue.node.as_ref().unwrap().name, "Alice");
        assert_eq!(queue.node.as_ref().unwrap().discount, 10);
    }

    #[test]
    fn test_add_multiple_people() {
        let mut queue = Queue::new();
        queue.add("Alice".to_string(), 10);
        queue.add("Bob".to_string(), 20);
        
        // Bob should be at the front (most recently added)
        assert_eq!(queue.node.as_ref().unwrap().name, "Bob");
        assert_eq!(queue.node.as_ref().unwrap().next_person.as_ref().unwrap().name, "Alice");
    }

    #[test]
    fn test_search_existing_person() {
        let mut queue = Queue::new();
        queue.add("Alice".to_string(), 10);
        queue.add("Bob".to_string(), 20);
        
        assert_eq!(queue.search("Alice"), Some(("Alice".to_string(), 10)));
        assert_eq!(queue.search("Bob"), Some(("Bob".to_string(), 20)));
    }

    #[test]
    fn test_search_non_existing_person() {
        let mut queue = Queue::new();
        queue.add("Alice".to_string(), 10);
        
        assert_eq!(queue.search("Charlie"), None);
    }

    #[test]
    fn test_rm_single_person() {
        let mut queue = Queue::new();
        queue.add("Alice".to_string(), 10);
        
        let removed = queue.rm();
        assert_eq!(removed, Some(("Alice".to_string(), 10)));
        assert!(queue.node.is_none());
    }

    #[test]
    fn test_rm_fifo_order() {
        let mut queue = Queue::new();
        queue.add("First".to_string(), 10);
        queue.add("Second".to_string(), 20);
        queue.add("Third".to_string(), 30);
        
        // Should remove "First" (first in, first out)
        assert_eq!(queue.rm(), Some(("First".to_string(), 10)));
        assert_eq!(queue.rm(), Some(("Second".to_string(), 20)));
        assert_eq!(queue.rm(), Some(("Third".to_string(), 30)));
        assert_eq!(queue.rm(), None);
    }

    #[test]
    fn test_invert_queue() {
        let mut queue = Queue::new();
        queue.add("First".to_string(), 10);
        queue.add("Second".to_string(), 20);
        queue.add("Third".to_string(), 30);
        
        queue.invert_queue();
        
        // After inversion, "First" should be at the front
        assert_eq!(queue.node.as_ref().unwrap().name, "First");
        assert_eq!(queue.node.as_ref().unwrap().next_person.as_ref().unwrap().name, "Second");
    }
}

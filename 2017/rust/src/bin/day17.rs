type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    next: Link,
}

struct List {
    head: Link,
    current: usize,
    len: usize,
}

impl List {
    fn skip(&mut self, mut count: usize) {
        count %= self.len;
        self.current = (self.current + count) % self.len;
    }

    fn insert(&mut self, val: i32) {
        let mut current = &mut self.head;
        for _ in 0..self.current {
            current = &mut current.as_mut().unwrap().next;
        }

        println!("Inserting {} after {}", val, current.as_ref().unwrap().val);

        let new_node = Box::new(Node {
            val,
            next: None,
        });

        let node = current.as_mut().unwrap();
        node.next = Some(new_node);
        // new_node.next = current;
        // *current = Some(new_node);

        self.current += 1;
    }
}

fn print_list(list: &List) {
    let mut current: &Link = &list.head;
    while let Some(node) = current {
        print!("{},", node.val);
        current = &node.next;
    }
    println!();
}

fn main() {
    let step = 3;
    let mut list = List {
        head: Some(Box::new(Node {
            val: 0,
            next: None,
        })),
        current: 0,
        len: 1,
    };

    // list.skip(step);
    list.insert(17);
    list.insert(12);

    print_list(&list);
}

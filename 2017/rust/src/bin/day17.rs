use std::rc::Rc;

type Link = Option<Rc<Node>>;

struct Node {
    val: i32,
    next: Link,
}

struct List {
    head: Link,
    current: Link,
    len: usize,
}

impl List {
    fn skip(&mut self, mut count: usize) {
        // count %= self.len;
        // self.current = (self.current + count) % self.len;

        while count > 0 {

        }

    }

    fn insert(&mut self, val: i32) {
        let mut current = &mut self.head;
        for _ in 0..self.current {
            current = &mut current.as_mut().unwrap().next;
        }

        let next = &mut current.as_mut().unwrap().next;

        let new_node = Rc::new(Node {
            val,
            next: next.take(),
        });

        let node = current.as_mut().unwrap();
        node.next = Some(new_node);

        self.current += 1;
        self.len += 1;
    }
}

fn print_list(list: &List) {
    let mut current: &Link = &list.head;
    let mut i = 0;
    while let Some(node) = current {
        if list.current == i {
            print!("({})  ", node.val);
        } else {
            print!("{}  ", node.val);
        }
        current = &node.next;
        i += 1;
    }
    println!("/ current: {}, len: {}", list.current, list.len);
}

fn part1(list: &List) -> i32 {
    let mut current = &list.head;
    let mut i = 0;
    while let Some(node) = current {
        if i == list.current + 1 {
            return node.val;
        }
        current = &node.next;
        i += 1;
    }
    panic!();
}

fn main() {
    const STEP: usize = 367;

    let mut list = List {
        head: Some(Rc::new(Node {
            val: 0,
            next: None,
        })),
        current: 0,
        len: 1,
    };

    for i in 1..=2017 {
        list.skip(STEP);
        list.insert(i);
        
        if i % 100000 == 0 {
            println!("{}", i);
        }
    }

    println!("Part 1: {}", part1(&list));
}

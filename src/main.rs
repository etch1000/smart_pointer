use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

// Defining our own Smart Pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // Using Box<T> to Point to Data on the Heap
    let ib: Box<i32> = Box::new(15);

    println!("ib = {}", ib);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:#?}", list);

    let a = 5;

    let b = &a;

    assert_eq!(5, a);
    assert_eq!(5, *b);

    let x = 10;
    
    let y = Box::new(x);

    assert_eq!(10, x);
    assert_eq!(10, *y);

    // Using MyBox
    let h = 177;

    let k = MyBox::new(h);

    assert_eq!(177, h);
    assert_eq!(177, *k);

    let m = MyBox::new(String::from("Rust"));

    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

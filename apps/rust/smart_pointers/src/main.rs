use std::rc::Rc;

use _rust_smart_pointers::custom_pointer::{List::{Cons, Nil}, MyBox};

fn main() {
    let b = Box::new(5);
    println!("b data is on the heap 11!!!! => {}", b);

    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));

    dbg!(list);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("Creating our own smart pointer: MyBox");
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let hello = |name: &str| println!("Hello, {name}!");

    let me = MyBox::new(String::from("Chriszzz"));
    println!("Calling MyBox<String> instance works because of deref coersion!");
    hello(&me);
    hello(&me);

    // earling release me from the heap using drop(me)!!!1!!!!;
    drop(me);

    let another_smart_pointer = MyBox::new(String::from("Another one1!!!"));
    hello(&another_smart_pointer);

    println!("An example with Rc<T> reference_counting");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

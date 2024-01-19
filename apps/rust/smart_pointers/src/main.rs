use _rust_smart_pointers::custom_pointer::{List::{Cons, Nil}, MyBox};

fn main() {
    let b = Box::new(5);
    println!("b data is on the heap 11!!!! => {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

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

    
}

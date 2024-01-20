use std::{ops::Deref, rc::Rc};

#[derive(Debug)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}


#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Impl Deref allowing dereference the value, to allow *y like operations.
// Since Deref just allow deref method, we use another impl block.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
  fn drop(&mut self) {
      println!("Dropping MyBox with data");
  }
}

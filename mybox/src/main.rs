use std::ops::Deref;


pub struct MyBox<T>(T);

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn main() {
    println!("Hello, world!");
}

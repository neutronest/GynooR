use std::any::Any;
use std;

pub fn print_typeid<T: std::any::Any>(_: &T) {
    println!("{:?}", std::any::TypeId::of::<T>());
}

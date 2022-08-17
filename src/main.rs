use crate::bindings::{scale, Vec2};

mod bindings;

fn main() {
    unsafe{
        let mut vec = Box::new(Vec2{ x: 5, y: 10 });
        scale(&mut *vec, 2);
        println!("scaled vector: {:?}", *vec);
    }
}

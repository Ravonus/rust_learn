#![allow(dead_code)]
use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes x = {} ", mem::size_of_val(&p2), p2.x);

    let p3 = *p2;
    println!(
        "p3 takes up {} bytes and p3 = {}",
        mem::size_of_val(&p3),
        p3.x
    )
}

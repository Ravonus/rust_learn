use std::mem;

fn main() {
    // u = unsigned 0... 255 + no negative. i is signed -127 + 128
    let a: u8 = 123; //8 bits
    println!("a = {}", a);

    // a = 456;

    //mut kerywork. So we can reasign homie

    let mut b: i8 = 0; //mutable
    println!("b = {}", b);

    b = 12;

    println!("new b = {}", b);

    let mut c = 123456789; //32-bit signed i32
    println!("c ={}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after mods", c);

    //i8 u8 i16 u16 i32 u32 i64 u64
    let z: isize = 123; //isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d = 'x';
    println!("d = {} and size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // defualt double-precision value 8 bytes or 64 bits, f64 if you want 32bit you have to specify it.
    println!("e ={}, size = {} bytes", e, mem::size_of_val(&e));

    let f: f32 = 3.5; // 32bit f32 is forced
    println!("f ={}, size = {} bytes", f, mem::size_of_val(&f));

    //boolean true false 1byte
    let g = false;
    println!("g ={}, size = {} bytes", g, mem::size_of_val(&g));

    let h = 4 > 0; //true
    println!("h ={}, size = {} bytes", h, mem::size_of_val(&h));
}

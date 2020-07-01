//Rust Operators

use std::mem;

const MEANING_OF_LIFE: u8 = 42; // no fixed address

static Z: i32 = 153;

static mut UNSAFE_TEST: i32 = 655;

fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;

        println!("inside, a = {}", a)
    }
    // println!("can't do this{}", b);
    println!("a = {}", a);
}

fn fundamental_data_types() {
    // arithmetic
    let mut a = 2 + 3 * 4; // +-*/
    println!("{}", a);
    a = a + 1; // can't -- ++ in rust but you can do -=,  +=, *=,, /= ,%=
    a -= 2;

    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    //bitwise
    let c = 1 | 2; // | or & AND ^ XOR ! NOR
                   // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);
    //Shift
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    //logical
    let _pi_less_4 = std::f64::consts::PI < 4.0; //trrue
                                                 // > <= >= ==
    let x = 5;
    let _x_is_5 = x == 5; // true
}

fn main() {
    fundamental_data_types();

    scope_and_shadowing();

    println!("meing of life {}", MEANING_OF_LIFE);

    println!("z is {}", Z);

    unsafe {
        println!("unsafe is {}", UNSAFE_TEST);
    }
}

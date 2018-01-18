#![allow(dead_code)]
use std;
use std::mem;


const MEANING_OF_LIFE:u8 = 42; // no fixed address
static mut Z:i32 = 123;



pub fn fundamental_data_types() {
    // signed i = -127 ... 128
    // unsigned u = 0...255
   let a:u8 = 123; // 8bits
    println!("a = {}", a);
    // a = 456;

    // mut
    let mut b:i8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; // 32-bit signed i32
    println!("c = {}, size = {} bytes", c , mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification", c);
    
    // i8 u8 i16 u16 i32 u32 i64 u64
    // isize
    // usize
    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d = 'x';
    println!("d = {}, size = {} bytes", d , mem::size_of_val(&d));
    
    let e:f32 = 2.5; // double-precision, 8 bytes or 64 bits, f64 / f32
    println!("e = {}, size = {} bytes", e , mem::size_of_val(&e));

    // true false
    let g = false;
    println!("g = {}, size = {} bytes", g , mem::size_of_val(&g));
    // let f = 4 > 0; //true

}

pub fn operators() {
    //arithmetic
    let mut a = 2+3*4;
    println!("{}", a);
    a = a + 1;
    a -= 2;
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed is {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                    // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    let x = 5;
    let x_is_5 = x == 5; // true
    println!("pi < 4.0 = {}", pi_less_4);
    println!("x == 5 = {}", x_is_5);
}

pub fn scope_and_shadowing() {
    let a = 123;
    {
        let b = 456;
        println!("inside, b = {}", b);
        let a = 777;
        println!("inside, a = {}", a);
    }
    println!("outside, a = {}", a);
    //println!("outside, b = {}", b);
}

pub fn outside() {
    println!("{}", MEANING_OF_LIFE);
    unsafe {
        println!("{}", Z);
    }
}

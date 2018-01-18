#![allow(dead_code)]
use std::mem;

struct Point {
    x: f64,
    y: f64
}

struct GPoint<T> {
    x: T,
    y: T
}

struct Line {
    start: Point,
    end: Point
}

pub fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };
    println!("start is ({}, {}), end is ({}, {})",
        myline.start.x, myline.start.y, myline.end.x, myline.end.y
    );
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}
}

pub fn enums() {
    //let c:Color = Color::RgbColor(10,0,0);
    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};
    
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0) | Color::CmykColor{cyan:_,magenta:_,yellow:_,black:255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})",r,g,b),
        _ => ()
        //_ => println!("some other color")
    }
}

union IntOrFloat {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => { println!("meaning of life"); }
            IntOrFloat { f } => { println!("f32 = {}", f); }
        }
    }
}

pub fn unions() {
    let mut iof = IntOrFloat{ i: 123 };
    
    //unsafe {
        iof.i = 42;
    //}

    let value = unsafe { iof.i };
    process_value(iof);
    process_value(IntOrFloat { f: 1.23 });
}

pub fn option() {
    // Option<T>
    let x = 3.0;
    // let y = 2.0;
    let y = 0.0;

    // Some(z) None

    let result:Option<f64> = 
        if y != 0.0 { Some(x/y) } else { None };

    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y)
    }

    // if let / while let
    if let Some(z) = result { println!("z = {}", z); }
}

pub fn arrays() {
    let mut a:[i32;5] = [1,2,3,4,5];
    println!("a has {} elements, first is {}",
        a.len(), a[0]
    );
    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a);

    if a != [1,2,3,4,5]
    {
        println!("not match");
    }

    let b = [1u16;10]; //b.len() == 10
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] = 
        [
            [1.0,0.0,0.0],
            [0.0,2.0,0.0]
        ];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
    
}

pub fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);

    a.push(44);
    println!("a = {:?}", a);

    // usize isize
    // let idx:i32 = 0;
    let idx:usize = 0;
    // let idx:usize = 111;

    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    // Option
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(77);
    println!("{:?}", a);

    let last_elem = a.pop(); //Option
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    // let Some(last_value) = a.pop();

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn use_slice(slice: &mut[i32]) {
    println!("first element = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

pub fn slices() {
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]);
    //use_slice(&mut data);
    println!("{:?}", data);
}

pub fn strings() {
    // utf-8
    let s:&'static str = "hello there!"; // &str = string slice
    // s = "abc";
    // let h = s[0];
    // println!("{}", s);
    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    // heap
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a +=1;
    }
    println!("{}", letters);

    // &str <> String
    let u:&str = &letters;

    // concatentation
    //let z = letters + &letters;
    //let mut abc = String::from("hello world");
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

fn sum_and_product(x:i32, y:i32) -> (i32,i32) {
    (x+y, x*y)
}

pub fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);
    println!("sp = {:?}", sp);

    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    let (a,b) = sp;
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, a, b);
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4,7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem {}", (combined.1).1);

    let ((c,d), (e,f)) = combined;
    println!("last elem {}", f);

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);
    
    let meaning = (42,);
    println!("{:?}", meaning);
}

struct GLine<T> {
    start: GPoint<T>,
    end: GPoint<T>
}

pub fn generics() {
    let a:GPoint<f64> =  GPoint { x: 0.0, y: 0f64 }; // let a = GPoint { x: 0, y: 0 };
    let b = GPoint { x: 1.2, y: 3.4 }; // let b = GPoint { x: 1.2, y: 3.4 };

    let myline = GLine { start: a, end: b };
}
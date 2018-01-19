#![allow(dead_code)]
#![allow(unused_imports)]

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

use std::ops::Add;
impl Add for Point {
    type Output = Point;

    fn add(self, other:Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

trait Animal {

    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {

    fn create(name:&'static str) -> Human {
        Human { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {

    fn create(name:&'static str) -> Cat {
        Cat { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self { result += *x; }
        return result;
    }
}

pub fn traits() {
    // let h = Human { name: "John" };
    let h:Human = Animal::create("John");
    h.talk();

    // let c = Cat { name: "Misty" };
    let c = Cat::create("Misty");
    c.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}

pub fn overloading() {
    let p = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let p3 = p + p2;
    println!("{:?}", p3);
}

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
} // monomorphisation

fn print_it_too(z: &Printable) {
    println!("{}", z.format());
}

pub fn dispatch() {
    let a = 123;
    let b = "hello".to_string();
    // let c = 5.6;

    // println!("{}", a.format());
    // println!("{}", b.format());

    // print_it(a);
    // print_it(b);
    // print_it(c);

    print_it_too(&a);
    print_it_too(&b);
}
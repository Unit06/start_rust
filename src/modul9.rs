#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

//Module 8 - Memory management
//Ownership
//Borrowing
//Lifetimes
//Reference counted variables

//-----------------------------------------Ownership
//Only one variable can own a piece of memory
//For primitive types, copying data is cheap
//For complex types, ownership is transferred

pub fn mod9_ownership() {
    let i = 5;
    let j = i;
    println!("{}", j);
    println!("{}", i);

    let v = vec![1, 2, 3, 4, 5];
    // let w = v;
    // println!("{:?}", w);
    // println!("{:?}", v);

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vector used in foo");
        v
    };
    let v = foo(v);
    println!("{:?}", v);
}

//-----------------------------------------Borrowing
//Only one variable can own a piece of memory
//Variables can borrow ownership to other pieces of memory
//let a = 6;
//let b = &a;
//println!(“{}”, *b);
//a += 2;		// error
//The borrow has to match the mutability
pub fn mod9_borrowing() {
    let mut a = 6;
    {
        let b = &mut a;
        println!("{}", *b);
        *b += 2;
    }
    println!("{}", a);

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
        //v.push(6);
    }
}

//-----------------------------------------Lifetimes
//An indication of how long an object will live
// Rust prevents parts of objects outliving the object
// struct Object<’lifetime> {
//    field: &’lifetime str
// }
// Lifetime elision - compiler builds lifetimes for us when evident
#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person
}

impl Person {
    // fn get_name(&self) -> &String {
    fn get_name<'l> (&'l self) -> &'l String {
        &self.name
    }
}

pub fn mod9_lifetime() {
    println!("{}", get_str());

    let p1 = Person { name: String::from("John") };
    let d1 = Dog { name: String::from("Max"), owner: &p1};
    println!("{:?}", d1);

    let mut a: &String;
    {
        let p2 = Person { name: String::from("Mary")};
        // a = p2.get_name();
        a = p1.get_name();
    }
    println!("{}", a);
}

fn get_str() -> &'static str {
    "Hello"
}

//-----------------------------------------Reference counted variables
//A structure that can hold multiple references to a variable
// Can be shared in different places
//use std::rc::Rc;
//fn do_smth(var: Rc<String>) ...
//let var = Rc::new(String::from(“test”));
//var.clone()
//Count the variable pointers
//Rc::strong_count(&var)
use std::rc::Rc;

struct Car {
    brand: Rc<String>
}

impl Car {
    fn new(brand: Rc<String>) -> Car { Car { brand: brand} }
    fn drive(&self) {
        println!("{} is driving", &self.brand);
    }
}

pub fn mod9_rc_var() {
    let brand = Rc::new(String::from("BMW"));
    println!("pointers: {}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("pointers: {}", Rc::strong_count(&brand));
    }
    println!("My car is a {}", brand);
    println!("pointers: {}", Rc::strong_count(&brand));
}
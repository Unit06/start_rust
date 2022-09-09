#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

//Module 8 - Traits
//Traits - чем то похожи на интерфейсы (определяют что делать со структурами) или абстрактный класс
//Traits generics - использование дженериков, чтобы моч в разные типы параметров в одной функции
//Returning traits -
//Adding traits to existing structures -
//Operator overloading -
//Static dispatch -
//Dynamic dispatch -

//-----------------------------------------Traits
//Similar to an interface or abstract class
// Add a definition to a structure
//trait Name {
//    fn must_implement(&self) -> i32;
//    fn do_action(&self) { ... }
//    fn do_non_instance_action() { ... }
// }
//
// Can have definition only or default implementation
// Can have instance and non-instance action
//-----------------------------------------
//Implement a trait
//impl Name for Person {
//    fn must_implement(&self) -> { 42 }
//    fn new(name: &str) -> Person {
//       Person{name: name}
//    }
// }
//
// Can provide a constructor
//trait Name {
//    fn new(name: &str) -> Self;
// }
//let john = Person::new(“John”);

struct RustDev {
    awesome: bool
}

struct JavaDev {
    awesome: bool
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) { println!("Hello world!") }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello world!\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Java 1.8"
    }

    fn say_hello(&self) {
        println!("System.out.println(\"Hello world!\");");
    }
}

pub fn mod8_trait1() {
    // let r = RustDev { awesome: true};
    let r = RustDev::new(true);
    let j = JavaDev::new(false);
    println!("{}", r.language());
    r.say_hello();
    println!("{}", j.language());
    j.say_hello();
}

//-----------------------------------------Traits generics
//Generics can be limited by traits
//fn color<T: Colorable> (a: T) {
//    ...
// }

trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str
}

struct Cat {
    color: &'static str
}

impl Bark for Dog {
    fn bark(&self) -> String {
        return format!("{} barking", self.species)
    }
}

fn bark_it<T: Bark>(b: T) {
    println!("{}", b.bark())
}

pub fn mod8_trait2() {
    let dog = Dog { species: "retriever" };
    let cat = Cat { color: "black" };
    bark_it(dog);
    // bark_it(cat);
}

//-----------------------------------------Returning traits
//The compiler needs to know the space required for a function return type
// A workaround is to return a box with a dyn trait
//fn get_animal() -> Box<dyn Animal> {
//    ...
// }
// dyn is a new addition to the language old code might not have it

struct Dog2 {}
struct Cat2 {}

trait Animal {
    fn make_noise(&self) -> &'static str;
}

impl Animal for Dog2 {
    fn make_noise(&self) -> &'static str {
        "woof"
    }
}

impl Animal for Cat2 {
    fn make_noise(&self) -> &'static str {
        "meow"
    }
}

fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        Box::new(Dog2 {} )
    } else {
        Box::new( Cat2 {} )
    }
}

pub fn mod8_trait3() {
    println!("The animal says {}", get_animal(0.5).make_noise());
    println!("The animal says {}", get_animal(2.0).make_noise());
}

//-----------------------------------------Adding traits to existing structures
//We can add a trait to a structure we didn’t create
//impl My_Trait for Vec<i32> {
//    ...
// }

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

pub fn mod8_trait4() {
    let a = vec![1, 2, 3, 4, 5];
    println!("sum = {}", a.sum());
    // let b = vec![1.0, 2.0, 3.0];
    // println!("sum float = {}", b.sum());
}

//-----------------------------------------Operator overloading
//We can implement standard operators for our custom structs
//use std::ops::Add;
//struct Custom {
//    ...
// }
//impl Add for Custom {
//    type Output = Custom;
//    fn add(self: Custom, other: Custom) -> Custom {
//       ...
//    }
// }
//custom1 + custom2

use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

pub fn mod8_trait5() {
    let p1 = Point { x: 1.3, y: 4.6 };
    let p2 = Point { x: 3.7, y: 1.4 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}

//-----------------------------------------//Static dispatch
//A generic trait will be converted to the required type at compile time
// Monomorphization converting to one form

trait Duplicateable {
    fn dupl(&self) -> String;
}

impl Duplicateable for String {
    fn dupl(&self) -> String {
        format!("{0}{0}", *self)
    }
}

impl Duplicateable for i32{
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}

fn duplicate<T: Duplicateable> (x: T) {
    println!("{}", x.dupl());
}

pub fn mod8_stat_dis() {
    let a = 42;
    let b = "Hi John ".to_string();
    duplicate(a);
    duplicate(b);
}

//-----------------------------------------//Dynamic dispatch
//A generic trait will be converted to the required type at run time

trait Duplicateable2 {
    fn dupl2(&self) -> String;
}

impl Duplicateable2 for String {
    fn dupl2(&self) -> String {
        format!("{0}{0}", *self)
    }
}

impl Duplicateable2 for i32{
    fn dupl2(&self) -> String {
        format!("{}", *self * 2)
    }
}

// fn duplicate<T: Duplicateable> (x: T) {
//     println!("{}", x.dupl());
// }

fn duplicate2(x: &dyn Duplicateable2) {
    println!("{}", x.dupl2());
}

pub fn mod8_stat_dyn() {
    let a = 42;
    let b = "Hi John ".to_string();
    // duplicate(a);
    // duplicate(b);
    duplicate2(&a);
    duplicate2(&b);
}



















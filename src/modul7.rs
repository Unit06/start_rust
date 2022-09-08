#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

//Module 7 - Functions
//Functions and scope
//Closures - лямбда или же анонимные функции
//Higher Order Functions - берут другую функцию как параметр
//Macros - метпрограммирование макросов

//Functions and scope
static mut R: i32 = 0;

//обычная функция
//fn main() {
//    say_hi();
// }
//
// fn say_hi() {
//    println!("Hello there!");
// }

//функция с передачей параметра
//fn main() {
//    say_hi("John");
// }
//
// fn say_hi(name: &str) {
//    println!("Hello {}!", name);
// }

//функция с передачей параметра и возврата переменной
//fn main() {
//    let mut name = "John";
//    say_hi(&mut name);
//    print!("The new name is {}", name)
// }
//
// fn say_hi(name: &mut &str) {
//    *name = "Alex";
//    println!("Hello {}!", name);
// }
//
//fn main() {
//    let mut name = "John";
//    let greeting = say_hello(&mut name);
//    println!("{}", greeting);
// }
//
// fn say_hello(name: &mut &str) -> String {
//    let greeting = format!("Hello {}", name);
//    greeting
// }

//scope
//No memory leaks - no need to manually deallocate variables
//{
//    let a = 3;
// }
// println(“a = {}”, a)		// error
//Global variables can be declared but they are unsafe
//let a = 3;
//fn main() {
//    unsafe { println!(“{}”, a); }
// }

pub fn fun_scop() {
    {
        let a = 3;
        println!("{}", a);
    }
    // println!("{}", a);

    unsafe {
        R = 4;
        println!("R = {}", R);
    }

    unsafe {
        println!("R = {}", R);
    }
}

//Closures - лямбда или же анонимные функции
//A function within a function
//An anonymous function, lambda expression
//|a: i32, b: i32| println!(“{}”, a + b);
//|a: i32, b: i32| -> i32 { a + b };
//A function can be assigned to a variable
//let sum = |a: i32, b: i32| -> i32 a + b;
//sum(2, 3);
//A clojure can be generic
//let gen = |x| { println!("received {}", x) };
//gen(3);

pub fn mod7_clousers() {
    let a = |a: i32| a+1;
    println!("{}", a(6));
    let b = |b: i32| {
        let c = b + 1;
        c
    };
    println!("{}", b(4));

    let gen = |x| println!("{}", x);
    gen(3);
    // gen(true);
}

//Higher Order Functions - берут другую функцию как параметр
//Functions that take another function as a parameter
//fn apply(f: fn(i32) -> i32, a: i32) {
// }
//apply(|x| -> x + 1, a);

pub fn mod7_hofs() {
    let square = |a: i32| a * a;
    apply(square, 6);

    // Calculate the sum of all the squares less than 500
    // only for even numbers
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit { break; }
        else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("The sum is {}", sum);

    //With HOFs
    let sum2 =
        (0..).map(|x| x * x)
            .take_while(|&x| x <= limit)
            .filter(|x| is_even(*x))
            .fold(0, |sum, x| sum + x);
    println!("The sum using HOFs is {}", sum2);
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result {}", f(a));
}


//Macros - метпрограммирование макросов
//Write code that writes code - meta programming
//Match an expression and perform some operation
//Code is expanded and compiled
//macro_rules! my_macro {
//    (match) => ( code to run )
// }
//my_macro!
//println!(“This is an {} macro”, “awesome”);
//We can match multiple expressions
//macro_rules! my_macro {
//    (match1) => ( code to run )
//    (match2) => ( code to run )
// }
//Designators
// expr         path
// ident        meta
// block        ty
// stmt         tt
// pat

macro_rules! my_macro {
    () => (println!("First macro"))
}

// macro_rules! name {
//     ($name: expr) => { println!("Hey {}", $name)}
// }

macro_rules! name {
    ($($name: expr),*) => ( $(println!("Hey {}", $name);)* )
}

macro_rules! xy {
    (x => $e: expr) => (println!("X is {}", $e));
    (y => $e: expr) => (println!("Y is {}", $e));
}

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name));
        }
    }
}

pub fn mod7_macros() {
    my_macro!();
    name!("John");
    name!("Alex", "Mary", "Carol");
    xy!(x => 5);
    xy!(y => 3 * 9);
    build_fn!(hey);
    hey();
}



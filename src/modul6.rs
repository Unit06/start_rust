#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

use rand::Rng; //for if
use crate::modul6::Suit::{Heart, Club, Diamond, Spade}; //for match

/*
Control structures

If statement
Match statement
Pattern matching
For loop
While loop
 */

pub fn if_statement() {
    //if logical_expression {
    //    functionality for true
    // }
    //
    //if logical_expression {
    //    functionality for true
    // } else {
    //    functionality for false
    // }
    //If statement can return a result
    //let res = if expr1 {
    //    result for true
    // } else {
    //    result for false
    // }

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 11);
    // if num >= 5 {
    //     println!("Number {} is greater than or equal to 5", num);
    // } else {
    //     println!("Number {} is smaller than 5", num);
    // }

    if num > 5 {
        println!("{} > 5", num);
    } else if num == 5 {
        println!("{} == 5", num);
    } else {
        println!("{} < 5", num);
    }

    let res = if num >= 5 { true } else { false };
    println!("{}", res);
}

//---------------------------Match
//Similar to when or switch in other languages
//match expression {
//    expr1 => { … }
//    expr2 => { … }
//    _ => { … }
// }
//
// Can return a result
// Ranges are allowed

pub fn match_statement() {
    print_choice(Heart);
    print_choice(Club);
    print_choice(Diamond);
    print_choice(Spade);

    country(44);
    country(34);
    country(125);
    country(-15);
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "unknown",
        _ => "invalid"
    };
    println!("Country is {}", country);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond
}

fn print_choice(choice: Suit) {
    match choice {
        Heart => { println!("\u{2665}") }
        Spade => { println!("\u{2660}") }
        Club => { println!("\u{2663}") }
        Diamond => { println!("\u{2666}") }
    }
}

//---------------------------Pattern matching
//Multiple values 1 | 2
// Ranges 1..=5
// Conditions x if a > b
// Tuple matching
//
//match expression {
//    expr1 => { … }
//    expr2 => { … }
//    _ => { … }
// }

pub fn pattern_matching() {
    for i in 0..15 {
        println!("{}. I have {} oranges", i, get_oranges(i));
    }

    // let point = (0, 0);
    // let point = (6, 0);
    // let point = (0, 5);
    let point = (2, 5);

    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0, {})", y),
        (x, y) => println!("({}, {})", x, y),
    }
}

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "lots of"
    }
}

//---------------------------For loop
//Loop through a collection or range, execute code for each element
//for element in collection {
//    functionality
// }
//
// Continue will skip a step
// Break will stop the loop
pub fn loop1() {
    for i in 1..11 {
        println!("{0} * {0} = {1}", i, i * i);
    }

    let pets = ["cat", "dog", "chihuahua", "bear", "hamster"];
    for pet in pets.iter() {
        if pet == &"chihuahua" {
            println!("{} barks too much", pet);
            continue
        }
        if pet == &"bear" {
            println!("{} is not a pet", pet);
            break
        }
        println!("I love my {}", pet);
    }

    for (pos, i) in (1..11).enumerate() {
        let square = i * i;
        let nb = pos + 1;
        println!("{0} * {0} = {1} ", nb, square);
    }
}

//---------------------------While loop
//Loop as long as a condition is true
//while condition {
//    ...
// }
//
// Continue skips a step
// Break stops the loop
//loop {			// while true {
//    ...
// }
pub fn loop2() {
    get_squares(3478);
    get_cubes(4938);
}

fn get_squares(limit: i32) {
    let mut x = 1;
    while x * x < limit {
        println!("{0} * {0} = {1}", x, x * x);
        x += 1;
    }
}

fn get_cubes(limit: i32) {
    let mut x = 1;
    loop {
        println!("{0} * {0} * {0} = {1}", x, x * x * x);
        x += 1;
        if x * x * x > limit {
            break
        }
    }
}


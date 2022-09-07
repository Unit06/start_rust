#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]
/*
Udemy Course.

A language for Rustaceans. Learn the basics and advanced concepts, including memory
management and concurrency.

Module 2 - Project basics
Module 3 - Language Basics
Module 4 - Modules and Crates
Module 5 - Data types
Module 6 - Control structures
Module 7 - Functions
Module 8 - Traits
Module 9 - Memory management
Module 10 - Error handling
Module 11 - Concurrency

Catalin Stefan
 */

//Подключаемые библиотеки
//----------------------------------Crates
//use crate::archive::arch::arch_file;  //mod4 Crates
//use crate::archive::arch::arch_file as arc; //mod4 Crates
//use rand::Rng; //mod4 Crates
//----------------------------------numbers generator
//use rand::Rng;
//use rand::{thread_rng, Rng};
//use rand::distributions::Alphanumeric;

//Модули занятий подключаются как доп. файлы. Список модулей указан выше.
//mod modul2;
//mod modul3;
//mod modul5;
//mod modul6;
mod modul7;
mod modul8;
mod modul9;
mod modul10;
mod modul11;

//mod player; //к модулю 4 о модулях

//Модули, которые относятся к самим занятиям и являются чисто учебными
//mod archive;
//mod player;

fn main() {
    //----------------------------------вывод модуля 2
    //modul2::user_input(); //вывод ввода с клавиатуры
    //modul2::print_var(); //вывод разного вида переменных

    //----------------------------------вывод модуля 3
    //modul3::mod3_var();
    //modul3::mod3_string();
    //modul3::mod3_const();
    //modul3::mod3_oper();
    //использование функций
    /*let mut name = "John";
    let greeting = modul3::say_hello(&mut name);
    println!("{}", greeting);
    */

    //----------------------------------вывод модуля 4
    //----------------------------------Modules
    //player::play_movie("snatch.mp4");
    //player::play_audio("rhcp.mp3");
    //clean::perform_clean();
    //clean::files::clean_files();
    //----------------------------------Crates
    //arch_file("somefile.txt");
    //arc("somefile.txt");
    //let mut rng = rand::thread_rng();
    //let a: i32 = rng.gen();
    //println!("{}", a);

    //----------------------------------numbers generator
    //let mut rng = rand::thread_rng();
    //rng.gen();
    //let a: i32 = rng.gen(); //Generate an integer
    //let b: i32 = rng.gen_range(0, 100); //Bounded generation
    //println!("{}", a);
    //println!("{}", b);
    //----------------------------------String generator
    //for i in 1..6 {
    //    let rand_string: String = thread_rng()
    //        .sample_iter(&Alphanumeric)
    //        .take(30)
    //        .collect();
    //    println!("{}", rand_string)
    //}
    //----------------------------------вывод модуля 5 Массивы
    //-------------Arrays
    //modul5::mod5_arrays();
    //-------------Vectors
    //modul5::mod5_vectors();
    //-------------Slices
    //let numbers = [1, 2, 3, 4, 5];
    //let slice = &numbers[1..4];
    //println!("{:?}", slice);
    //let mut colors = ["red", "green", "blue", "pink"];
    //println!("{:?}", colors);
    //modul5::mod5_slices(&mut colors[2..4]);
    //println!("{:?}", colors);
    //-------------Tuples
    //modul5::mod5_tuples();
    //-------------Structures
    //modul5::mod5_structures();
    //-------------Enums
    //modul5::mod5_enums();
    //-------------Generics
    //modul5::mod5_generics();
    //----------------------------------вывод модуля 6
    //modul6::if_statement();
    //modul6::match_statement();
    //modul6::pattern_matching();
    //modul6::loop1();
    //modul6::loop2();
    //----------------------------------вывод модуля 7
}

//Модуль 4
//можно объединять функции в модули при помощи mod
/*
mod clean {

    pub fn perform_clean() {
        println!("Cleaning hdd");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files");
        }
    }
}
 */
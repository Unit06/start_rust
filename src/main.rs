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
//use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;
//use rand::Rng;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

//Модули занятий подключаются как доп. файлы. Список модулей указан выше.
mod modul2;
mod modul3;
mod modul4;
mod modul5;
mod modul6;
mod modul7;
mod modul8;
mod modul9;
mod modul10;
mod modul11;

//Модули, которые относятся к самим занятиям и являются чисто учебными
mod archive;
//mod player;

fn main() {
    //player::play_movie("snatch.mp4");
    //player::play_audio("rhcp.mp3");
    arc("file111.txt");
    //numbers generator
    //let mut rng = rand::thread_rng();
    //let a: i32 = rng.gen();
    //let b: i32 = rng.gen_range(0, 100);
    //println!("{}", a);
    //println!("{}", b);
    //String generator
    for i in 1..6 {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();
        println!("{}", rand_string)
    }
}
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

/* Менеджер пакетов Cargo

cargo new - создание нового проекта
cargo build - компиляция
cargo run - запуск
cargo clean - очистка. Удаляет все скомпилированные ранее файлы, временные тоже.
Необходимо когда много ошибок и проект надо перекомпилировать.
cargo check - проверка проекта "исходного кода"
cargo doc - создание документации исходного кода на базе комментариев(ниже)

 */
use std::io;
//User input.
//Язык не очень подходит для взаимодействия с пользователем, ниже пример ввода
//произвольной строки.
pub fn user_input() {
    let mut input = String::new();
    println!("Say someting");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input);
        },
        Err(e) => {
            println!("Something went wrong {}", e);
        }
    }
}

/* Комментарии

// This is a line comment

/* This is not
very common */ - Multi line comments are allowed but rarely used

Doc comments
/// This is mainly used to document functionality
//! This is mainly used to document crates
//! # Main heading
//! ```
//! fn main() {...} - code comment
//! ```
DocLink - https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md
*/

// Вывод переменных
pub fn print_var() {
    println!("Hello world!"); //обычный вывод
    println!("My name is {} and I’m {}", "Alex", 29); //форматированный вывод
    println!("a + b = {}", 3+6); //вывод выражения
    println!("{0} has a {2} and {0} has a {1}", "Alex", "cat", "dog"); //позиционный вывод
    println!("{name} {surname}", surname="Smith", name="Alex"); //именованный вывод
    println!("binary: {:b}, Hex: {:x}, Octal: {:o}", 5, 5, 5); //вывод разных форматов чисел
    println!("Array {:?}", [1, 2, 3]); //вывод сложных структур (необходимо указание формата)
}



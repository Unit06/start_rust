#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]
// переменные
pub fn mod3_var() {
let name = "Michael"; //
let age = 32; //
//Rust является сильно типизированным языком
//Тип переменной необязателен, если его можно определить
//Тип может быть указан в явном виде
let amount: i64 = 8473926758472;
//let amount = 8473926758472 - error

//Имена могут содержать буквы, цифры и знаки подчеркивания
//Должны начинаться с буквы или символа подчеркивания
//Соблюдайте соглашение об именовании snake_case
//по умолчанию переменные не изменяемые
let length = 34;
//length = 35;	- при попытке вывести изменить переменную выведет ошибку
//изменяемая переменная
let mut mut_length = 34;
mut_length = 35; // ошибки при изменении нет!

//переопределение разрешено
let color = "blue";
let color = "red";
println!("Color is {}", color);		// Color is red

//объявление нескольких переменных
let (a, b, c) = (2, 3, 4);

//Типы данных
/*
Integer
Size        signed  unsigned
8 bit       i8      u8
16 bit      i16     u16
32 bit      i32     u32
64 bit      i64     u64
128 bit     i128    u128
arch        isize   usize
 */

/*
float
32 bit - f32
64 bit - f64
 */
//let pi: f32 = 4; - ошибка несовпадения формата данных

//разделитель для более удобного чтения длинных чисел
let million = 1_000_000;
let random = 3_836.45_346_546;

//булевый тип
let is_day = true;
let is_night = false;

//символьный тип
let char1 = 'A';
let smiley_face = '\u{1F601}';

}

pub fn mod3_string() {
let cat: &str = "Fluffy";
println!("{}", cat);
let cat: &'static str = "Fluffy"; //статическое объявление
println!("{}", cat);
//строки неизменяемы. Для того, чтобы их изменять, надо создавать объект
let dog = String::new();
println!("{}", dog);
let mut dog = String::from("Max"); //добавление данных в объект
println!("{}", dog);
let owner = format!("Hi I'm {} the owner of {}", "Mark", dog); //добавление форматированных данных
println!("{}", owner);
//работа со строками, как пример (методы)
println!("{}", dog.len()); //длина строки
dog.push(' '); //добавление пробела в конец
dog.push_str("the dog"); //добавление строку в конце
println!("{}", dog);
let new_dog = dog.replace("the", "is my"); //замена внутри строки
println!("{}", new_dog);

}

pub fn mod3_const() {
const URL: &str = "google.com"; //использует const, заглавные буквы, определение типа
//могут быть как глобальные, так и локальные. Переопределение запрещено.
println!("{}", URL);
}

pub fn mod3_oper() {
/*
обычные операторы:
сложение - +
вычитание - -
умножение - *
деление - /
остаток от деления - %
++ и -- не поддерживаются

Операторы сравнения - > >= < <= == !=
Логические операторы -  && AND, || OR, ! NOT
Побитовое смещение -  & AND, | OR, ^ XOR, ! NOT, << left shift, >> right shift
 */

let a = 4 + 8;
let b = 10 / 3;
let c = 10 % 3;
println!("a={}, b={}, c={}", a, b, c);
println!("{}", a >= b);
println!("{}", a >= b && b <= c);
}

//функции
pub fn say_hello(name: &mut &str) -> String {
    let greeting = format!("Hello {}", name);
    greeting
}
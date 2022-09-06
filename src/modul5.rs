#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

//-------------for Enums
use crate::modul5::Colors::Red;
//use crate::modul5::Person::Name;

//Data types
//-------------Arrays
/*
Static - cannot be resized
Element values can be modified but not deleted
Indexed
 */
pub fn mod5_arrays() {
    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    println!("{:?}", primes);
    println!("{:?}", doubles);

    let numbers = [0;15];
    println!("{:?}", numbers);

    //Create array with default values
    const DEFAULT: i32 = 3;
    let mut numbers = [DEFAULT; 15];
    println!("{:?}", numbers);
    println!("{:?}", numbers[3]);

    //Updating elements
    numbers[3] = 5;
    println!("{:?}", numbers);

    //Using an iterator
    for number in numbers.iter() {
        println!("{}", number);
    }
}
//-------------Vectors - not default struct in rust
pub fn mod5_vectors(){
    let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 5];  //macro vec
    println!("{:?}", primes);
//Adding elements
    primes.push(7);
    println!("{:?}", primes);
//Removing elements
    primes.remove(2);
    println!("{:?}", primes);
//Create vectors with default values
    let mut numbers = vec![2;10];
    println!("{:?}", numbers);

    const DEFAULT: bool = true;
    let values = vec![DEFAULT;8];
    println!("{:?}", values);
//Updating elements
    numbers[5] = 8;
    println!("{:?}", numbers);
//Using an iterator
    for number in numbers.iter() {
        println!("{}", number * number);
    }
}

//-------------Slices
//A slice is a pointer to a block of memory
pub fn mod5_slices(colors_slice: &mut [&str]){
    colors_slice[0] = "yellow";
    colors_slice[1] = "orange";
    // colors_slice[2] = "brown";
//Size is determined at runtime
// Can be used on arrays, vectors and strings
// Indexed
// Mutable slices allow us to change values
}

//-------------Tuples
//A collection of values of various types
pub fn mod5_tuples(){
    let mut person: (&str, i64, bool) = ("John", 27, true);
    println!("{:?}", person);
    println!("{:?}", person.0); //Accessing elements
    person.0 = "Mike"; //Updating elements
    println!("{:?}", person.0);
    let (name, age, employment) = person; //Destructuring a tuple
    //number of variables must correspond to number of elements
    println!("name: {}, age: {}, employed: {}", name, age, employment);
    // Static - cannot be resized
    // Element values can be updated
    // Indexed
    // Limited to 12 elements
}

//-------------Structures
//A collection of key-value pairs
//Look like a Class (Rust dont have concept of object and class)
pub fn mod5_structures(){
    let emp = Employee {
        name: String::from("John"),
        company: String::from("Google"),
        age: 35
    };

    println!("{:?}", emp); //печать всей структуры
    println!("{}", emp.name); //печать отдельного элемента
    println!("{}", emp.fn_details()); //печать значений структуры(функция)
    println!("{}", Employee::static_fn_detail()); //использование метода(статическая функция)
}
/*
//-------------Enums
//A collection of values
pub fn mod5_enums(){
    let my_color = Colors::Red;
    println!("{:?}", my_color);
    let my_color = Red;
    println!("{:?}", my_color);

    let person = Name(String::from("Alex"));
    println!("{:?}", person);
}
*/
//-------------Generics
//Allows us to have variable data types
pub fn mod5_generics(){
    let p1: Point<i32> = Point {X: 6, Y: 8};
    let p2: Point<f64> = Point {X: 3.25, Y: 8.63};
    println!("{:?}", p1);
    println!("{:?}", p2);

    let c1 = Red("#f00");
    let c2 = Red(255);
    println!("{:?}", c1);
    println!("{:?}", c2);

    let p3: Point2<i32, f64> = Point2 {x: 34, y: 8.5};
    println!("{:?}", p3);
}

//-------------for Structures
#[derive(Debug)] //объявление необходимое для вывода печати структур
struct Employee { //создание самой структуры
    name: String,
    company: String,
    age: u32
}
//Adding methods to a structure
impl Employee {
    fn fn_details(&self) -> String {
        format!("name: {}, age: {}, company: {} ", &self.name, &self.age, &self.company)
    }
//A structure can have static methods
// не использует &self, таким образом не берёт данные из самой структуры
    fn static_fn_detail() -> String {
        String::from("Details of a person")
    }
}
/*
//-------------for Enums
//We can add data types to enum elements
#[derive(Debug)] //объявление необходимое для вывода печати структур
enum Colors {
    Red,
    Green,
    Blue
}

#[derive(Debug)] //объявление необходимое для вывода печати структур
enum Person {
    Name(String),
    Surname(String),
    Age(u32)
}
*/
//-------------for Generics
#[derive(Debug)]
struct Point<T> {
    X: T,
    Y: T
}

#[derive(Debug)]
enum Colors<T> {
    Red(T),
    Blue(T),
    Green(T)
}

#[derive(Debug)]
struct Point2<T, V> {
    x: T,
    y: V
}

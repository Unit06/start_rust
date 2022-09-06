pub mod arch {
    pub fn arch_file(name: &str) {
        println!("Archiving file {}", name)
    }
}

// Crates - abstraction on top of modules. Structure multiple modules together inside one unit
// two types - binary, library
// cargo is used to manage crates
//External crates are imported into the project -  must be added to the toml file
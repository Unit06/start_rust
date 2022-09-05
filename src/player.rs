//можно объединять функции в модули при помощи mod

pub mod module123{

}

pub fn play_movie(name: &str) {
    println!("Play movie {}", name)
}

pub fn play_audio(name: &str) {
    println!("Play audio {}", name)
}

// Crates - abstraction on top of modules. Structure multiple modules together inside one unit
// two types - binary, library
// cargo is used to manage crates
//external creates import - add to toml file











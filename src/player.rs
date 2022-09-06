pub fn play_movie(name: &str) {
    println!("Play movie {}", name)
}

pub fn play_audio(name: &str) {
    println!("Play audio {}", name)
}

/*
Модули можно делать вложенными:
pub mod mod_name {
   pub mod submod {
      fn fun_submodule() {
         ...
      }
   }
}

внутри основной функции они будут вызываться как:
mod mod_name;

fn main() {
   mod_name::submod::fun_submodule();
}

 */











use tester::*;

#[path="../tester.rs"]
mod tester;

mod io {
  extern mod console;
}

mod drawing {
  extern mod color;
}

fn main() {
  let mut console = io::console::Console {
    foreground: drawing::color::Color::rgba(1.0, 0.0, 0.0, 1.0),
    background: drawing::color::Color::rgba(0.0, 0.0, 0.0, 1.0)
  };

  console.foreground = drawing::color::Color::rgba(1.0, 1.0, 0.0, 1.0);

  io::console::println("hello");
  console.println("hey");

  console.foreground = drawing::color::Color::rgba(1.0, 1.0, 1.0, 1.0);
  console.print("");
}

use std::io::{self, Write};

/// Prints a message to `stdout` if provided.
/// And reads a `String` from `stdin` and returns it
pub fn input(msg: Option<&str>) -> String {
  if let Some(msg) = msg {
      print!("{}", msg);
  
      io::stdout().flush().expect("Unable to print to stdout");
  }

  let mut value = String::new();

  io::stdin().read_line(&mut value)
      .expect("Unable to read from stdin");

  value
}

/// Reads from `stdin` using the `input` function and
/// attempts to parse the integer, if the value
/// provided fails when parsed into an integer
/// panics
pub fn input_int(msg: Option<&str>) -> i32 {
  let value = input(msg);

  value.trim().parse::<i32>().expect(&format!("{} is not a valid number", value))
}

/// Reads from `stdin` using the `input` function and
/// attempts to parse the integer, if the value
/// provided fails when parsed into an integer
/// panics
pub fn input_float(msg: Option<&str>) -> f32 {
  let value = input(msg);

  value.trim().parse::<f32>().expect(&format!("{} is not a valid number", value))
}

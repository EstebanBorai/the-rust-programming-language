use std::fs;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
      if args.len() < 3 {
          return Err("not enough arguments");
      }

      let query = args[1].clone();
      let filename = args[2].clone();

      Ok(Self { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_contents = fs::read_to_string(config.filename)?;

  println!("{}", file_contents);

  Ok(())
}

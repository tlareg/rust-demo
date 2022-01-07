use std::{error::Error, fs::read_to_string};

pub fn box_example() -> Result<(), Box<dyn Error>> {
  let html = render_markdown()?;
  println!("{}", html);
  Ok(())
}

fn render_markdown() -> Result<String, Box<dyn Error>> {
  let file = std::env::var("MARKDOWN")?;
  let source = read_to_string(file)?;
  Ok(markdown::to_html(&source))
}

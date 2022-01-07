use std::fs::read_to_string;

pub fn thiserror_example() -> Result<(), MyError> {
  let html = render_markdown()?;
  println!("{}", html);
  Ok(())
}

fn render_markdown() -> Result<String, MyError> {
  let file = std::env::var("MARKDOWN")?;
  let source = read_to_string(file)?;
  Ok(markdown::to_html(&source))
}

#[derive(thiserror::Error, Debug)]
pub enum MyError {
  #[error("Environment variable not found")]
  EnvironmentVariableNotFound(#[from] std::env::VarError),
  #[error(transparent)]
  IOError(#[from] std::io::Error),
}

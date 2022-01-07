pub fn anyhow_example() -> anyhow::Result<()> {
  let html = render_markdown()?;
  println!("{}", html);
  Ok(())
}

fn render_markdown() -> anyhow::Result<String> {
  let file = std::env::var("MARKDOWN")?;
  let source = std::fs::read_to_string(file)?;
  Ok(markdown::to_html(&source))
}

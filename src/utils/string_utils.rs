pub fn lower_case(s: &String) -> String {
  let mut lower = String::new();
  for c in s.to_ascii().iter() {
    lower.push(c.to_lowercase().to_char());
  }
  lower
}

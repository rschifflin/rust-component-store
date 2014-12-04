pub fn snake_case(s: &String) -> String {
  let mut snake = s.to_ascii()
    .iter()
    .fold(String::new(), |acc, letter| -> String {
      if letter.is_uppercase() { append_upper(acc, letter.to_lowercase().as_char()) }
      else if [' ', '-', '_'].contains(&letter.as_char()) { append_seperator(acc) }
      else { append_other(acc, letter.to_lowercase().as_char()) }
    });

  match snake.to_ascii().last() {
    Some(last) if last == &'_'.to_ascii() => {
      snake.pop();
      snake
    }
    _ => snake
  }
}

fn append_upper(mut prev: String, tail: char) -> String {
  match prev.to_ascii().last() {
    None => {
      prev.push(tail);
      prev
    }
    Some(last) if last == &'_'.to_ascii() => {
      prev.push(tail);
      prev
    }
    _ => {
      prev.push('_');
      prev.push(tail);
      prev
    }
  }
}

fn append_seperator(mut prev: String) -> String {
  match prev.to_ascii().last() {
    None => prev,
    Some(&last) if last == '_'.to_ascii() => prev,
    _ => {
      prev.push('_');
      prev
    }
  }
}

fn append_other(mut prev: String, tail: char) -> String {
  prev.push(tail);
  prev
}

#[test]
fn test_snake_on_empty() {
  let original_string = "".to_string();
  assert_eq!(snake_case(&original_string), original_string);
}

#[test]
fn test_snake_on_single_upper() {
  let original_string = "A".to_string();
  let expected_string = "a".to_string();
  assert_eq!(snake_case(&original_string), expected_string);
}

#[test]
fn test_snake_on_single_lower() {
  let original_string = "a".to_string();
  assert_eq!(snake_case(&original_string), original_string);
}

#[test]
fn test_snake_on_single_upper_word() {
  let original_string = "Abc".to_string();
  let expected_string = "abc".to_string();
  assert_eq!(snake_case(&original_string), expected_string);
}

#[test]
fn test_snake_on_single_lower_word() {
  let original_string = "abc".to_string();
  assert_eq!(snake_case(&original_string), original_string);
}

#[test]
fn test_snake_on_multi_upper_word() {
  let original_string = "AbcDefGhi".to_string();
  let expected_string = "abc_def_ghi".to_string();
  assert_eq!(snake_case(&original_string), expected_string);
}

#[test]
fn test_snake_on_multi_upper_word_with_seperators() {
  let original_string = "Abc  Def_Ghi-jkl".to_string();
  let expected_string = "abc_def_ghi_jkl".to_string();
  assert_eq!(snake_case(&original_string), expected_string);
}

#[test]
fn test_snake_on_leading_space() {
  let original_string = "  _ Abc".to_string();
  let expected_string = "abc".to_string();
  assert_eq!(snake_case(&original_string), expected_string);
}

#[test]
fn test_snake_on_trailing_space() {
  let original_string = "bc d  ".to_string();
  let expected_string = "bc_d".to_string();
  assert_eq!(snake_case(&original_string), expected_string);
}

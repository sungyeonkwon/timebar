use crate::error::TimebarError;
use crate::helpers::{get_filled_empty, string_to_u32};

#[test]
fn handles_negative_integer() {
  let negative_input = String::from("-30");
  let result = string_to_u32(&negative_input);
  let expected = Err(TimebarError::InvalidInteger);

  assert_eq!(expected, result);
}

#[test]
fn keeps_bar_length() {
  let test_p_set = [10.0, 10.3, 10.5, 10.7, 20.0, 50.0];
  let mut results = Vec::new();

  for p in test_p_set.iter() {
    let (filled, empty) = get_filled_empty(p);
    let total = filled + empty;
    results.push(total);
  }

  for total in results.iter() {
    assert_eq!(total, &results[0]);
  }
}

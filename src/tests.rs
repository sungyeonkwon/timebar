use crate::error::TimebarError;

#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
fn handles_negative_integer() {
  let negative_input = String::from("-30");
  let result = crate::helpers::string_to_u32(&negative_input);
  let expected = Err(TimebarError::InvalidInteger);

  assert_eq!(expected, result);
}

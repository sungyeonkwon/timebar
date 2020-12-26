#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic(expected = "Must provide a positive integer")]
fn handles_negative_integer() {
  let negative_input = String::from("-30");
  crate::bar::life::string_to_u32(&negative_input);
}

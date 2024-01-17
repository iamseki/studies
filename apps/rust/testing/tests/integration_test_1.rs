use _rust_testing::external;

#[test]
fn it_adds_two() {
  assert_eq!(4, external::add_two_from_lib(2));
}
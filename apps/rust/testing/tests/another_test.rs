use _rust_testing::external;

#[test]
fn another_it_adds_two() {
  assert_eq!(7, external::add_two_from_lib(5));
}
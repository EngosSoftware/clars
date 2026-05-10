use clars::get_app_and_args;

#[test]
fn get_args_should_work() {
  assert!(get_app_and_args().0.starts_with("test_helpers-"));
}

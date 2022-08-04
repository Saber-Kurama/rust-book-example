use assert_cmd::Command;


#[test]
fn works() {
  assert!(true)
}

#[test]
fn runs1() {
  let mut cmd = Command::cargo_bin("hello").unwrap();
  cmd.assert().success();
}

#[test]
fn runs() {
  let mut cmd = Command::cargo_bin("hello").unwrap();
  cmd.assert().success().stdout("Hello, world!\n");
}


// #[test]
// fn flase_not_ok() {
//   let mut cmd = Command::cargo_bin("false").unwrap();
//   cmd.assert().failure();
// }
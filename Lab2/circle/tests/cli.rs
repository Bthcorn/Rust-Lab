use assert_cmd::Command;


#[test]
fn run_without_arguments() {
    let mut cmd = Command::cargo_bin("cir_circle").unwrap();
    cmd.arg("").assert().success().stdout("the circumference of a circle with radius 0 is 0\n");
}


#[test]
fn run_with_one_argument() {
    let mut cmd = Command::cargo_bin("cir_circle").unwrap();
    cmd.arg("42").assert().success().stdout("the circumference of a circle with radius 42 is 263.89378290154264\n");
}
use assert_cmd::Command;
use float_cmp::approx_eq;
use exercises::cal_slope;

#[test]
fn test_slope() {
    let mut cmd = Command::cargo_bin("cal_slope").unwrap();
    cmd.args(&["0", "0", "1", "1"])
        .assert()
        .success()
        .stdout("The slope of the line is 1.\n");
}


#[test]
fn test_triangle() {
    let mut cmd = Command::cargo_bin("triangle").unwrap();
    cmd.args(&["5"])
        .assert()
        .success()
        .stdout("*\n**\n***\n****\n*****\n");
}

#[test]
fn test_triangle_0() {
    let mut cmd = Command::cargo_bin("triangle").unwrap();
    cmd.args(&["0"])
        .assert()
        .success()
        .stdout("");
}

#[test]
fn test_float() {
    let slope = exercises::cal_slope(0., 0., 1., 1.);
    assert!(approx_eq!(f64, 1.0, slope, epsilon = 0.0001));
}
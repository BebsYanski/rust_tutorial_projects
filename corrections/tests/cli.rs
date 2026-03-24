use std::process::Command;
#[test]
fn works() {
    assert!(true);
}

#[test]
fn work2() {
    assert!(true)
}

#[test]
fn runs() {
    let mut cmd = if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.args(["/C", "dir"]);
        c
    } else {
        let mut c = Command::new("ls");
        c.arg("-la");
        c
    };

    let res = cmd.output();

    // Debug-print the result so failures show useful info.
    println!("{:?}", res.as_ref().ok());

    assert!(res.is_ok())
}

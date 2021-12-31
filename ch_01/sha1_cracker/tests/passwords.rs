use galvanic_test::test_suite;

test_suite! {
    name sha_cracker_trace_test;
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;
    test password_exists_in_list_1() {
        let mut cmd = Command::cargo_bin("sha1_cracker").unwrap();
        cmd.arg("./wordlist.txt");
        cmd.arg("a4ff81efb344996b21e9dc8229c5eb01bb226700");
        cmd.assert().success();
        cmd.assert().stdout(predicate::str::contains("Password found: "));
    }
    test password_exists_in_list_2() {
        let mut cmd = Command::cargo_bin("sha1_cracker").unwrap();
        cmd.arg("./wordlist.txt");
        cmd.arg("ec461b5480380ecf863d9802edbe70152aee1c46");
        cmd.assert().success();
        cmd.assert().stdout(predicate::str::contains("Password found: "));
    }
    test password_exists_in_list_3() {
        let mut cmd = Command::cargo_bin("sha1_cracker").unwrap();
        cmd.arg("./wordlist.txt");
        cmd.arg("ef89a3a842b0384565a210f0122804f411fe51fb");
        cmd.assert().success();
        cmd.assert().stdout(predicate::str::contains("Password found: "));
    }
    test password_does_not_exist1() {
        let mut cmd = Command::cargo_bin("sha1_cracker").unwrap();
        cmd.arg("./wordlist.txt");
        cmd.arg("a4ff81efb334996b21e9dc8229c5eb01bb226701");
        cmd.assert().success();
        cmd.assert().stdout(predicate::str::contains("password not found in wordlist :("));
    }
    test password_does_not_exist2() {
        let mut cmd = Command::cargo_bin("sha1_cracker").unwrap();
        cmd.arg("./wordlist.txt");
        cmd.arg("ef89a3a842b0384565a210f0122804f411fe51fa");
        cmd.assert().success();
        cmd.assert().stdout(predicate::str::contains("password not found in wordlist :("));
    }
    test password_does_not_exist3() {
        let mut cmd = Command::cargo_bin("sha1_cracker").unwrap();
        cmd.arg("./wordlist.txt");
        cmd.arg("ec461b5480380ecf863d9802edbe70152aee1c47");
        cmd.assert().success();
        cmd.assert().stdout(predicate::str::contains("password not found in wordlist :("));
    }
}

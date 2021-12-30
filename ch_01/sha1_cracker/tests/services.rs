use galvanic_test::test_suite;

test_suite! {
    name sha_cracker_trace_test;
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;
    test sha_cracker_trace_test1() {
        let mut cmd = Command::cargo_bin("sha1_cracker").unwrap();
        cmd.arg("./wordlist.txt");
        cmd.arg("a4ff81efb344996b21e9dc8229c5eb01bb226700");
        cmd.assert().success();
        cmd.assert().stdout(predicate::str::contains("longhorn"));
    }
}

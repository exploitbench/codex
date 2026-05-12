#![cfg(not(target_os = "windows"))]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use core_test_support::responses;
use core_test_support::test_codex_exec::test_codex_exec;
use predicates::str::contains;
use pretty_assertions::assert_eq;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn exec_max_turns_stops_before_follow_up_model_request() -> anyhow::Result<()> {
    let test = test_codex_exec();
    let server = responses::start_mock_server().await;
    let body = responses::sse(vec![
        responses::ev_response_created("resp1"),
        responses::ev_shell_command_call("call1", "echo hi"),
        responses::ev_completed("resp1"),
    ]);
    let response_mock = responses::mount_sse_once(&server, body).await;

    test.cmd_with_server(&server)
        .arg("--skip-git-repo-check")
        .arg("--max-turns")
        .arg("1")
        .arg("-m")
        .arg("gpt-5.1")
        .arg("run a command")
        .assert()
        .code(1)
        .stderr(contains("Maximum turn limit reached (1)."));

    assert_eq!(response_mock.requests().len(), 1);

    Ok(())
}

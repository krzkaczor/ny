use common::agent::Agent;
use owo_colors::OwoColorize;
use tempdir::TempDir;

mod shared;

#[test]
fn test_help_works() {
    let tmp_dir = TempDir::new("help_test").unwrap();
    let cwd = tmp_dir.into_path();

    let output = shared::bash(&cwd, "ny --help");
    assert!(output.contains("Usage: ny [COMMAND]"));
}

#[test]
fn test_help_add_help_works() {
    let tmp_dir = TempDir::new("help_test").unwrap();
    let cwd = tmp_dir.into_path();

    let output = shared::bash(&cwd, "ny add --help");
    assert!(output.contains("Usage: ny add [OPTIONS] <PACKAGES>..."));
}

#[test]
fn test_help_run_echo_cli_help_prints_echo_help() {
    for agent in Agent::all() {
        println!(
            "Testing {}",
            agent.as_str().bg::<owo_colors::colors::BrightRed>()
        );
        let tmp_dir = TempDir::new(agent.as_str()).unwrap();
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());
        shared::bash(&cwd, shared::agent_to_init_command(agent).as_str());
        shared::bash(&cwd, "ny add echo-cli");

        let output = shared::bash(&cwd, "ny run echo-cli --help");
        assert!(output.contains("Outputs the passed text to the command line."));
    }
}

#[test]
fn test_help_run_echo_cli_some_help_passes_all_args() {
    for agent in Agent::all() {
        println!(
            "Testing {}",
            agent.as_str().bg::<owo_colors::colors::BrightRed>()
        );
        let tmp_dir = TempDir::new(agent.as_str()).unwrap();
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());
        shared::bash(&cwd, shared::agent_to_init_command(agent).as_str());
        shared::bash(&cwd, "ny add echo-cli");

        let output = shared::bash(&cwd, "ny run echo-cli some --help");
        assert!(output.contains("some --help"));
    }
}

#[test]
fn test_help_echo_cli_help_prints_echo_help() {
    for agent in Agent::all() {
        println!(
            "Testing {}",
            agent.as_str().bg::<owo_colors::colors::BrightRed>()
        );
        let tmp_dir = TempDir::new(agent.as_str()).unwrap();
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());
        shared::bash(&cwd, shared::agent_to_init_command(agent).as_str());
        shared::bash(&cwd, "ny add echo-cli");

        let output = shared::bash(&cwd, "ny echo-cli --help");
        assert!(output.contains("Outputs the passed text to the command line."));
    }
}

#[test]
fn test_help_echo_cli_some_help_passes_all_args() {
    for agent in Agent::all() {
        println!(
            "Testing {}",
            agent.as_str().bg::<owo_colors::colors::BrightRed>()
        );
        let tmp_dir = TempDir::new(agent.as_str()).unwrap();
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());
        shared::bash(&cwd, shared::agent_to_init_command(agent).as_str());
        shared::bash(&cwd, "ny add echo-cli");

        let output = shared::bash(&cwd, "ny echo-cli some --help");
        assert!(output.contains("some --help"));
    }
}

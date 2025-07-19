use common::agent::Agent;
use owo_colors::OwoColorize;
use std::io;
use tempdir::TempDir;

mod shared;

#[test]
fn test_add_pure_js_dependency() -> Result<(), io::Error> {
    for agent in Agent::all() {
        println!(
            "Testing {}",
            agent.as_str().bg::<owo_colors::colors::BrightRed>()
        );
        let tmp_dir = TempDir::new(agent.as_str())?;
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());

        shared::bash(&cwd, shared::agent_to_init_command(agent).as_str());
        let stdout = shared::bash(&cwd, "ny add repeat-string");

        shared::assert_package_json_dependency(&cwd, "repeat-string", false);
        // assert that types are not installed because tsconfig is missing
        // bun initializes typescript enabled project always so actually ny installed missing @types as well
        if agent.ne(&Agent::Bun) {
            assert!(!stdout.contains("Installing missing types"))
        }
    }

    Ok(())
}

#[test]
fn test_add_typescript_dependency() -> Result<(), io::Error> {
    for agent in Agent::all() {
        println!(
            "Testing {}",
            agent.as_str().bg::<owo_colors::colors::BrightRed>()
        );
        let tmp_dir = TempDir::new(agent.as_str())?;
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());
        shared::bash(&cwd, shared::agent_to_init_command(agent).as_str());
        shared::bash(&cwd, "ny add typescript");
        shared::bash(&cwd, "ny run tsc --init");

        shared::bash(&cwd, "ny add repeat-string");

        shared::assert_package_json_dependency(&cwd, "repeat-string", false);
        shared::assert_package_json_dependency(&cwd, "@types/repeat-string", true);
    }

    Ok(())
}

use std::io;
use std::path::{ PathBuf};
use std::process::Command;
use owo_colors::OwoColorize;
use serde_json::{Map, Value};
use tempdir::TempDir;
use common::agent::Agent;

#[test]
fn test_create_repo() -> Result<(), io::Error> {
    for agent in Agent::all() {
        println!("Testing {}", agent.as_str().bg::<owo_colors::colors::BrightRed>());
        let tmp_dir = TempDir::new(agent.as_str())?;
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());

        bash(&cwd, agent_to_init_command(agent).as_str());
        bash(&cwd, "ny add repeat-string");

        assert_package_json_dependency(&cwd, "repeat-string");
    }

    Ok(())
}

#[test]
fn test_execute_shell_script() -> Result<(), io::Error> {
    for agent in Agent::all() {
        let tmp_dir = TempDir::new("npm")?;
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());

        bash(&cwd, agent_to_init_command(&agent).as_str());

        insert_npm_scripts(&cwd, &[
            ("shell", "echo 'a-was-run' && echo 'another-too'"),
            ("shell2", "cd /tmp && pwd"),
        ]);

        let shell_output = bash(&cwd, "ny run shell");
        assert!(shell_output.contains("a-was-run"));
        assert!(shell_output.contains("another-too"));
    }

    Ok(())
}

#[test]
fn test_execute_shell_script2() -> Result<(), io::Error> {
    for agent in Agent::all() {
        let tmp_dir = TempDir::new("npm")?;
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());

        bash(&cwd, agent_to_init_command(agent).as_str());

        insert_npm_scripts(&cwd, &[
            ("shell", "cd /tmp && pwd"),
        ]);

        let shell_output = bash(&cwd, "ny run shell");
        assert!(shell_output.contains("/tmp"));
    }

    Ok(())
}

#[test]
fn test_execute_npm_script() -> Result<(), io::Error> {
    for agent in Agent::all() {
        let tmp_dir = TempDir::new("npm")?;
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());

        bash(&cwd, agent_to_init_command(agent).as_str());
        bash(&cwd, "ny add echo-cli"); // required dep by some scripts

        insert_npm_scripts(&cwd, &[
            ("npm-simple", "echo-cli 'Hello from npm-simple'"),
            ("npm-simple2", "echo-cli 'lorem ipsum'"),
            ("npm-recursive", "npm run npm-simple && npm run npm-simple2"),
        ]);

        let npm_simple_output = bash(&cwd, "ny run npm-simple");
        assert!(npm_simple_output.contains("Hello from npm-simple"));

        let npm_recursive_output = bash(&cwd, "ny run npm-recursive");
        assert!(npm_recursive_output.contains("Hello from npm-simple"));
        assert!(npm_recursive_output.contains("lorem ipsum"));
    }

    Ok(())
}


fn agent_to_init_command(agent: &Agent) -> String {
    let package = "is-number";
    // initializes repo and adds a dummy package so lockfile gets created
    match agent {
        Agent::Npm => format!("npm init -y && npm install {package}"),
        Agent::Yarn => format!("yarn init -y && yarn add {package}"),
        Agent::Pnpm => format!("pnpm init && pnpm add {package}"),
        Agent::Bun => format!("bun init -y && bun add {package}"),
    }
}

fn bash(cwd: &PathBuf, cmd: &str) ->String {
    println!("> {cmd}");
    let debug_path = get_debug_dir();
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .env("PATH", format!("{}:{}", debug_path.display(), std::env::var("PATH").unwrap_or_default()))
        .current_dir(cwd)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        assert!(false, "bash command did not exit successfully: {}", String::from_utf8_lossy(&output.stderr).to_string());
    }
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    println!("{}", stdout);

    stdout
}

fn get_debug_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .join("target")
        .join("debug")
}

fn assert_package_json_dependency(cwd: &PathBuf, expected_dep:&str) {
    let path = cwd.join("package.json");
    let manifest_raw = std::fs::read_to_string(path).unwrap();
    let manifest: serde_json::Value = serde_json::from_str(&manifest_raw).unwrap();

    let dep = manifest.get("dependencies")
        .and_then(|deps| deps.get(expected_dep));

    assert!(dep.is_some(), "missing dependency '{}'", expected_dep);
}


fn insert_npm_scripts(cwd: &PathBuf, scripts: &[(&str, &str)]) {
    let path = cwd.join("package.json");
    let manifest_raw = std::fs::read_to_string(&path).unwrap();
    let mut manifest: serde_json::Value = serde_json::from_str(&manifest_raw).unwrap();

    if !manifest.get("scripts").map(|v| v.is_object()).unwrap_or(false) {
        manifest["scripts"] = Value::Object(Map::new());
    }

    let scripts_entry = manifest.get_mut("scripts").unwrap().as_object_mut().unwrap();


    for (key, value) in scripts {
        scripts_entry.insert(key.to_string(), Value::String(value.to_string()));
    }

    let new_manifest = serde_json::to_string_pretty(&manifest).unwrap();
    std::fs::write(path, new_manifest).unwrap();
}
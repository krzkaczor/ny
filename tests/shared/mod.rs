use common::agent::Agent;
use serde_json::{Map, Value};
use std::path::PathBuf;
use std::process::Command;

#[allow(dead_code)]
pub fn agent_to_init_command(agent: &Agent) -> String {
    let package = "is-number";
    // initializes repo and adds a dummy package so lockfile gets created
    match agent {
        Agent::Npm => format!("npm init -y && npm install {package}"),
        Agent::Yarn => format!("yarn init -y && yarn add {package}"),
        Agent::Pnpm => format!("pnpm init && pnpm add {package}"),
        Agent::Bun => format!("bun init -y && bun add {package}"),
    }
}

#[allow(dead_code)]
pub fn bash(cwd: &PathBuf, cmd: &str) -> String {
    println!("> {cmd}");
    let debug_path = get_debug_dir();
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .env(
            "PATH",
            format!(
                "{}:{}",
                debug_path.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .current_dir(cwd)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        assert!(
            false,
            "bash command did not exit successfully: {}",
            String::from_utf8_lossy(&output.stderr).to_string()
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    println!("{}", stdout);

    stdout
}

#[allow(dead_code)]
pub fn get_debug_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("target").join("debug")
}

#[allow(dead_code)]
pub fn assert_package_json_dependency(cwd: &PathBuf, expected_dep: &str, dev: bool) {
    let path = cwd.join("package.json");
    let manifest_raw = std::fs::read_to_string(path).unwrap();
    let manifest: serde_json::Value = serde_json::from_str(&manifest_raw).unwrap();

    let dep = manifest
        .get(if !dev {
            "dependencies"
        } else {
            "devDependencies"
        })
        .and_then(|deps| deps.get(expected_dep));

    assert!(dep.is_some(), "missing dependency '{}'", expected_dep);
}

#[allow(dead_code)]
pub fn insert_npm_scripts(cwd: &PathBuf, scripts: &[(&str, &str)]) {
    let path = cwd.join("package.json");
    let manifest_raw = std::fs::read_to_string(&path).unwrap();
    let mut manifest: serde_json::Value = serde_json::from_str(&manifest_raw).unwrap();

    if !manifest
        .get("scripts")
        .map(|v| v.is_object())
        .unwrap_or(false)
    {
        manifest["scripts"] = Value::Object(Map::new());
    }

    let scripts_entry = manifest
        .get_mut("scripts")
        .unwrap()
        .as_object_mut()
        .unwrap();

    for (key, value) in scripts {
        scripts_entry.insert(key.to_string(), Value::String(value.to_string()));
    }

    let new_manifest = serde_json::to_string_pretty(&manifest).unwrap();
    std::fs::write(path, new_manifest).unwrap();
}

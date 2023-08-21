use colored::Colorize;
use eyre::{ContextCompat, Result};
use std::path::Path;

use crate::{execute::Executor, fs::find_in_parents, fs::Filesystem};

pub fn run(
    executor: &dyn Executor,
    fs: &dyn Filesystem,
    task: &str,
    cwd: &Path,
    extra_args: Option<&[&str]>,
) -> Result<()> {
    let package_json_path = find_in_parents(fs, cwd, "package.json").with_context(|| {
        format!("Couldn't find package.json in the current directory: {cwd:?} or its parents.")
    })?;
    let package_json = load_package_json(fs, &package_json_path)
        .with_context(|| format!("Couldn't parse package.json: {package_json_path:?}"))?;

    let bin_path = construct_path_env(package_json_path.parent().unwrap());

    if let Some(mut script) = load_script(&package_json, task) {
        if let Some(extra_args) = extra_args {
            script += " ";
            script += &extra_args.join(" ")
        }

        println!("{}", format!("$ {}", script).dimmed());

        executor.execute(
            "sh",
            &["-c", &script],
            Some(bin_path),
            false, // do not print command as it's quite odd to see "sh -c <script>"
            false, // do not silence output
        )
    } else {
        let program = task;
        println!("{}", format!("$ {}", program).dimmed());

        executor.execute(
            program,
            extra_args.unwrap_or_default(),
            Some(bin_path),
            false, // do not print command as it's quite odd to see "sh -c <script>"
            false, // do not silence output
        )
    }
}

fn load_package_json(fs: &dyn Filesystem, path: &Path) -> Option<serde_json::Value> {
    fs.read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
}

fn load_script(package_json: &serde_json::Value, script_name: &str) -> Option<String> {
    package_json
        .get("scripts")
        .and_then(|scripts| scripts.get(script_name))
        .and_then(|script| script.as_str())
        .map(|s| s.to_string())
}

fn construct_path_env(dir: &Path) -> String {
    // @note: we avoid for checking if <dir>/node_modules/.bin even exists because OS will just handle such cases automatically for us.
    let mut path_env = String::new();

    let mut current_dir = dir;
    loop {
        path_env += current_dir.join("node_modules/.bin").to_str().unwrap();
        path_env += ":";

        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            break;
        }
    }

    path_env
}

#[cfg(test)]
mod tests {
    use crate::execute::{expect_execute_once, MockExecutor};
    use crate::utils::vec_of_strings;

    use crate::fs::{test_utils::*, MockFilesystem};

    use super::*;

    #[test]
    fn test_load_script() {
        let package_json = serde_json::json!({
            "scripts": {
                "test": "echo \"test\""
            }
        });

        let result = load_script(&package_json, "test");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "echo \"test\"");

        let result = load_script(&package_json, "test2");
        assert!(result.is_none());
    }

    #[test]
    fn test_construct_path_env() {
        assert_eq!(
            construct_path_env(Path::new("/project/nested")),
            "/project/nested/node_modules/.bin:/project/node_modules/.bin:/node_modules/.bin:"
        );
    }

    #[test]
    fn command_run_from_package_json() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "sh",
            vec_of_strings!("-c", r#"mocha "*.ts" --no-timeout --bail"#),
            Some("/project/node_modules/.bin:/node_modules/.bin:".to_string()),
            false,
            false,
        );
        let mut mock_fs = MockFilesystem::new();
        expect_file(
            &mut mock_fs,
            Path::new("/project/package.json").to_owned(),
            r#"{"scripts": {"test": "mocha \"*.ts\""}}"#.to_owned(),
        );

        let result = run(
            &mock_executor,
            &mock_fs,
            "test",
            Path::new("/project"),
            Some(&["--no-timeout", "--bail"]),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn command_run_from_node_modules() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "mocha",
            vec_of_strings!("--help"),
            Some("/project/node_modules/.bin:/node_modules/.bin:".to_string()),
            false,
            false,
        );
        let mut mock_fs = MockFilesystem::new();
        expect_file(
            &mut mock_fs,
            Path::new("/project/package.json").to_owned(),
            r#"{"scripts": {}}"#.to_owned(),
        );

        let result = run(
            &mock_executor,
            &mock_fs,
            "mocha",
            Path::new("/project"),
            Some(&["--help"]),
        );

        assert!(result.is_ok());
    }
}

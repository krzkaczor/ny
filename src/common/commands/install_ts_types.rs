use colored::Colorize;
use eyre::{Context, ContextCompat, Result};
use std::path::Path;

use crate::agent::Agent;
use crate::execute::Executor;
use crate::fs::{find_in_parents, Filesystem};
use crate::http::HttpClient;

use super::add;

pub fn check_if_ts_repo(fs: &dyn Filesystem, cwd: &Path) -> bool {
    find_in_parents(fs, cwd, "tsconfig.json").is_some()
}

pub fn install_ts_types(
    executor: &dyn Executor,
    fs: &dyn Filesystem,
    http_client: &dyn HttpClient,
    agent: &Agent,
    cwd: &Path,
    packages: &[String],
    workspace_root: bool,
) -> Result<()> {
    let packages_missing_types = packages
        .iter()
        .filter(|package| !package.starts_with("@types/"))
        .filter(|package| !check_if_package_has_types(fs, cwd, package).unwrap_or_default())
        .collect::<Vec<_>>();

    let packages_to_install = packages_missing_types
        .iter()
        .map(|package| package_name_to_types_package_name(package.to_string()))
        .filter(|package| !check_if_package_exists_locally(fs, cwd, package).unwrap_or_default())
        .filter(|package| {
            check_if_package_exists_in_registry(http_client, package).unwrap_or_default()
        })
        .collect::<Vec<_>>();

    if !packages_to_install.is_empty() {
        println!();
        if packages_to_install.len() == 1 {
            println!(
                "Installing missing types: {}",
                packages_to_install.join(", ").dimmed()
            );
        } else {
            println!(
                "Installing missing types for {} packages: {}",
                packages_to_install.len(),
                packages_to_install.join(", ").dimmed()
            );
        }

        add::add(
            executor,
            agent,
            true,
            workspace_root,
            &packages_to_install,
            true, // silence output of PM's add command. The command itself is still printed.
        )?;
    }

    Ok(())
}

fn package_name_to_types_package_name(mut package: String) -> String {
    // handle scoped packages
    if package.starts_with('@') {
        package = package.trim_start_matches('@').to_string();
        package = package.replace('/', "__")
    }
    // cut off version
    package = package.split('@').collect::<Vec<_>>()[0].to_string();

    format!("@types/{}", package)
}

fn check_if_package_has_types(fs: &dyn Filesystem, cwd: &Path, package: &str) -> Result<bool> {
    let package_json_sub_path = Path::new("node_modules").join(package).join("package.json");
    let package_json_path = find_in_parents(fs, cwd, package_json_sub_path.to_str().unwrap())
        .context("can't find package root")?;
    let package_json = fs
        .read_to_string(package_json_path.as_path())
        .context("can't read package.json")?;
    let package_json: serde_json::Value = serde_json::from_str(&package_json).unwrap();
    let types = package_json.get("types");
    let typings = package_json.get("typings");

    Ok(types.is_some() || typings.is_some())
}

fn check_if_package_exists_locally(fs: &dyn Filesystem, cwd: &Path, package: &str) -> Result<bool> {
    let package_json_sub_path = Path::new("node_modules").join(package).join("package.json");
    find_in_parents(fs, cwd, package_json_sub_path.to_str().unwrap())
        .context("can't find package root")?;

    Ok(true)
}

// @todo: this should be parallelized
fn check_if_package_exists_in_registry(
    http_client: &dyn HttpClient,
    package: &str,
) -> Result<bool> {
    let url = format!("https://registry.npmjs.org/{}", package);
    http_client.request_if_success(&url)
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::{
        execute::{expect_execute_once, MockExecutor},
        fs::{test_utils::expect_file, MockFilesystem},
        http::{test_utils::expect_package_exist_in_registry, MockHttpClient},
        utils::vec_of_strings,
    };

    use super::*;

    #[test]
    fn test_package_name_to_types_package_name() {
        assert_eq!(
            package_name_to_types_package_name("foo".to_string()),
            "@types/foo".to_owned()
        );
        assert_eq!(
            package_name_to_types_package_name("foo@tag".to_string()),
            "@types/foo".to_owned()
        );
        assert_eq!(
            package_name_to_types_package_name("@foo/bar".to_string()),
            "@types/foo__bar".to_owned()
        );
    }

    #[test]
    fn test_check_if_ts_repo() {
        let mut mock_fs = MockFilesystem::new();
        expect_file(
            &mut mock_fs,
            Path::new("/project/tsconfig.json").to_owned(),
            "".to_owned(),
        );

        assert_eq!(check_if_ts_repo(&mock_fs, Path::new("/project")), true);
    }

    #[test]
    fn test_check_if_not_ts_repo() {
        let mut mock_fs = MockFilesystem::new();
        mock_fs
            .expect_exists()
            .with(eq(Path::new("/project/tsconfig.json").to_owned()))
            .returning(|_| false);
        mock_fs
            .expect_exists()
            .with(eq(Path::new("/tsconfig.json").to_owned()))
            .returning(|_| false);

        assert_eq!(check_if_ts_repo(&mock_fs, Path::new("/project")), false);
    }

    #[test]
    fn test_check_if_package_has_types() {
        let mut mock_fs = MockFilesystem::new();
        expect_file(
            &mut mock_fs,
            Path::new("/project/node_modules/package/package.json").to_owned(),
            r#"{"types": "index.d.ts"}"#.to_owned(),
        );

        assert_eq!(
            check_if_package_has_types(&mock_fs, Path::new("/project"), "package")
                .unwrap_or_default(),
            true
        );
    }
    #[test]
    fn test_check_if_package_has_types_with_typings_key() {
        let mut mock_fs = MockFilesystem::new();
        expect_file(
            &mut mock_fs,
            Path::new("/project/node_modules/package/package.json").to_owned(),
            r#"{"typings": "index.d.ts"}"#.to_owned(),
        );

        assert_eq!(
            check_if_package_has_types(&mock_fs, Path::new("/project"), "package")
                .unwrap_or_default(),
            true
        );
    }

    #[test]
    fn test_check_if_package_has_no_types() {
        let mut mock_fs = MockFilesystem::new();
        expect_file(
            &mut mock_fs,
            Path::new("/project/node_modules/package/package.json").to_owned(),
            r#"{}"#.to_owned(),
        );

        assert_eq!(
            check_if_package_has_types(&mock_fs, Path::new("/project"), "package")
                .unwrap_or_default(),
            false
        );
    }

    #[test]
    fn test_integration_install_ts_types() {
        let mut mock_fs = MockFilesystem::new();
        //package-a without types
        expect_file(
            &mut mock_fs,
            Path::new("/project/node_modules/package-a/package.json").to_owned(),
            r#"{}"#.to_owned(),
        );
        // package-b with types
        expect_file(
            &mut mock_fs,
            Path::new("/project/node_modules/package-b/package.json").to_owned(),
            r#"{"types":"exists"}"#.to_owned(),
        );
        // package-c without built-in types but with @types as a separate package that were installed before
        expect_file(
            &mut mock_fs,
            Path::new("/project/node_modules/package-c/package.json").to_owned(),
            r#"{}"#.to_owned(),
        );
        expect_file(
            &mut mock_fs,
            Path::new("/project/node_modules/@types/package-c/package.json").to_owned(),
            r#"{}"#.to_owned(),
        );
        mock_fs.expect_exists().returning(|_| false);

        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "npm",
            vec_of_strings!["install", "--save-dev", "@types/package-a"],
            None,
            true,
            true,
        );

        let mut mock_http_client = MockHttpClient::new();
        expect_package_exist_in_registry(&mut mock_http_client, "@types/package-a", true);

        let agent = Agent::Npm;

        install_ts_types(
            &mock_executor,
            &mock_fs,
            &mock_http_client,
            &agent,
            Path::new("/project"),
            &vec_of_strings!["package-a", "package-b", "package-c"],
            false,
        )
        .unwrap();
    }

    #[test]
    fn test_integration_package_with_types_missing_in_registry() {
        let mut mock_fs = MockFilesystem::new();
        //package-a without types
        expect_file(
            &mut mock_fs,
            Path::new("/project/node_modules/package-a/package.json").to_owned(),
            r#"{}"#.to_owned(),
        );
        mock_fs.expect_exists().returning(|_| false);

        let mock_executor = MockExecutor::new();

        let mut mock_http_client = MockHttpClient::new();
        expect_package_exist_in_registry(&mut mock_http_client, "@types/package-a", false);

        let agent = Agent::Npm;

        install_ts_types(
            &mock_executor,
            &mock_fs,
            &mock_http_client,
            &agent,
            Path::new("/project"),
            &vec_of_strings!["package-a"],
            false,
        )
        .unwrap();
    }

    #[test]
    fn test_integration_install_ts_types_when_empty() {
        let mut mock_fs = MockFilesystem::new();
        // package-a with types
        expect_file(
            &mut mock_fs,
            Path::new("/project/node_modules/package-a/package.json").to_owned(),
            r#"{"types":"exists"}"#.to_owned(),
        );

        let mock_executor = MockExecutor::new();
        let mock_http_client = MockHttpClient::new();
        let agent = Agent::Npm;

        install_ts_types(
            &mock_executor,
            &mock_fs,
            &mock_http_client,
            &agent,
            Path::new("/project"),
            &vec_of_strings!["package-a"],
            false,
        )
        .unwrap();
    }
}

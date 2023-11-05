use crate::{agent::Agent, execute::Executor};
use eyre::{eyre, Result};

pub fn add(
    executor: &dyn Executor,
    agent: &Agent,
    dev: bool,
    workspace_root: bool,
    packages: &[String],
    silence_stdout: bool,
) -> Result<()> {
    let packages_refs: Vec<_> = packages.iter().map(|s| s.as_str()).collect();

    fn merge_and_clean_args<'a>(
        arg1: &'a str,
        arg2: Option<&'a str>,
        arg3: Option<&'a str>,
        mut var_args: Vec<&'a str>,
    ) -> Vec<&'a str> {
        let mut args = vec![arg1];
        if let Some(args2) = arg2 {
            args.push(args2);
        }
        if let Some(args3) = arg3 {
            args.push(args3);
        }

        args.append(&mut var_args);

        args
    }

    match agent {
        Agent::Npm => executor.execute(
            "npm",
            &merge_and_clean_args(
                "install",
                if dev { Some("--save-dev") } else { None },
                None, //npm doesn't require workspace_root flag
                packages_refs,
            ),
            None,
            true,
            silence_stdout,
        ),
        Agent::Yarn => executor.execute(
            "yarn",
            &merge_and_clean_args(
                "add",
                if dev { Some("-D") } else { None },
                if workspace_root { Some("-W") } else { None },
                packages_refs,
            ),
            None,
            true,
            silence_stdout,
        ),
        Agent::Pnpm => executor.execute(
            "pnpm",
            &merge_and_clean_args(
                "add",
                if dev { Some("-D") } else { None },
                if workspace_root { Some("-w") } else { None },
                packages_refs,
            ),
            None,
            true,
            silence_stdout,
        ),
        Agent::Bun => {
            if workspace_root {
                return Err(eyre!("Bun doesn't support workspace_root flag"));
            }

            executor.execute(
                "bun",
                &merge_and_clean_args(
                    "add",
                    if dev { Some("-D") } else { None },
                    None,
                    packages_refs,
                ),
                None,
                true,
                silence_stdout,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::execute::{expect_execute_once, MockExecutor};
    use crate::utils::vec_of_strings;

    use super::*;

    #[test]
    fn test_add_npm() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "npm",
            vec_of_strings!("install", "packageA", "packageB"),
            None,
            true,
            false,
        );

        let result = add(
            &mock_executor,
            &Agent::Npm,
            false,
            false,
            &vec_of_strings!["packageA", "packageB"],
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_npm_workspace() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "npm",
            vec_of_strings!("install", "packageA", "packageB"),
            None,
            true,
            false,
        );

        let result = add(
            &mock_executor,
            &Agent::Npm,
            false,
            true,
            &vec_of_strings!["packageA", "packageB"],
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_yarn() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "yarn",
            vec_of_strings!("add", "-D", "packageA", "packageB"),
            None,
            true,
            false,
        );

        let result = add(
            &mock_executor,
            &Agent::Yarn,
            true,
            false,
            &vec_of_strings!["packageA", "packageB"],
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_yarn_workspace() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "yarn",
            vec_of_strings!("add", "-D", "-W", "packageA", "packageB"),
            None,
            true,
            false,
        );

        let result = add(
            &mock_executor,
            &Agent::Yarn,
            true,
            true,
            &vec_of_strings!["packageA", "packageB"],
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_pnpm() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "pnpm",
            vec_of_strings!("add", "-D", "packageA", "packageB"),
            None,
            true,
            false,
        );

        let result = add(
            &mock_executor,
            &Agent::Pnpm,
            true,
            false,
            &vec_of_strings!["packageA", "packageB"],
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_pnpm_workspace() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "pnpm",
            vec_of_strings!("add", "-D", "-w", "packageA", "packageB"),
            None,
            true,
            false,
        );

        let result = add(
            &mock_executor,
            &Agent::Pnpm,
            true,
            true,
            &vec_of_strings!["packageA", "packageB"],
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_bun() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "bun",
            vec_of_strings!("add", "-D", "packageA", "packageB"),
            None,
            true,
            false,
        );

        let result = add(
            &mock_executor,
            &Agent::Bun,
            true,
            false,
            &vec_of_strings!["packageA", "packageB"],
            false,
        );

        assert!(result.is_ok());
    }
}

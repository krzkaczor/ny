use eyre::Result;

use crate::{agent::Agent, execute::Executor};

pub fn install(executor: &dyn Executor, agent: &Agent) -> Result<()> {
    match agent {
        Agent::Npm => executor.execute("npm", &["install"], None, true, false),
        Agent::Yarn => executor.execute("yarn", &["install"], None, true, false),
        Agent::Pnpm => executor.execute("pnpm", &["install"], None, true, false),
        Agent::Bun => executor.execute("bun", &["install"], None, true, false),
    }
}

#[cfg(test)]
mod tests {
    use crate::execute::{expect_execute_once, MockExecutor};
    use crate::utils::vec_of_strings;

    use super::*;

    #[test]
    fn test_install_npm() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "npm",
            vec_of_strings!("install"),
            None,
            true,
            false,
        );

        let result = install(&mock_executor, &Agent::Npm);

        assert!(result.is_ok());
    }

    #[test]
    fn test_install_yarn() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "yarn",
            vec_of_strings!("install"),
            None,
            true,
            false,
        );

        let result = install(&mock_executor, &Agent::Yarn);

        assert!(result.is_ok());
    }

    #[test]
    fn test_install_pnpm() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "pnpm",
            vec_of_strings!("install"),
            None,
            true,
            false,
        );

        let result = install(&mock_executor, &Agent::Pnpm);

        assert!(result.is_ok());
    }

    #[test]
    fn test_install_bun() {
        let mut mock_executor = MockExecutor::new();
        expect_execute_once(
            &mut mock_executor,
            "bun",
            vec_of_strings!("install"),
            None,
            true,
            false,
        );

        let result = install(&mock_executor, &Agent::Bun);

        assert!(result.is_ok());
    }
}

use colored::Colorize;
use eyre::{Context, Result};

use mockall::automock;

#[automock]
pub trait Executor {
    #[allow(clippy::needless_lifetimes)]
    fn execute<'a>(
        &self,
        program: &str,
        args: &[&'a str],
        extra_path: Option<String>,
        verbose: bool,        // print out the command being executed
        silence_stdout: bool, // silence "regular" output, still print out errors
    ) -> Result<()>;
}

pub struct RealExecutor {}
impl Executor for RealExecutor {
    fn execute(
        &self,
        program: &str,
        args: &[&str],
        extra_path: Option<String>,
        verbose: bool,
        silence_stdout: bool,
    ) -> Result<()> {
        if verbose {
            println!("{}", format!("$ {} {}", program, args.join(" ")).dimmed());
        }

        let mut cmd_builder_ref = std::process::Command::new(program);
        let cmd_builder = cmd_builder_ref.args(args);

        if let Some(extra_env) = extra_path {
            let current_path = std::env::var("PATH").unwrap_or_else(|_| "".to_string());
            cmd_builder.env("PATH", extra_env + ":" + &current_path);
        }
        if silence_stdout {
            cmd_builder.stdout(std::process::Stdio::null());
        }

        let mut proc = cmd_builder_ref
            .spawn()
            .with_context(|| format!("Couldn't run command: {program}"))?;
        let exit_status = proc
            .wait()
            .with_context(|| format!("Couldn't run command: {program}"))?;
        if !exit_status.success() {
            std::process::exit(exit_status.code().unwrap_or(1));
        }

        Ok(())
    }
}

#[cfg(test)]
// @todo can this be rewritten to not take ownership?
pub fn expect_execute_once(
    mock_executor: &mut MockExecutor,
    program: &str,
    args: Vec<String>,
    extra_path: Option<String>,
    verbose: bool,
    silence_stdout: bool,
) {
    let program = program.to_owned();
    mock_executor
        .expect_execute()
        .times(1)
        .withf(
            move |_program, _args, _extra_path, _verbose, _silence_stdout| {
                _program == program
                    && _args == args
                    && _extra_path == &extra_path
                    && _verbose == &verbose
                    && _silence_stdout == &silence_stdout
            },
        )
        .returning(|_, _, _, _, _| Ok(()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let executor = RealExecutor {};
        let result = executor.execute("sh", &["-c", "true"], None, false, false);
        assert!(result.is_ok());
    }
}

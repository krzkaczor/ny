use clap::{Parser, Subcommand};

#[derive(Parser, PartialEq, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

// @note: we do pre-parsing of CLI args in `parse_from` function down below
#[derive(Subcommand, PartialEq, Debug)]
pub enum Commands {
    /// Install dependencies
    #[command(alias("i"))]
    Install,
    /// Run a package.json task
    #[command(alias("r"))]
    Run {
        /// Name of a task from package.json to run
        #[arg(required = true)]
        task: String,

        /// Extra arguments to append to the task
        #[arg(required = false, trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },

    /// Add dependency
    #[command(alias("a"))]
    Add {
        /// Names of dependencies to add
        #[arg(required = true)]
        packages: Vec<String>,
        /// Install as dev dependency
        #[arg(
            short,
            alias = "save-dev",
            short_alias = 'D',
            long,
            default_value_t = false
        )]
        dev: bool,
        /// Add root workspace dependency
        #[arg(short, short_alias = 'W', long, default_value_t = false)]
        workspace_root: bool,
    },
}

// our cli is too complex to parse by clap alone so first we do a little bit of preprocessing
pub fn parse_from(mut args: Vec<String>) -> Cli {
    if args.len() > 1 {
        // if first arg is a task name
        if (args[1] != "install" && args[1] != "i")
            && (args[1] != "run" && args[1] != "r")
            && (args[1] != "add" && args[1] != "a")
            // and is not a flag
            && !args[1].starts_with('-')
        {
            // append "run" arg as the default
            args.insert(1, "run".to_string());
        }

        if args[1] == "run" || args[1] == "r" {
            // if there are only 4 args in total and --help at the end
            // ex. "ny run program --help"
            if args.len() == 4 && args.last() == Some(&"--help".to_string()) {
                // prepend it with -- so clap doesnt confuse it with --help flag for ny itself
                args.insert(args.len() - 1, "--".to_string());
            }
        }
    }
    Cli::parse_from(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::vec_of_strings;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn run_command_extra_args() {
        let parsed = parse_from(vec_of_strings!["/ny", "mocha", "--arg1", "--arg2", "abc"]);

        assert_eq!(
            parsed.command,
            Some(Commands::Run {
                task: "mocha".to_string(),
                extra_args: vec_of_strings!["--arg1", "--arg2", "abc"]
            })
        );
    }

    #[test]
    fn run_naked_command_help_arg() {
        let parsed = parse_from(vec_of_strings!["/ny", "mocha", "--help"]);

        assert_eq!(
            parsed.command,
            Some(Commands::Run {
                task: "mocha".to_string(),
                extra_args: vec_of_strings!["--help"]
            })
        );
    }

    #[test]
    fn run_naked_command_help_arg_manual_escape() {
        let parsed = parse_from(vec_of_strings!["/ny", "mocha", "--", "--help"]);

        assert_eq!(
            parsed.command,
            Some(Commands::Run {
                task: "mocha".to_string(),
                extra_args: vec_of_strings!["--", "--help"]
            })
        );
    }

    #[test]
    fn run_command_help_arg() {
        let parsed = parse_from(vec_of_strings!["/ny", "run", "mocha", "--help"]);

        assert_eq!(
            parsed.command,
            Some(Commands::Run {
                task: "mocha".to_string(),
                extra_args: vec_of_strings!["--help"]
            })
        );
    }

    #[test]
    fn run_command_help_arg_manual_escape() {
        let parsed = parse_from(vec_of_strings!["/ny", "run", "mocha", "--", "--help"]);

        assert_eq!(
            parsed.command,
            Some(Commands::Run {
                task: "mocha".to_string(),
                extra_args: vec_of_strings!["--", "--help"]
            })
        );
    }

    #[test]
    fn add_package_dev() {
        let parsed = parse_from(vec_of_strings!["/ny", "add", "--dev", "pkg"]);

        assert_eq!(
            parsed.command,
            Some(Commands::Add {
                packages: vec_of_strings!["pkg"],
                dev: true,
                workspace_root: false
            })
        );
    }

    #[test]
    fn add_package_dev_alias() {
        let parsed = parse_from(vec_of_strings!["/ny", "add", "--save-dev", "pkg"]);

        assert_eq!(
            parsed.command,
            Some(Commands::Add {
                packages: vec_of_strings!["pkg"],
                dev: true,
                workspace_root: false
            })
        );
    }
}

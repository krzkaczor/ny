use std::env;
extern crate common;
use eyre::{eyre, Result};

use common::{
    agent::Agent,
    cli::{parse_from, Commands},
    commands::{
        add::add,
        install::install,
        install_ts_types::{check_if_ts_repo, install_ts_types},
        run::run,
    },
    execute::RealExecutor,
    fs::RealFs,
    http::RealHttpClient,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let cli = parse_from(args);
    let cwd = env::current_dir()?;
    let executor = RealExecutor {};
    let fs = RealFs {};
    let http_client = RealHttpClient {};
    let agent = Agent::recognize(&fs, &cwd)
        .ok_or_else(|| eyre!("Couldn't find any lockfile inside {cwd:?} or any of its parents."))?;

    match cli.command {
        Some(Commands::Install) => install(&executor, &agent),
        Some(Commands::Run { task, extra_args }) => {
            let task = task.as_str();
            let extra_args: Vec<&str> = extra_args.iter().map(String::as_str).collect();
            run(&executor, &fs, &agent, task, &cwd, Some(&extra_args))
        }
        Some(Commands::Add {
            packages,
            dev,
            workspace_root,
        }) => {
            add(&executor, &agent, dev, workspace_root, &packages, false)?;
            if check_if_ts_repo(&fs, &cwd) {
                install_ts_types(
                    &executor,
                    &fs,
                    &http_client,
                    &agent,
                    &cwd,
                    &packages,
                    workspace_root,
                )
            } else {
                Ok(())
            }
        }
        None => install(&executor, &agent),
    }?;

    Ok(())
}

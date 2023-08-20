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
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let cli = parse_from(args);
    let cwd = env::current_dir().unwrap();
    let executor = RealExecutor {};
    let fs = RealFs {};
    // @todo this could be ommited for "run" command
    let agent = Agent::recognize(&fs, &cwd)
        .ok_or_else(|| eyre!("Couldn't find any lockfile inside {cwd:?} or any of its parents."))?;

    match cli.command {
        Some(Commands::Install) => install(&executor, &agent),
        Some(Commands::Run { task, extra_args }) => {
            let task = task.as_str();
            let extra_args: Vec<&str> = extra_args.iter().map(String::as_str).collect();
            run(&executor, &fs, task, &cwd, Some(&extra_args))
        }
        Some(Commands::Add {
            packages,
            dev,
            workspace_root,
        }) => {
            add(&executor, &agent, dev, workspace_root, &packages, true)?;
            if check_if_ts_repo(&fs, &cwd) {
                install_ts_types(&executor, &fs, &agent, &cwd, &packages, workspace_root)
            } else {
                Ok(())
            }
        }
        None => install(&executor, &agent),
    }?;

    Ok(())
}

// FIX TypeScript support
//   * @ngneat/falso <- it has types!
//   * "ny add @trpc/server @trpc/client"
// gracefully handler error while downloading typings
// @todo: push to brew repository
// improve error messages:
// * "ny test" when executed not in js repo returns: "Error: Couldn't find any lockfile inside "/Users/krzkaczor/Workspace/Personal/ny" or any of its parents.". Probably it should say that "test" was treated as task name.
// * "ny not-existing-task" returns:
//"Error: Couldn't run command: dupa
// Caused by:
// No such file or directory (os error 2)"
// should omit caused by part probably

// new features:
// * DID YOU MEAN for typod tasks?

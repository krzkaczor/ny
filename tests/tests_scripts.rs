use std::io;
use tempdir::TempDir;

mod shared;

#[test]
fn test_scripts_shell_script() -> Result<(), io::Error> {
    for agent in shared::all_agents() {
        let tmp_dir = TempDir::new("npm")?;
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());

        shared::bash(&cwd, shared::agent_to_init_command(&agent).as_str());

        shared::insert_npm_scripts(
            &cwd,
            &[
                ("shell", "echo 'a-was-run' && echo 'another-too'"),
                ("shell2", "cd /tmp && pwd"),
            ],
        );

        let shell_output = shared::bash(&cwd, "ny run shell");
        assert!(shell_output.contains("a-was-run"));
        assert!(shell_output.contains("another-too"));
    }

    Ok(())
}

#[test]
fn test_scripts_shell_script2() -> Result<(), io::Error> {
    for agent in shared::all_agents() {
        let tmp_dir = TempDir::new("npm")?;
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());

        shared::bash(&cwd, shared::agent_to_init_command(agent).as_str());

        shared::insert_npm_scripts(&cwd, &[("shell", "cd /tmp && pwd")]);

        let shell_output = shared::bash(&cwd, "ny run shell");
        assert!(shell_output.contains("/tmp"));
    }

    Ok(())
}

#[test]
fn test_scripts_npm_script() -> Result<(), io::Error> {
    for agent in shared::all_agents() {
        let tmp_dir = TempDir::new("npm")?;
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());

        shared::bash(&cwd, shared::agent_to_init_command(agent).as_str());
        shared::bash(&cwd, "ny add echo-cli"); // required dep by some scripts

        shared::insert_npm_scripts(
            &cwd,
            &[
                ("npm-simple", "echo-cli 'Hello from npm-simple'"),
                ("npm-simple2", "echo-cli 'lorem ipsum'"),
                ("npm-recursive", "npm run npm-simple && npm run npm-simple2"),
            ],
        );

        let npm_simple_output = shared::bash(&cwd, "ny run npm-simple");
        assert!(npm_simple_output.contains("Hello from npm-simple"));

        let npm_recursive_output = shared::bash(&cwd, "ny run npm-recursive");
        assert!(npm_recursive_output.contains("Hello from npm-simple"));
        assert!(npm_recursive_output.contains("lorem ipsum"));
    }

    Ok(())
}

#[test]
fn test_scripts_passing_extra_args() -> Result<(), io::Error> {
    for agent in shared::all_agents() {
        let tmp_dir = TempDir::new("npm")?;
        let cwd = tmp_dir.into_path();
        println!("Working dir: {}", &cwd.display());

        shared::bash(&cwd, shared::agent_to_init_command(agent).as_str());
        shared::bash(&cwd, "ny add echo-cli"); // required dep by some scripts

        shared::insert_npm_scripts(&cwd, &[("test", "echo-cli 'some-output'")]);

        let stdout = shared::bash(&cwd, "ny run test extra-msg");
        assert!(stdout.contains("extra-msg"));

        let stdout = shared::bash(&cwd, "ny test extra-msg");
        assert!(stdout.contains("extra-msg"));
    }

    Ok(())
}

#[cfg(test)]
mod test_e2e {
    use std::io::{self};
    use std::process::Command;
    // use super::*;
    use tempdir::TempDir;


    #[test]
    fn test_e2e_bun() -> Result<(), io::Error> {
        let tmp_dir = TempDir::new("bun")?;
        let mut ny = assert_cmd::Command::cargo_bin("ny").unwrap();

        let cwd = tmp_dir.path();
        println!("CWD: {:?}", cwd);

        Command::new("bun")
            .arg("init")
            .arg("-y")
            .current_dir(cwd)
            .output()
            .expect("Failed to execute init");

        let output = ny.arg("add")
            .arg("express")
            .current_dir(cwd)
            .output()
            .expect("Failed to execute ny add");

        println!("{:?}", output);

        return Ok(());
    }
}
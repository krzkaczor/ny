use std::{collections::HashMap, path::Path};

use crate::fs::Filesystem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Agent {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl Agent {
    pub fn recognize(fs: &dyn Filesystem, dir: &Path) -> Option<Self> {
        if let Some(agent) = Self::recognize_in_dir(fs, dir) {
            return Some(agent);
        }
        if let Some(parent) = dir.parent() {
            return Self::recognize(fs, parent);
        }
        None
    }

    fn recognize_in_dir(fs: &dyn Filesystem, dir: &Path) -> Option<Self> {
        for (key, value) in FILE_TO_AGENT.iter() {
            if fs.exists(dir.join(key).as_path()) {
                return Some(*value);
            }
        }
        None
    }
}

lazy_static! {
    static ref FILE_TO_AGENT: HashMap<&'static str, Agent> = {
        HashMap::from([
            ("package-lock.json", Agent::Npm),
            ("npm-shrinkwrap.json", Agent::Npm),
            ("yarn.lock", Agent::Yarn),
            ("pnpm-lock.yaml", Agent::Pnpm),
            ("bun.lockb", Agent::Bun),
            ("bun.lock", Agent::Bun),
        ])
    };
}

#[cfg(test)]
mod tests {
    use crate::fs::MockFilesystem;

    use super::*;

    #[test]
    fn test_recognize_in_path() {
        let dir = Path::new("/npm-project");
        let mut mock_fs = MockFilesystem::new();
        mock_fs
            // keep in mind that this can be called in any order
            .expect_exists()
            .returning(|path| path == Path::new("/npm-project/package-lock.json"));

        assert_eq!(Agent::recognize(&mock_fs, dir), Some(Agent::Npm));
    }

    #[test]
    fn test_recognize_in_parent() {
        let dir = Path::new("/npm-project");
        let mut mock_fs = MockFilesystem::new();
        mock_fs
            // keep in mind that this can be called in any order
            .expect_exists()
            .returning(|path| path == Path::new("/yarn.lock"));

        assert_eq!(Agent::recognize(&mock_fs, dir), Some(Agent::Yarn));
    }

    #[test]
    fn test_recognize_bun_new_lockfile_in_parent() {
        let dir = Path::new("/npm-project");
        let mut mock_fs = MockFilesystem::new();
        mock_fs
            // keep in mind that this can be called in any order
            .expect_exists()
            .returning(|path| path == Path::new("/bun.lock"));

        assert_eq!(Agent::recognize(&mock_fs, dir), Some(Agent::Bun));
    }

    #[test]
    fn test_not_recognized() {
        let dir = Path::new("/any-project");
        let mut mock_fs = MockFilesystem::new();
        mock_fs
            // keep in mind that this can be called in any order
            .expect_exists()
            .returning(|_| false);

        assert_eq!(Agent::recognize(&mock_fs, dir), None);
    }
}

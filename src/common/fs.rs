use std::path::{Path, PathBuf};

use eyre::Result;

use mockall::automock;

#[automock]
pub trait Filesystem {
    fn exists(&self, path: &Path) -> bool;
    fn read_to_string(&self, path: &Path) -> Result<String>;
}

pub struct RealFs {}
impl Filesystem for RealFs {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn read_to_string(&self, path: &Path) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }
}

pub fn find_in_parents(fs: &dyn Filesystem, dir: &Path, filename: &str) -> Option<PathBuf> {
    if fs.exists(&dir.join(filename)) {
        Some(dir.join(filename))
    } else {
        dir.parent()
            .and_then(|parent| find_in_parents(fs, parent, filename))
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use mockall::predicate::*;
    use std::path::PathBuf;

    // @todo can this be rewritten to not take ownership?
    pub fn expect_file(mock_fs: &mut MockFilesystem, path: PathBuf, contents: String) {
        mock_fs
            .expect_exists()
            .with(eq(path.clone()))
            .returning(|_| true);
        mock_fs
            .expect_read_to_string()
            .with(eq(path))
            .returning(move |_| Ok(contents.clone()));
    }

    #[test]
    fn test_find_in_parents_in_root() {
        let mut mock_fs = MockFilesystem::new();
        mock_fs
            .expect_exists()
            .with(eq(Path::new("/project/package.json")))
            .returning(|_| true);

        assert_eq!(
            find_in_parents(&mock_fs, Path::new("/project"), "package.json"),
            Some(PathBuf::from("/project/package.json"))
        );
    }

    #[test]
    fn test_find_in_parents_in_parent() {
        let mut mock_fs = MockFilesystem::new();
        mock_fs
            .expect_exists()
            .with(eq(Path::new("/project/nested/package.json")))
            .returning(|_| false);
        mock_fs
            .expect_exists()
            .with(eq(Path::new("/project/package.json")))
            .returning(|_| true);

        assert_eq!(
            find_in_parents(&mock_fs, Path::new("/project/nested"), "package.json"),
            Some(PathBuf::from("/project/package.json"))
        );
    }

    #[test]
    fn test_find_in_parents_not_found() {
        let mut mock_fs = MockFilesystem::new();
        mock_fs.expect_exists().returning(|_| false);

        assert_eq!(
            find_in_parents(&mock_fs, Path::new("/project/nested"), "package.json"),
            None
        );
    }
}

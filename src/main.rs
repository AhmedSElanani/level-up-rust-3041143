extern crate tempfile;
use std::{fs, path};

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        return std::fs::File::open(&self).is_ok();

        // My Solution
        // use std::fs::OpenOptions;

        // return OpenOptions::new()
        //     .read(true)
        //     .create(true)
        //     .open(&self)
        //     .is_ok();
    }

    fn is_writeable(&self) -> bool {
        return fs::metadata(self)
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false);

        // My Solution
        // use std::fs::OpenOptions;

        // return OpenOptions::new()
        //     .write(true)
        //     .create(true)
        //     .open(&self)
        //     .is_ok();
    }

    fn exists(&self) -> bool {
        return self.exists();
    }
}

fn main() {}

// tests from the course
#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile::tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}

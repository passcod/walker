use std::fs::{DirEntry, ReadDir, read_dir, metadata};
use std::io;
use std::path::{Path, PathBuf};

pub struct Walker {
  root: PathBuf,
  cur: Option<ReadDir>,
  stack: Vec<ReadDir>,
}

impl Walker {
  pub fn new(path: &Path) -> io::Result<Self> {
    let cur = try!(read_dir(path));
    Ok(Walker {
      root: path.to_path_buf(),
      cur: Some(cur),
      stack: Vec::new()
    })
  }

  pub fn rewind(&mut self) -> io::Result<()> {
    self.cur = Some(try!(read_dir(self.root.as_path())));
    self.stack = Vec::new();
    Ok(())
  }
}

fn is_dir(path: &Path) -> bool {
  match metadata(path) {
    Ok(m) => m.is_dir(),
    Err(_) => false
  }
}

impl Iterator for Walker {
  type Item = io::Result<DirEntry>;

  fn next(&mut self) -> Option<io::Result<DirEntry>> {
    match self.cur {
      None => return None,
      Some(ref mut d) => {
        match d.next() {
          Some(Err(e)) => return Some(Err(e)),
          Some(Ok(entry)) => {
            let path = entry.path();
            if is_dir(path.clone().as_path()) {
              match read_dir(path) {
                Err(e) => return Some(Err(e)),
                Ok(next) => self.stack.push(next)
              };
            }
            
            return Some(Ok(entry))
          },
          None => {}
        }
      }
    }

    match self.stack.pop() {
      None => return None,
      Some(next) => {
        self.cur = Some(next);
        self.next()
      }
    }
  }
}

#[cfg(test)]
mod test {
  use std::env::temp_dir;
  use std::fs;
  use super::Walker;
  
  macro_rules! check {($e:expr) => (
    match $e {
      Ok(t) => t,
      Err(e) => panic!("{} failed with: {}", stringify!($e), e),
    }
  )}

  #[test]
  fn test_walker() {
    let mut tmp = temp_dir();
    tmp.push("walk_dir");
    let dir = tmp.as_path();
    check!(fs::create_dir(dir));

    let dir1 = &dir.join("01/02/03");
    check!(fs::create_dir_all(dir1));
    check!(fs::File::create(&dir1.join("04")));

    let dir2 = &dir.join("11/12/13");
    check!(fs::create_dir_all(dir2));
    check!(fs::File::create(&dir2.join("14")));

    let files = check!(Walker::new(dir));
    let mut cur = [0; 2];
    for f in files {
      let f = f.unwrap().path();
      let stem = f.file_stem().unwrap().to_str().unwrap();
      let root = stem.as_bytes()[0] - b'0';
      let name = stem.as_bytes()[1] - b'0';
      assert!(cur[root as usize] < name);
      cur[root as usize] = name;
    }

    check!(fs::remove_dir_all(dir));
  }
}

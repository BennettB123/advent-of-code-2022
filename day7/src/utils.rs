use std::{fmt};

pub struct Directory <'a> {
    pub parent: Option<&'a mut Directory<'a>>,
    pub dirs: Vec<Box<Directory<'a>>>,
    pub files: Vec<Box<File>>,
    pub name: String,
}

pub struct File {
    pub name: String,
    pub size: u64,
}

pub struct FileSystem<'a> {
    pub current_dir: &'a mut Directory<'a>,
}

impl<'a> FileSystem<'a> {
    pub fn new(root: & 'a mut Directory<'a>) -> Self {
        FileSystem {
            current_dir: root,
        }
    }

    pub fn cd(& 'a mut self, target_dir: &str) -> Result<(), CdError>{
        // check if we are going to parent
        if target_dir == ".." {
            self.current_dir = match &mut self.current_dir.parent {
                Some(parent) => parent,
                None => return Err(CdError),
            };
            return Ok(());
        }

        // search for target_dir
        for dir in  &mut self.current_dir.dirs {
            if dir.name == target_dir {
                self.current_dir = dir;
                return Ok(());
            }
        }
        // if target_dir does not exist, return CdError
        Err(CdError)
    }

    pub fn mkdir(&'a mut self, mut new_dir: Directory<'a>) {
        new_dir.parent = Some(self.current_dir);
        self.current_dir.dirs.push(Box::new(new_dir));
    }

    pub fn touch(&mut self, new_file: File) {
        self.current_dir.files.push(Box::new(new_file));
    }
}

// Error Types
#[derive(Debug)]
pub struct CdError;
impl std::error::Error for CdError {}
impl fmt::Display for CdError {
    fn fmt(&self,  f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Directory does not exist")
    }
}


/////////////////////////////////////////////////////////////
// rust_dir_nav::lib1.rs                                   //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 12 Apr 2020  //
/////////////////////////////////////////////////////////////

use std::io;
use std::fs::{self, DirEntry};
#[allow(unused_imports)]
use std::path::{Path, PathBuf};
//use std::ffi::OsStr;

pub trait Event {
  fn do_dir(&self, s:&String);
  fn do_file(&self, s:&String);
}
struct Appl;
impl Event for Appl {
    fn do_dir(&self, d:&String) {
        print!("\n  {:?}", d);
    }
    fn do_file(&self, f:&String) {
        print!("\n    {:?}", f);
    }    
}

type Patterns = Vec::<std::ffi::OsString>;

#[allow(dead_code)]
pub struct DirNav<App:Event> { 
    pats:Patterns,
    app:App,
}
impl<App:Event> DirNav<App> {
    pub fn new(a:App) -> Self where App:Event {
        Self {
            pats : Patterns::new(),
            app:a,
        }
    }
    /*-- takes either String or &str --*/
    pub fn add_pat<S:Into<String>>(&mut self, p:S) -> &mut DirNav<App> {
        let mut t = std::ffi::OsString::new();
        t.push(p.into());
        self.pats.push(t);
        self
    }
    pub fn clear_pat(&mut self) {
        self.pats.clear();
    }
    pub fn visit(&self, dir: &Path) -> io::Result<()> where App:Event {
        let mut cd = dir.to_string_lossy().to_string();
        let mut first = true;
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    cd = self.replace_sep(&path);
                    self.visit(&path)?;
                } else {
                    if self.in_patterns(&entry) {
                        if first {
                            print!("\n{:#?}", cd.replace("\\", "/"));
                            first = false;
                        }
                        let name = entry.file_name().to_string_lossy().to_string();
                        self.app.do_file(&name);
                        //cb(&entry);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn replace_sep(&self, path:&Path) -> String {
        let rtn = path.to_string_lossy();
        rtn.replace("\\", "/")
    }

    pub fn in_patterns(&self, d:&DirEntry) -> bool {
        let p = d.path();
        let ext = p.extension();
        match ext {
            Some(extn) => {
                return self.pats.contains(&(extn.to_os_string()));
            }
            None => return false,
        }
    }        
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

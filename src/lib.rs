/////////////////////////////////////////////////////////////
// rust_dir_nav::lib1.rs                                   //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 12 Apr 2020  //
/////////////////////////////////////////////////////////////
/*
   DirNav<App> is a directory navigator that  uses the generic
   parameter App to define how files and directories are
   handled.
*/
use std::io;
use std::fs::{self, DirEntry};
#[allow(unused_imports)]
use std::path::{Path, PathBuf};

/// trait required of the App generic parameter type
pub trait DirEvent {
  fn do_dir(&mut self, d:&String);
  fn do_file(&mut self, f:&String);
}
//---------------------------------------
// Sample implementation of DirNav param
// -------------------------------------- 
// struct Appl;
// impl DirEvent for Appl {
//     fn do_dir(&mut self, d:&String) {
//         print!("\n  {:?}", d);
//     }
//     fn do_file(&mut self, f:&String) {
//         print!("\n    {:?}", f);
//     }    
// }

type Patterns = Vec::<std::ffi::OsString>;

/// Directory Navigator Structure
#[allow(dead_code)]
pub struct DirNav<App:DirEvent> { 
    /// file exts to look for
    pats:Patterns,
    /// instance of App : DirEvent
    app:App,
    /// number of files processed
    num_fun : usize,
    /// number of dirs processed
    num_dir : usize,
}
impl<App:DirEvent> DirNav<App> {
    pub fn new(a:App) -> Self where App:DirEvent {
        Self {
            pats : Patterns::new(),
            app:a,
            num_fun: 0,
            num_dir: 0,
        }
    }
    /// return reference to App to get results
    pub fn get_app(&mut self) -> &App { &self.app }
    /// return number of dirs processed
    pub fn get_dirs(&self) -> usize { self.num_dir }
    /// return number of files processed
    pub fn get_funs(&self) -> usize { self.num_fun }

    /// add extention to search for
    /// takes either String or &str
    pub fn add_pat<S:Into<String>>(&mut self, p:S) -> &mut DirNav<App> {
        let mut t = std::ffi::OsString::new();
        t.push(p.into());
        self.pats.push(t);
        self
    }
    /// remove all patterns from store
    pub fn clear_pat(&mut self) {
        self.pats.clear();
    }
    /// Depth First Search for file extentions starting at path dir
    pub fn visit(&mut self, dir: &Path) 
             -> io::Result<()> where App:DirEvent {
        self.num_dir += 1;
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
                    self.num_fun += 1;
                    if self.in_patterns(&entry) {
                        if first {
                            print!("\n{:#?}", cd.replace("\\", "/"));
                            first = false;
                        }
                        let name = 
                          entry.file_name().to_string_lossy().to_string();
                        self.app.do_file(&name);
                    }
                }
            }
        }
        Ok(())
    }
    /// replace Windows directory seperator with Linux seperator
    pub fn replace_sep(&self, path:&Path) -> String {
        let rtn = path.to_string_lossy();
        rtn.replace("\\", "/")
    }
    /// does store contain d.path().extension() ?
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
    // test_setup() should run first. To ensure that:
    //   use cargo -- --test-threads=1
    // to see console output:
    //   use cargo test -- --show-output --test-threads=1
    use super::*;
    #[test]
    fn test_setup() {
        let _ = std::fs::create_dir("./test_dir");
        let _ = std::fs::create_dir("./test_dir/test_sub1_dir");
        let _ = std::fs::create_dir("./test_dir/test_sub2_dir");
        let _ = std::fs::File::create("./test_dir/test_file.rs");
        let _ = std::fs::File::create("./test_dir/test_sub1_dir/test_file1.rs");
        let _ = std::fs::File::create("./test_dir/test_sub1_dir/test_file2.exe");
        let _ = std::fs::File::create("./test_dir/test_sub2_dir/test_file3.txt");
    }
    #[test]
    fn test_walk() {
        /// ApplTest - generic parameter for testing DirNav<ApplTest>
        /// rslt_store holds results of file search
        #[derive(Debug)]
        struct ApplTest {
            rslt_store: Vec<String>,
        }
        impl DirEvent for ApplTest {
            fn do_dir(&mut self, _d:&String) {
                //print!("\n  {:?}", d);
            }
            fn do_file(&mut self, f:&String) {
                //print!("\n    {:?}", f);
                self.rslt_store.push((*f).clone());
            }
        }
        let a = ApplTest { rslt_store: Vec::<String>::new(), };
        let mut dn = DirNav::<ApplTest>::new(a);
        dn.add_pat("rs").add_pat("exe").add_pat("txt");
        let mut pb = PathBuf::new();
        pb.push("./test_dir".to_string());
        let _ = dn.visit(&pb);
        let rl = &dn.get_app().rslt_store;
        /*
          run exe in target/debug with --nocapture option
          to see output of print statement below.
        */
        print!("\n  {:?}",rl);

        // test for found files
        let l = |s:&str| -> String { s.to_string() };

        assert!(rl.contains(&l("test_file.rs")));
        assert!(rl.contains(&l("test_file1.rs")));
        assert!(rl.contains(&l("test_file2.exe")));
        assert!(rl.contains(&l("test_file3.txt")));
        /*
          uncomment line below to make test fail
        */
        //assert!(rl.contains(&l("foobar")));
    }
}

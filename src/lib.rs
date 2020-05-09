/////////////////////////////////////////////////////////////
// rust_dir_nav::lib1.rs                                   //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 12 Apr 2020  //
/////////////////////////////////////////////////////////////
/*
   DirNav<App> is a directory navigator that  uses the generic
   parameter App to define how files and directories are
   handled.
   - displays only paths that have file targets by default
   - hide(false) will show all directories traversed
   - recurses directory tree at specified root by default
   - recurse(false) examines only specified path.
*/
use std::fs::{self, DirEntry};
use std::io;
use std::io::{Error, ErrorKind};
#[allow(unused_imports)]
use std::path::{Path, PathBuf};

/// trait required of the App generic parameter type
pub trait DirEvent {
    fn do_dir(&mut self, d: &str);
    fn do_file(&mut self, f: &str);
}
//---------------------------------------
// Sample implementation of DirNav param
// --------------------------------------
// #[derive(Debug, Default)]
// pub struct Appl;
// impl DirEvent for Appl {
//     fn do_dir(&mut self, d:&str) {
//         print!("\n  {:?}", d);
//     }
//     fn do_file(&mut self, f:&str) {
//         print!("\n    {:?}", f);
//     }
// }

/////////////////////////////////////////////////
// Patterns are a collection of extension strings
// used to identify files as search targets

type SearchPatterns = Vec<std::ffi::OsString>;

/// Directory Navigator Structure
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DirNav<App: DirEvent> {
    /// file extensions to look for
    pats: SearchPatterns,
    /// instance of App : DirEvent, requires do_file and do_dir methods
    app: App,
    /// number of files processed
    num_file: usize,
    /// number of dirs processed
    num_dir: usize,
    /// recurse ?
    recurse : bool,
    /// hide dirs with no targets ?
    hide: bool,
}
impl<App: DirEvent + Default> DirNav<App> {
    pub fn new() -> Self
    where
        App: DirEvent + Default,
    {
        Self {
            pats: SearchPatterns::new(),
            app: App::default(),
            num_file: 0,
            num_dir: 0,
            recurse: true,
            hide: true,
        }
    }
    /// do recursive visit?
    pub fn recurse(&mut self, p:bool) {
        self.recurse = p;
    }
    /// hide dirs with no targets?
    pub fn hide(&mut self, p:bool) {
        self.hide = p;
    }
    /// return reference to App to get results, if any
    pub fn get_app(&mut self) -> &mut App {
        &mut self.app
    }
    /// return number of dirs processed
    pub fn get_dirs(&self) -> usize {
        self.num_dir
    }
    /// return number of files processed
    pub fn get_files(&self) -> usize {
        self.num_file
    }
    /// return patterns, e.g., file extensions to look for
    pub fn get_patts(&self) -> &SearchPatterns {
        &self.pats
    }

    /// add extention to search for - takes either String or &str
    pub fn add_pat<S: Into<String>>(&mut self, p: S) -> &mut DirNav<App> {
        let mut t = std::ffi::OsString::new();
        t.push(p.into());
        self.pats.push(t);
        self
    }
    /// reset to default state
    pub fn clear(&mut self) {
        self.pats.clear();
        self.num_dir = 0;
        self.num_file = 0;
        self.app = App::default();
    }
    /// Depth First Search for file extentions starting at path dir<br />
    /// Displays only directories with files matching pattern
    pub fn visit(&mut self, dir: &Path) -> io::Result<()>
    where App: DirEvent
    {
        self.num_dir += 1;
        let dir_name: String = 
          self.replace_sep(dir).to_string_lossy().to_string();
        let mut files = Vec::<std::ffi::OsString>::new();
        let mut sub_dirs = Vec::<std::ffi::OsString>::new();
        if dir.is_dir() {
            /* search local directory */
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let cd = self.replace_sep(&path);
                    sub_dirs.push(cd);
                } else {
                    self.num_file += 1;
                    if self.in_patterns(&entry) | self.pats.is_empty() {
                        files.push(entry.file_name());
                    }
                }
            }
            /*-- display only dirs with found files --*/
            if !files.is_empty() || !self.hide {
                self.app.do_dir(&dir_name);
            }
            for fl in files {
                let flnm = fl.to_string_lossy().to_string();
                self.app.do_file(&flnm);
            }
            /*-- recurse into subdirectories --*/
            for sub in sub_dirs {
                let mut pb = std::path::PathBuf::new();
                pb.push(sub);
                if self.recurse {
                    self.visit(&pb)?;
                }
            }
            return Ok(());  // normal return
        }
        Err(Error::new(ErrorKind::Other, "not a directory"))
    }
    /// replace Windows directory separator with Linux separator
    pub fn replace_sep(&self, path: &Path) -> std::ffi::OsString {
        let rtn = path.to_string_lossy();
        let mod_path = rtn.replace("\\", "/");
        let mut os_str: std::ffi::OsString = std::ffi::OsString::new();
        os_str.push(mod_path);
        os_str
    }
    /// does store contain d.path().extension() ?
    pub fn in_patterns(&self, d: &DirEntry) -> bool {
        let p = d.path();
        let ext = p.extension();
        match ext {
            Some(extn) => self.pats.contains(&(extn.to_os_string())),
            None => false,
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
    #[derive(Debug)]
    struct ApplTest {
        rslt_store: Vec<String>,
    }
    impl DirEvent for ApplTest {
        fn do_dir(&mut self, _d: &str) {
            //print!("\n  {:?}", d);
        }
        fn do_file(&mut self, f: &str) {
            //print!("\n    {:?}", f);
            self.rslt_store.push((*f).to_string());
        }
    }
    impl Default for ApplTest {
        fn default() -> Self {
            ApplTest {
                rslt_store: Vec::<String>::new(),
            }
        }
    }
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
        let mut dn = DirNav::<ApplTest>::new();
        dn.add_pat("rs").add_pat("exe").add_pat("txt");
        let mut pb = PathBuf::new();
        pb.push("./test_dir".to_string());
        let _ = dn.visit(&pb);
        let rl = &dn.get_app().rslt_store;
        /*
          run exe in target/debug with --nocapture option
          to see output of print statement below.
        */
        print!("\n  {:?}", rl);

        // test for found files
        let l = |s: &str| -> String { s.to_string() };

        assert!(rl.contains(&l("test_file.rs")));
        assert!(rl.contains(&l("test_file1.rs")));
        assert!(rl.contains(&l("test_file2.exe")));
        assert!(rl.contains(&l("test_file3.txt")));
        /*
          uncomment line below to make test fail
        */
        //assert!(rl.contains(&l("foobar")));
    }
    #[test]
    fn test_patts() {
        let mut dn = DirNav::<ApplTest>::new();
        dn.add_pat("foo").add_pat("bar");
        assert_eq!(dn.get_patts().len(), 2);
        let pats = dn.get_patts();
        let mut foo_str = std::ffi::OsString::new();
        foo_str.push("foo");
        assert!(pats.contains(&foo_str));
        let mut bar_str = std::ffi::OsString::new();
        bar_str.push("bar");
        assert!(pats.contains(&bar_str));
        dn.clear();
        assert_eq!(dn.get_patts().len(), 0);
    }
}

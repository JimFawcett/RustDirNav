/////////////////////////////////////////////////////////////
// rust_dir_nav::test1.rs                                  //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 12 Apr 2020  //
/////////////////////////////////////////////////////////////

use rust_dir_nav::*;
#[allow(unused_imports)]
use std::env::current_dir;
use std::io;

struct Appl;
impl DirEvent for Appl {
    fn do_dir(&mut self, d: &str) {
        print!("\n  {}", d);
    }
    fn do_file(&mut self, f: &str) {
        print!("\n      {}", f);
    }
}
impl Default for Appl {
    fn default() -> Self {
        Appl
    }
}

fn main() -> io::Result<()> {

    let mut dn = DirNav::<Appl>::new();

    /*-- takes a variety of formats --*/

    let _pat1: String = "rs".to_string();
    let _pat4: String = "rlib".to_string();

    dn.add_pat(&_pat1);
    dn.add_pat("toml".to_string());
    dn.add_pat("txt");
    dn.add_pat(_pat4);
    dn.add_pat(&"exe".to_string());

    //dn.hide(false);

    let path = current_dir()?;
    print!("\n  Searching path {:?}\n", &path);

    let _rslt = dn.visit(&path)?;
    
    print!(
        "\n\n  processed {} files and {} dirs",
        dn.get_files(),
        dn.get_dirs()
    );
    print!("\n");

    dn.clear();
    dn.add_pat("rs").add_pat("toml").add_pat("exe").add_pat("txt");
    let mut path = std::path::PathBuf::new();
    path.push("./test_dir");
    print!("\n  Searching path {:?}\n", &path);
    let _rslt = dn.visit(&path)?;
    print!(
        "\n\n  processed {} files in {} dirs",
        dn.get_files(),
        dn.get_dirs()
    );
    ///////////////////////////////////////////////
    // uncomment lines below to see error return
    //---------------------------------------------
    // print!("\n");
    // path.pop();
    // path.push("foobar");
    // dn.visit(&path)?;
    
    print!("\n\n");
    Ok(())
}

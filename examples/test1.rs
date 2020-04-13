/////////////////////////////////////////////////////////////
// rust_dir_nav::test1.rs                                  //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 12 Apr 2020  //
/////////////////////////////////////////////////////////////

use std::io;
use std::env::current_dir;
use rust_dir_nav::*;

struct Appl;
impl Event for Appl {
    fn do_dir(&self, d:&String) {
        print!("\n  {:?}", d);
    }
    fn do_file(&self, f:&String) {
        print!("\n    {:?}", f);
    }
}

fn main() -> io::Result<()> {

    let a = Appl;
    let mut dn = DirNav::<Appl>::new(a);

    let pat1:String = "rs".to_string();
    let _pat2:String = "toml".to_string();
    let _pat3:String = "JSON".to_string();
    let pat4:String = "rlib".to_string();
    let _pat5:String = "exe".to_string();

    /*-- takes a variety of formats --*/
    dn.add_pat(&pat1);
    dn.add_pat("toml".to_string());
    dn.add_pat("rmeta");
    dn.add_pat(pat4);
    dn.add_pat(&"exe".to_string());

    let path = current_dir()?;
    // path.pop();
    let _rslt = dn.visit(&path);
    print!("\n");

    dn.clear_pat();
    dn.add_pat("rs").add_pat("toml").add_pat("exe");
    let rslt = dn.visit(&path);
    print!("\n\n");
    rslt
}

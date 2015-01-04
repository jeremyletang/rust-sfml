// Cargo build script
// See: http://doc.crates.io/build-script.html
//
use std::os;

fn main(){
  let key = "LDFLAGS";
  match os::getenv(key){
    Some(val) => println!("cargo:rustc-flags=-L {}", val),
    _ => {}
  }
}

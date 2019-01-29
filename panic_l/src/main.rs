use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
fn main() {
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file:{:?}", other_error),
        },
    };
    
    read_username_from_file();

}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello2.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}

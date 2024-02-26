use std::fs::File;
use std::io;

fn optional_fun() {
    let f: Result<File, io::Error> = File::open("file.txt");
    match f {
        Ok(file) => println!("there is a file {:?}!", file),
        Err(e) => println!("no file {e}")
    }
    let v = vec![1, 2, 3];
    let num: Option<&i32> = v.get(2);
    let real_num: &i32 = num.unwrap();
}



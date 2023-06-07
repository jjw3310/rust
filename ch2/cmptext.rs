use std::fs;
fn main() {
    let afile = "../ch1/fizzbuzz_python.txt";
    let bfile = "../ch1/fizzbuzz_rust.txt";

    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();

    let astr = astr.trim();
    let bstr = bstr.trim();

    if astr == bstr {
        println!("ok");
    } else {
        println!("ng");
    }
}
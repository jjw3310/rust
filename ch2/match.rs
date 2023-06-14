//error file

fn main() {
    let text = match fs::read_to_string("somefile.txt") {
        Ok(text) => {
            text
        },
        Err(e) => {
            println!("error");
        }
    };
    println!("{}", text);
}
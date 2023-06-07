use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("읽어올 파일을 지정");
        return;
    }
    // 두 번째 요소 (첫 번째는 프로그램명)
    let filename = &args[1];

    let text = fs::read_to_string(filename).unwrap();

    println!("{}", text);
}
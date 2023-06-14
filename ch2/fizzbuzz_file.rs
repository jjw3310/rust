use std::fs::{self, File};
use std::io::{Write, BufWriter};

fn main() {
    let filename = "fizzbuzz_file_result.txt";
    //파일로 저장할 부분 블록 지정
    {
        let fp = File::create(filename).unwrap();
        let mut writer = BufWriter::new(fp);

        for i in 1..=100 {
            let mut line = format!("{}\n", i);
            if(i % 3==0) && (i % 5 == 0) {
                line = String::from("FizzBuzz\n");
            } else if i % 3 == 0 {
                line = String::from("Fizz\n");
            } else if i % 5 == 0 {
                line = String::from("Buzz\n");
            }

            //파일에 쓰기
            let bytes = line.as_bytes();
            writer.write(bytes).unwrap();
        }
    } //<- 파일은 여기서 자동 닫힘

    let s = fs::read_to_string(filename).unwrap();
    println!("{}", s);
}
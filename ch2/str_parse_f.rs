// fn main() {
//     let s = "3.1415";

//     let num = s.parse::<f64>().expect("변환 실패");

//     println!("{:2}", num);
// }

fn main() {
    let s = "3.1415a";

    let num = s.parse::<f64>();

    match num {
        Ok(result) => println!("{:.2}", result),
        Err(e) => println!("에러가 발생했습니다. 이유 : '{:?}'", e)
    }

    println!("{:2}", num);
}
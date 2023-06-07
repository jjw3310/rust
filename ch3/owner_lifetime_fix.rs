fn gen_message() -> String {
    let msg = String::from("실수!@");
    return msg; // 소유권이 함수의 반환 값으로 이동
}

fn main() {
    let m = gen_message(); // 소유권이 m으로 이동
    println!("{}", m); //ok
}
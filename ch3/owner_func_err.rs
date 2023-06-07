fn main() {
    let g1 = String::from("실수할 줄 아는 것!");
    show_message(g1); // 소유권 이동
    println!("{}", g1); // g1 사용 불가
}

fn show_message(message: String) {
    println!("{}", message);
}
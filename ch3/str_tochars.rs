fn main() {
    let pr = "구슬이 서 말";
    for c in pr.chars() {
        print!("[{}]", c);
    }

    println!("\n글자 수 = {}자", pr.chars().count());

    //Vec<char>로 변환
    let pr_chars: Vec<char> = pr.chars().collect();
    println!("Vec<char> : {:?}", pr_chars);
    for c in pr_chars.iter() {
        print!("({})", c);
    }

    println!("\n글자 수 = {}자", pr_chars.len());
}
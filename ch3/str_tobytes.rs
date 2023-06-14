fn main() {
    let pr = "구슬이 서말";
    for c in pr.bytes() {
        print!("{:2x}", c);
    }

    println!("\n바이트 = {}B", pr.len());
}
fn main() {
    let s2 = "abcdefg";
    println!("{}", &s2[0..1]);

    let s = "안녕하세요";

    let ch = &s[..3];
    println!("{}", ch); //안

    let ch = &s[6..9];
    println!("{}", ch); //하
}
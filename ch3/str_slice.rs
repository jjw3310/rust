fn main() {
    let pr = "지혜는 무기!";
    //앞의 2글자 6바이트
    println!("앞 2글자 : {}", &pr[0..6]);
    //'무기' 부분
    println!("4-5번째 글자 : {}", &pr[10..16]);
}
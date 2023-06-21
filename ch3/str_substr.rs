fn main() {
    let pr = "😁🎈✔😂🐾🙌";
    let mut sub1 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i < 2 { sub1.push(c); continue;}
        break;
    }
    let mut sub2 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if 3 <= i && i <= 4 { sub2.push(c);}
    }
    println!("앞 2글자 : {}", &pr[0..8]); //1
    println!("앞 2글자: {}", sub1); //2
    println!("4-5번째 문자 : {}", sub2); //3
}
// fn echo(s: &'static str) {
//     println!("{}", s);
// }

// fn main() {
//     echo("웅변은 은");
//     echo("침묵은 금");


//     //이부분 에러
//     let s = String::from("테스트");
//     println!("{}", s);
//     // echo(&s);
// }


fn echo(s: &'static str) {
    println!("{}", string);
}

fn main() {
    let s = String::from("GM!");
    echo("hello"); //1
    println!(s); //2
    echo("{}", s); //3
    println!("{}", s); //4
}
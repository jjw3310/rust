fn main() {
    let s = "안녕하세요";

    // let ch = s.chars().nth(0).unwrap();
    // println!("{}", ch); //안

    // let ch = s.chars().nth(2).unwrap();
    let mut ch = "";
    for i in 0..5 {
        ch = s.chars().nth(i).unwrap();
       println!("{}", ch);
    }
}


// fn main() {
//     let s = "안녕하세요";
    
//     for i in 0..5 {
//         if let Some(ch) = s.chars().nth(i) {
//             print!("{}", ch);
//        } else {
//             print!("error");
//         }
//     }
// }
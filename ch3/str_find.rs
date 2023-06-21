fn main() {
    let s = "제주도와 특산품 중 귤은 겨울에 많이 먹을 수 있다!";

    //귤!
    match s.find("귤") {
        Some(i) => println!("귤 = {}B", i),
        None => println!("'귤'이란 단어는 없다`#");
    };
    //바나나
    match s.find("바나나") {
        Some(i) => println!("바나나!! = {}B", i),
        None => println!("빠나나 달라!");
    }
}

// fn main() {
//     let s = "제주도와 특산품 중 귤은 겨울에 많이 먹을 수 있다!";
//     let text = match s.find("귤") {
//         Ok(text) => {
//             text
//         },
//         Err(e) => {
//             println!("error");
//         }
//     };
//     println!("{}", text);
// }
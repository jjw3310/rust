// fn main() {
//     let mut g1 = String::from("실수!@");
//     g1 = show_message(g1);
//     println!("{}", g1); // ok
// }

// fn show_message(message: String) -> String {
//     println!("{}", message);
//     return message;
// }

fn main() {
    let g1 = String::from("실수!@");
    show_message(&g1);
    println!("{}", g1); // ok
}

fn show_message(message: &String) {
    println!("{}", message);
}
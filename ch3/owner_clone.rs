fn main() {
    let g1 = String::from("온화한 마음");
    let g2 = g1.clone(); // 복제하면 소유권은 이동하지 않음
    println!("{}", g1); // ok
    println!("{}", g2); // ok
}
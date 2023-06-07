fn x2(arg: &mut i32) {
    *arg = *arg * 2;
}

fn main() {
    let mut v = 16;
    x2(&mut v); //인수에 2 곱함
    println!("{}", v);
}
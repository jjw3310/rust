fn main() {
    {
        let mut v = 300; //가변
        v = v + 5;
        println!("{}", v);
    }
    {
        let v = 300;
        let v = v + 5;
        println!("{}", v); 
    }
}
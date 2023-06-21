
#[cfg(test)]
mod tests {
    //1
    #[test]
    fn calc_test1() {
        assert_eq!(100*2, 200);
        assert_eq!((1+2)*3, 9);
    }
    //2
    #[test]
    fn calc_test2() {
        assert_eq!(100*2, 200);
        assert_eq!((1+2)*3, 9);
        assert_eq!(1+2+4, 7);
    }
    //3
    #[test]
    fn calc_test3() {
        assert_eq!(1+2+4, 7);
        assert_eq!((1+4), (4-1));
    }
    //4
    #[test]
    fn calc_test4() {
        assert_eq!(100*2, 200);
    }
}

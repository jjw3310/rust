fn main() {
    let pc_price = 980000.0;
    let a_shop_fee = 12000.0;
    let a_shop_sale = 0.8;
    let b_shop_sale = 0.9;

    println!("{}원", pc_price * a_shop_sale + a_shop_fee);
    println!("{}원", pc_price * b_shop_sale);
}
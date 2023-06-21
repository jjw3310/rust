use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 난수 초기화
    let mut seed = rand_init();
    // 100개 난수
    for _ in 0..100 {
        let v = rand(&mut seed, 1, 6);
        println!("{}", v);
    }
}

// 난수 초깃값 함수
fn rand_init() -> u32 {
    //현재 시각 활용 난수 초깃값 생성
    SystemTime::now()
    .duration_since(UINX_EPOCH).unwrap()
    .as_millis() as u32
}

// start부터 end 사이의 난수 생성
fn rand(seed: &mut u32, start: u32, end: u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    return *seed % (end - start + 1) + start;
}
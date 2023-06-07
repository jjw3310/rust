use rand::Rng;

const MAP_N: usize = 25;
fn main() {
    // 난수 생성기 준비
    let mut rng = rand::thread_rng();

    //미로 초기화
    let mut maze = [[0; MAP_N]; MAP_N]; // 배열 변수 선언

    //둘레 벽으로 감싸기
    for n in 0..MAP_N {
        maze[n][0] = 1; // 위쪽 벽
        maze[0][n] = 1; // 왼쪽 벽
        maze[n][MAP_N-1] = 1; // 오른쪽 벽
        maze[MAP_N-1][n] = 1; // 아래쪽 벽
    }

    //2칸마다 1개 벽
    for y in 2..MAP_N-2 {
        for x in 2..MAP_N-2 {
            if x % 2 == 1 || y % 2 == 1 { continue; }
            maze[y][x] = 1; // 벽
            //상하좌우 중 어느 하나를 벽으로 만들기
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y-1][x] = 1, //상
                1 => maze[y+1][x] = 1, //하
                2 => maze[y][x-1] = 1, //좌
                3 => maze[y][x+1] = 1, //우
                _ => {},
            }
        }
    }

    //미로 출력
    // 0과 1을 각각 흰, 검 타일로 치환
    let titles = ["□", "■"];
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", titles[maze[y][x]])
        }
        println!("");
    }

}
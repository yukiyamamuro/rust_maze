use rand::Rng;

const MAP_WIDTH: usize = 25;

fn main() {
    // initialize
    let mut rng = rand::thread_rng();

    let mut maze = [[0; MAP_WIDTH]; MAP_WIDTH];
    for n in 0..MAP_WIDTH {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAP_WIDTH -1] = 1;
        maze[MAP_WIDTH -1][n] = 1;
    }

    // generate
    for y in 2..MAP_WIDTH-2 {
        for x in 2..MAP_WIDTH-2 {
            if x % 2 == 1 || y % 2 == 1 { continue; }
            maze[y][x] = 1;
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y-1][x] = 1,
                1 => maze[y+1][x] = 1,
                2 => maze[y][x-1] = 1,
                3 => maze[y][x+1] = 1,
                _ => {},
            }
        }
    }



    // output
    let tiles = ["  ", "ZZ"];
    for y in 0..MAP_WIDTH {
        for x in 0..MAP_WIDTH {
            print!("{}", tiles[maze[y][x]]);
        }
        // 改行
        println!("");
    }
}

static INPUT: &'static str = include_str!(r"input.txt");

fn sum_trees(h: usize, w: usize, grid: Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut xc = 0;
    let mut yc = 0;
    let mut c = 0;

    while yc < h {
        if grid[yc][xc] == '#' {
            c += 1;
        }

        xc = (xc + x) % w;
        yc += y;
    }

    c
}

fn main() {
    let path: Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let h = path.len();
    let w = path[0].len();

    let mut r: u32 = sum_trees(h, w, path.clone(), 1, 1);
    r *= sum_trees(h, w, path.clone(), 3, 1);
    r *= sum_trees(h, w, path.clone(), 5, 1);
    r *= sum_trees(h, w, path.clone(), 7, 1);
    r *= sum_trees(h, w, path, 1, 2);

    println!("{}", r);
}

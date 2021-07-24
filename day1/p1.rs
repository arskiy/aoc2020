static INPUT: &'static str = include_str!(r"input1.txt");

fn main() {
    let inp: Vec<i32> = INPUT.trim().split('\n').map(|x| x.parse::<i32>().unwrap()).collect();
    for n in &inp {
        for m in &inp {
            if n + m == 2020 {
                println!("{} + {} = 2020\n{} * {} = {}", n, m, n, m, n * m);
            }
        }
    }
}


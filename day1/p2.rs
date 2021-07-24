static INPUT: &'static str = include_str!(r"input2.txt");

fn main() {
    let inp: Vec<i32> = INPUT
        .trim()
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    for n in &inp {
        for m in &inp {
            for o in &inp {
                if n + m + o == 2020 {
                    println!(
                        "{} + {} + {} = 2020\n{} * {} * {} = {}",
                        n,
                        m,
                        o,
                        n,
                        m,
                        o,
                        n * m * o
                    );
                }
            }
        }
    }
}

static INPUT: &'static str = include_str!(r"input1.txt");

fn main() {
    let inp: Vec<&str> = INPUT.trim().split('\n').collect();

    let mut res = 0;
    for i in inp {
        let a = i.split_whitespace().collect::<Vec<&str>>();
        let letter = a.clone()[1].replace(":", "").pop().unwrap();
        let ns = &a[0]
            .split("-")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        let pass = a[2];

        let mut count = 0;
        for l in pass.chars() {
            if l == letter {
                count += 1;
            }
        }

        if count >= ns[0] && count <= ns[1] {
            res += 1;
        }
    }
    println!("{}", res)
}

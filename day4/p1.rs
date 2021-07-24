static INPUT: &'static str = include_str!(r"input.txt");
/*
fn parse(inp: &str) -> Vec<Vec<&str>> {
    let x = inp.lines();
    for l in x {
        if l == "\n" {}
    }
}*/

fn main() {
    let x = INPUT.split("\n\n").collect::<Vec<&str>>();
    let mut cnt = 0;
    for l in x {
        let mut valid = 0;
        let s = l.replace("\n", " ");
        if s.find("byr").is_some() {
            valid += 1;
        }
        if s.find("iyr").is_some() {
            valid += 1;
        }
        if s.find("eyr").is_some() {
            valid += 1;
        }
        if s.find("hgt").is_some() {
            valid += 1;
        }
        if s.find("hcl").is_some() {
            valid += 1;
        }
        if s.find("ecl").is_some() {
            valid += 1;
        }
        if s.find("pid").is_some() {
            valid += 1;
        }
        println!("{} {}", s, valid);

        if valid == 7 {
            cnt += 1;
        }
    }

    println!("{:?}", cnt);

    //println!("{:?}", x);
}

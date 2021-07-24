static INPUT: &'static str = include_str!(r"input.txt");

fn main() {
    let x = INPUT.split("\n\n").collect::<Vec<&str>>();
    let mut pass_cnt = 0;
    for l in x {
        let mut valid_fields = 0;

        let s = l.replace("\n", " ");
        let s = s.split_whitespace().collect::<Vec<&str>>();

        let mut v = Vec::new();

        for i in &s {
            let fields = i.split(":").collect::<Vec<&str>>();
            v.push(fields);
        }

        for i in &v {
            println!("{:?}", i);
            match i[0] {
                "byr" => valid_fields += check_byr(i[1]) as i32,
                "iyr" => valid_fields += check_iyr(i[1]) as i32,
                "eyr" => valid_fields += check_eyr(i[1]) as i32,
                "hgt" => valid_fields += check_hgt(i[1]) as i32,
                "hcl" => valid_fields += check_hcl(i[1]) as i32,
                "ecl" => valid_fields += check_ecl(i[1]) as i32,
                "pid" => valid_fields += check_pid(i[1]) as i32,
                _ => (),
            }
        }

        println!("{:?} {}", &s, valid_fields);

        if valid_fields == 7 {
            pass_cnt += 1;
        }
    }

    println!("{:?}", pass_cnt);
}

fn check_byr(s: &str) -> bool {
    let n = s.parse::<i32>().unwrap();
    n >= 1920 && n <= 2002
}

fn check_iyr(s: &str) -> bool {
    let n = s.parse::<i32>().unwrap();
    n >= 2010 && n <= 2020
}

fn check_eyr(s: &str) -> bool {
    let n = s.parse::<i32>().unwrap();
    n >= 2020 && n <= 2030
}

fn check_hgt(s: &str) -> bool {
    if s.find("cm").is_some() {
        let x = s.trim_end_matches("cm");
        let y = x.parse::<i32>().unwrap();
        return y >= 150 && y <= 193;
    } else {
        let x = s.trim_end_matches("in");
        let y = x.parse::<i32>().unwrap();
        return y >= 59 && y <= 76;
    };
}

fn check_hcl(s: &str) -> bool {
    let mut it = s.chars();
    if it.next() != Some('#') {
        return false;
    }

    for _ in 0..6 {
        let x = it.next();
        if !x.unwrap().is_numeric() && !('a'..='f').contains(&x.unwrap()) {
            println!("{:?}", x);
            return false;
        }
    }

    if it.next() != None {
        return false;
    }

    true
}

fn check_ecl(s: &str) -> bool {
    let a = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    a.contains(&s)
}

fn check_pid(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }

    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }

    true
}

use std::cmp;
use std::num::IntErrorKind;

fn parse(input: &String) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for line in input.split('\n') {
        match line.parse::<i32>() {
            Ok(v) => {
                res.push(v);
            }
            Err(e) => match e.kind() {
                IntErrorKind::Empty => {
                    // This is a new elf.
                    res.push(0);
                }
                _ => {
                    panic!("unable to parse {line}")
                }
            },
        }
    }
    res
}

pub fn first(input: &String) -> String {
    let mut current_count: i32 = 0;
    let mut best_count: i32 = 0;

    for i in parse(&input) {
        match i {
            0 => {
                // This is a new elf.
                best_count = cmp::max(best_count, current_count);
                current_count = 0;
            }
            v => {
                current_count += v;
            }
        }
    }

    // Finally return whichever is better between
    return cmp::max(current_count, best_count).to_string();
}

pub fn second(input: &String) -> String {
    let mut res: Vec<i32> = vec![0, 0, 0];

    let mut current_count: i32 = 0;
    for i in parse(&input) {
        match i {
            0 => {
                // This is a new elf.
                res.push(current_count);
                current_count = 0;
            }
            v => {
                current_count += v;
            }
        }
    }
    res.push(current_count);

    res.sort();
    res.reverse();

    res[0..3].iter().sum::<i32>().to_string()
}

#[cfg(test)]
#[test]
fn test() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
        .into();

    {
        let got = first(&input);
        let want = "24000";

        assert_eq!(got, want)
    }
    {
        let got = second(&input);
        let want = "45000";

        assert_eq!(got, want)
    }
}

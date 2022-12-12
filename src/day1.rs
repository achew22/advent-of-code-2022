use std::cmp;
use std::num::IntErrorKind;

pub fn run(input: String) -> String {
    let mut current_count: i32 = 0;
    let mut best_count: i32 = 0;

    for line in input.split('\n') {
        match line.parse::<i32>() {
            Ok(v) => {
                current_count += v;
            }
            Err(e) => match e.kind() {
                IntErrorKind::Empty => {
                    // This is a new elf.
                    best_count = cmp::max(best_count, current_count);
                    current_count = 0;
                }
                _ => {
                    panic!("unable to parse {line}")
                }
            },
        }
    }

    // Finally return whichever is better between
    return cmp::max(current_count, best_count).to_string();
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

    let got = run(input);
    let want = "24000";

    assert_eq!(got, want)
}

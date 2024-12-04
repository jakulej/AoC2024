use regex::Regex;

fn main() {
    let regex_uncorrupted = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let regex_numbers = Regex::new(r"\d{1,3}").unwrap();
    let mut is_enable: bool = true;

    let score = regex_uncorrupted
        .captures_iter(include_str!("input.txt"))
        .map(|captured| {
            //println!("{:?}",captured.get(0).unwrap().as_str());

            let captured_string = captured.get(0).unwrap().as_str();
            match captured_string {
                "do()" => {
                    is_enable = true;
                    0
                }
                "don't()" => {
                    is_enable = false;
                    0
                }
                _ => {
                    if is_enable {
                        regex_numbers
                            .captures_iter(captured_string)
                            .map(|digit| digit.get(0).unwrap().as_str().parse::<usize>().unwrap())
                            .fold(1, |acc, x| acc * x)
                    } else {
                        0
                    }
                }
            }
        })
        .sum::<usize>();
    print!("{:?}", score)
}

fn main() {
    let score = include_bytes!("input.txt")
        .split(|b| b == &b'\n')
        .filter(|raport| {
            let raport_levels: Vec<isize> = std::str::from_utf8(raport)
                .unwrap()
                .split_whitespace()
                .map(|string| string.parse::<isize>().unwrap())
                .collect();

            let first_descending: bool = raport_levels[0] - raport_levels[1] > 0;

            for i in 0..raport_levels.len() - 1 {
                let result = raport_levels[i] - raport_levels[i + 1];

                let is_safe:bool = first_descending == (result>0) && result.abs() >= 1 && result.abs() <=3;
                if !is_safe {
                    return false;
                }
            }
            true
        })
        .count();
    print!("{:?}",score);
}

fn check_safety(raport_levels: Vec<isize>) -> bool {
    let first_descending: bool = raport_levels[0] - raport_levels[1] > 0;

    raport_levels.windows(2).all(|levels| {
        let score = levels[0] - levels[1];
        if !((score > 0) == first_descending && score.abs() >= 1 && score.abs() <= 3) {
            return false;
        }
        true
    })
}

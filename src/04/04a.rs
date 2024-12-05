fn main() {
    let pattern = String::from("XMAS");
    let pattern = pattern.as_bytes();

    let input_file: Vec<Vec<u8>> = include_bytes!("example.txt")
        .split(|b| b == &b'\n')
        .map(|line| line.to_vec())
        .collect();

    let score = input_file.iter().enumerate().map(|(i, line)| {
        line.iter()
            .enumerate()
            .filter(|(_j, letter)| letter == &&pattern[0])
            .map(|(j, _first_letter_of_pattern)| {
                let test = x_matching_patterns(input_file.clone(), pattern, i, j);
                println!("position:{:?},{:?} count:{:?}",i,j,test);
                test
            }).sum::<usize>()
    }).sum::<usize>();
    println!("{:?}",score);
}

fn x_matching_patterns(input_file: Vec<Vec<u8>>, pattern: &[u8], x: usize, y: usize) -> usize {
    let mut matching_patterns_count: usize = 0;
    for i_increment in -1..1 {
        for j_increment in -1..1 {
            if check_matching_pattern_in_direction(
                input_file.clone(),
                pattern,
                x,
                y,
                (i_increment, j_increment),
            ) {
                matching_patterns_count += 1;
            }
        }
    }
    matching_patterns_count
}

fn check_matching_pattern_in_direction(
    input_file: Vec<Vec<u8>>,
    pattern: &[u8],
    y: usize,
    x: usize,
    direction: (isize, isize),
) -> bool {
    pattern
        .iter()
        .enumerate()
        .skip(1)
        .all(|(letter_count, pattern_letter)| {
            let x_letter = (x as isize + direction.1 * letter_count as isize) as usize;
            let y_letter: usize = (y as isize + direction.0 * letter_count as isize) as usize;
            let is_x_y_inccorect:bool = x_letter > input_file[0].len() - 1 || y_letter > input_file.len() - 1;


            if is_x_y_inccorect {
                return false;
            }
            pattern_letter == &(input_file[x_letter][y_letter])
        })
}

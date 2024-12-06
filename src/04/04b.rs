const PATTERN_SKIP_CHAR: u8 = b'.';
const PATTERN_MIDDLE_CHAR: u8 = b'A';
const PATTERNS: [[&str;3];4] = [
    [
        "M.S",
        ".A.",
        "M.S",
    ],
    [
        "M.M",
        ".A.",
        "S.S",
    ],
    [
        "S.M",
        ".A.",
        "S.M",
    ],
    [
        "S.S",
        ".A.",
        "M.M",
    ]
    
];


fn main() {
    let input_file: Vec<Vec<u8>> = include_bytes!("input.txt")
        .split(|b| b == &b'\n')
        .map(|line| line.to_vec())
        .collect();
    let score = input_file
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_x, char)| char == &&PATTERN_MIDDLE_CHAR)
                .map(|(x, _char)| {

                    if PATTERNS.iter().any(|pattern| is_pattern_matching(input_file.clone(), pattern.clone(), (x, y))){
                        1
                    }else{
                        0
                    }

                })
                .sum::<usize>()
        })
        .sum::<usize>();
    print!("{:?}",score);
}
fn is_pattern_matching(
    file: Vec<Vec<u8>>,
    pattern: [&str;3],
    coordinate: (usize, usize),
) -> bool {
 (0..pattern.len()).all(|y_pattern| {
        (0..pattern.len()).all(|x_pattern| {
            
            let x_to_check = (coordinate.0 as isize - 1isize) as usize + x_pattern ;
            let y_to_check = (coordinate.1 as isize - 1isize) as usize + y_pattern;

            let is_xy_not_correct = x_to_check >= file[0].len() - 1 || y_to_check >= file.len();

            if is_xy_not_correct{
                return false;
            }
            
            pattern[y_pattern].as_bytes()[x_pattern] == PATTERN_SKIP_CHAR ||
            pattern[y_pattern].as_bytes()[x_pattern] == file[y_to_check][x_to_check]
        })
    })


}

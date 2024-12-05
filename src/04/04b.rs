fn main() {
    let pattern_skip_char = b'.';
    let pattern = 


    let input_file: Vec<Vec<u8>> = include_bytes!("example.txt")
    .split(|b| b == &b'\n')
    .map(|line| line.to_vec())
    .collect();

    let x = input_file[2].windows(3).filter(|window| window[1] == b'X').count();
    println!("{:?}",x);

}

fn file_to_2d_vec ( file_name: String) -> Vec<Vec<u8>> {
    include_bytes!(file_name)
    .split(|b| b == &b'\n')
    .map(|line| line.to_vec())
    .collect();
}
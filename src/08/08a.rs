use std::collections::HashMap;

fn main() {
    let mut map: Vec<Vec<u8>> = include_str!("input.txt")
        .split("\n")
        .map(|line| line.as_bytes().to_vec())
        .collect();
    let antennas = get_antennas_hashmap(&map);
    antennas.values().into_iter().for_each(|antennas_frequencies| {
        antennas_frequencies.iter().for_each(|first_antenna_cords|{
            antennas_frequencies.iter().for_each(|second_antenna_cords|{
                if !(first_antenna_cords==second_antenna_cords){
                let cords = calculate_antinode(first_antenna_cords, second_antenna_cords);
                if is_on_map(&map[0].len(), &map.len(), &cords){
                    map[cords.1][cords.0] = b'#';
                }
            }
            });
        });
    });

    let result = map.iter().flatten().filter(|b| b == &&b'#').count();

    // map.iter().for_each(|line| {
    //     let string = String::from_utf8(line.clone());
    //     println!("{:?}",string.unwrap());
    // });


    print!("{:?}",result);
}


fn get_antennas_hashmap(map: &Vec<Vec<u8>>) -> HashMap<u8, Vec<(usize, usize)>> {
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();

    map.iter().enumerate().for_each(|(y, line)| {
        line.iter()
            .enumerate()
            .filter(|(_, byte)| byte.is_ascii_alphanumeric())
            .for_each(|(x, alphanumeric)| {
                antennas
                    .entry(*alphanumeric)
                    .or_insert_with(|| {
                        let mut vec: Vec<(usize, usize)> = Vec::new();
                        vec.push((x, y));
                        vec
                    })
                    .push((x, y))
            });
    });
    antennas
}
fn calculate_antinode (first_antenna: &(usize,usize), second_antenna: &(usize,usize)) -> (usize,usize) {
    let vec = (2 * (second_antenna.0 as isize - first_antenna.0 as isize), 2 * (second_antenna.1 as isize - first_antenna.1 as isize));
    let x_antinode = (first_antenna.0 as isize + vec.0) as usize;
    let y_antinode = (first_antenna.1 as isize + vec.1) as usize;
    //println!("{:?}",vec);
    (x_antinode,y_antinode)
}
fn is_on_map(x_lenght: &usize, y_lenght: &usize, coords: &(usize,usize)) -> bool {
    !(coords.0 >= *x_lenght || coords.1 >= *y_lenght)
}

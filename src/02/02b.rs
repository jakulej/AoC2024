fn main() {
    let score = include_bytes!("input.txt")
        .split(|b| b == &b'\n')
        .enumerate()
        .filter(|(number,raport)| {
            let raport_levels: Vec<isize> = std::str::from_utf8(raport)
                .unwrap()
                .split_whitespace()
                .map(|string| string.parse::<isize>().unwrap())
                .collect();
            println!("BEGIN LINE {:?}", number);
            let isSafe:bool = check_safety(raport_levels,0);
            println!("{:?} line is {:?}",number,isSafe);
            isSafe
        })
        .count();
    print!("{:?}", score);
}

fn check_safety(raport_levels: Vec<isize>, level: usize) -> bool {
    if level > 1{
        return false;
    }
    let is_descending: bool = is_descending(raport_levels.clone());
    //println!("is_descending: {:?}",first_descending);

    raport_levels.windows(2).enumerate().all(|(i,levels)| {

        let score = levels[0] - levels[1];
        if !((score > 0) == is_descending && score.abs() >= 1 && score.abs() <= 3) {
            println!("TABLE {:?}",raport_levels);
            let mut raports_left_deleted = raport_levels.clone();
            raports_left_deleted.remove(i);
            println!("CHECKING LEFT {:?}",raports_left_deleted);
            if check_safety(raports_left_deleted, level +1){
                return true;
            }

            let mut raports_right_deleted = raport_levels.clone();
            raports_right_deleted.remove(i + 1);
            println!("CHECKING RIGHT {:?}",raports_right_deleted);
            if check_safety(raports_right_deleted, level +1){
                return true;
            }

            return false;
        }
        true
    })
}
fn is_descending (raport_levels: Vec<isize>) -> bool {
    (0..3).map(|i|{
        raport_levels[i] - raport_levels[i+1]
    }).filter(|level_diff| level_diff > &0).count() > 1

}

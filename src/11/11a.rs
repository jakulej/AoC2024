enum Rules {
    Zero,
    Even,
    Other
}

const BLINK_COUNT:u8 = 25;
fn main () {
    let mut stones:Vec<usize> = include_str!("input.txt").split_whitespace().map(|number| number.parse::<usize>().unwrap()).collect();
    for i in 0..BLINK_COUNT  {
        stones = blink(&stones);
    }
    print!("{}",stones.len())
    
}
fn blink (stones: &Vec<usize>) -> Vec<usize>{
    let mut stones_after_blink = Vec::new();

    stones.iter().for_each(|stone|{
    let mut stone_after_blink = match rule_to_apply(stone) {
            Rules::Zero => Vec::from([1usize]),
            Rules::Even => split_half(stone),
            _ => Vec::from([stone*2024])
            
        };
    stones_after_blink.append(&mut stone_after_blink);
    });

    stones_after_blink
}
fn rule_to_apply (stone: &usize) -> Rules {
    if stone == &0 {
        return Rules::Zero;
    }
    else if is_digit_number_even(stone){
        return Rules::Even;
    }else {
        return Rules::Other;
    }
}
fn is_digit_number_even(stone : &usize) -> bool {
    format!("{}",stone).len() % 2 == 0  
}
fn split_half(stone : &usize) -> Vec<usize> {
    let str = format!("{}",stone);
    let half_index = str.len() / 2;
    let (first,second) = str.split_at(half_index);
    let splitted:Vec<usize> = Vec::from([first.parse::<usize>().unwrap(),second.parse::<usize>().unwrap()]);

    splitted
}
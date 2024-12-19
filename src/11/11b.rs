use std::collections::{HashMap, LinkedList};

enum Rules {
    Zero,
    Even,
    Other
}


const BLINK_COUNT:u16 = 75;

fn main () {
    let input:Vec<usize> = include_str!("input.txt").split_whitespace().map(|number| number.parse::<usize>().unwrap()).collect();
    let mut stones = count_stones(input);
    for i in 0..BLINK_COUNT  {
        stones = blink(&stones);
    }
    let score = stones.values().sum::<usize>();
    print!("{}",score)
    
}
fn blink (stones: &HashMap<usize,usize>) -> HashMap<usize,usize>{
    let mut after_blink:HashMap<usize,usize> = HashMap::new();

    stones.iter().for_each(|(stone,occurence_count)| {
        let stone_after_blink = match rule_to_apply(stone) {
            Rules::Zero => Vec::from([1usize]),
            Rules::Even => split_half(stone),
            _ => Vec::from([stone*2024])
            
        };
        stone_after_blink.iter().for_each(|single_stone| {
            after_blink.entry(*single_stone).and_modify(|count| *count +=occurence_count).or_insert(*occurence_count);
        });
    });

    after_blink

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
fn count_stones(input: Vec<usize>) -> HashMap<usize,usize> {
    let mut  stones:HashMap<usize,usize> = HashMap::new();
    input.iter().for_each(|stone| {
        stones.entry(*stone).and_modify(|count| *count+=1).or_insert(1);
    });
    stones
}
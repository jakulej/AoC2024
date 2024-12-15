use std::{collections::LinkedList};
const FREE_SPACE:usize = usize::MAX;
fn main(){
    let file= include_bytes!("input.txt");
    let disk:Vec<usize> = create_disk_represenation(file).into_iter().collect();
    let formatted_disk = format_disk(disk);
    let score = formatted_disk.iter().enumerate().filter_map(|(i,data)| {
        if *data != FREE_SPACE {
            Some(i* (*data as usize))
        }else{
            None
        }
    }).sum::<usize>();
    println!("{:?}",score);
}
fn multiply_byte(byte: usize, n: usize) -> LinkedList<usize>{
    let mut list:LinkedList<usize> = LinkedList::new(); 
    for _ in 0..n {
        list.push_back(byte);
    }
    list
}
fn create_disk_represenation(file: &[u8]) -> LinkedList<usize> {
    let mut unformated_file:LinkedList<usize> = LinkedList::new(); 
    file.iter().enumerate().for_each(|(index, byte)| {
        let size = (byte - b'0') as usize;
        match index % 2 {
            0 => {
                let mut list: LinkedList<usize> = multiply_byte((index/2) as usize, size);
                unformated_file.append(&mut list);
            },
            1 => {
                let byte:usize = FREE_SPACE;
                let mut list: LinkedList<usize> = multiply_byte(byte, size);
                unformated_file.append(&mut list);
            },
            _ => unreachable!()
        }  
    });
    unformated_file
}
fn format_disk(disk: Vec<usize>) -> Vec<usize> {
    let mut formatted_disk = disk.clone();
    disk.iter().enumerate().filter(|(_,e)| e==&&FREE_SPACE).for_each(|(index, _free_space)| {
        let last_data_pos = formatted_disk.len() - &formatted_disk.iter().rev().position(|b| b !=&FREE_SPACE).unwrap() - 1;
        if last_data_pos>index {
            
            formatted_disk[index] = disk[last_data_pos];
            formatted_disk[last_data_pos] = FREE_SPACE;
        }
    });
    formatted_disk
}
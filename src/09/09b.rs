use std::collections::{BTreeMap, LinkedList};
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
    let data_position = find_data(&disk);
    
    data_position.iter().rev().for_each(|(number,(data_index,data_size))| {
        let free_space = find_empty_space(&formatted_disk);
        let space_to_insert = free_space
        .iter()
        .find(|(free_space_index,free_space_size)| free_space_size>=&data_size && free_space_index< &data_index);

        match space_to_insert{
            Some((index_to_insert,_)) => relocate_data(*index_to_insert,(*number,(*data_index,*data_size)), &mut formatted_disk),
            None => {}
        }

    });

    formatted_disk
}
fn relocate_data(free_space_index: usize, data_info: (usize,(usize,usize)), disk: &mut Vec<usize>){
                                                 //number,(index,lenght)
    let (number,(data_index,data_lenght)) = data_info;
    for i in 0..data_lenght {
        disk[free_space_index+i] = number;
        disk[data_index+i] = FREE_SPACE;
    }
}
fn find_empty_space(disk: &Vec<usize>) -> BTreeMap<usize,usize> {
                                //coords,count
    let mut empty_spaces:BTreeMap<usize, usize> = BTreeMap::new();

    let mut lenght = 0usize;
    disk.iter().enumerate().for_each(|(index, data)|{
        if data == &FREE_SPACE {
            lenght +=1;
        }else if lenght>0 {
            empty_spaces.insert(index-lenght, lenght);
            lenght = 0;
        }

    });
    empty_spaces
}
fn find_data(disk: &Vec<usize>) -> BTreeMap<usize,(usize,usize)> {
                                //number (index,lenght)
    let mut numbers_count:BTreeMap<usize,(usize,usize)> = BTreeMap::new();
    let mut number = 0usize;
    let mut lenght = 0usize;

    disk.iter().enumerate().for_each(|(index, data)| {
        if data == &number {
            lenght+=1;
        }else if lenght>0 {
            numbers_count.insert(number, (index-lenght,lenght));
            lenght = 0;
            if data !=&FREE_SPACE{
                number = *data;
                lenght = 1;
            }
        }else if data!=&FREE_SPACE {
            number = *data;
            lenght = 1;
        }
    });
    numbers_count.insert(number, (disk.len()-lenght,lenght));
    numbers_count
}


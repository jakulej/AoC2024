#[derive(PartialEq, Eq,Clone,Debug)]
enum Operation {
    Add,
    Mul,
    Concatenation
}
impl Operation {
    fn get_result(&self, x: &usize, y: &usize) ->usize{
        match self {
            Operation::Add => x+y,
            Operation::Mul => x*y,
            Operation::Concatenation => {
                let result = format!("{:?}{:?}",x,y);
                result.parse::<usize>().unwrap()
            }
        }
    }
}
fn main(){
    
    let score = include_str!("input.txt")
    .split("\n")
    .filter_map(|line| {
        let (result, numbers) = line.split_once(": ").unwrap();
        let result = result.parse::<usize>().unwrap();

        let numbers:Vec<usize> = numbers.split(" ").map(|number| number.parse::<usize>().unwrap()).collect();

        let operators_table = create_operation_possibities(generate_binary_table(numbers.len() - 1));

        if operators_table.iter().any(|operators_possibility| result == calculate_score(operators_possibility,&numbers)) {
            Some(result)
        }
        else {
            None
        }


    }).sum::<usize>();

    println!("{:?}",score);
}
fn calculate_score(operations: &Vec<Operation>, numbers: &Vec<usize>) ->usize{
    let mut accumulator = numbers[0];
    for i in 0..operations.len() {
        accumulator = operations[i].get_result(&accumulator, &numbers[i+1])
    }
    accumulator
}
fn generate_binary_table(size: usize) -> Vec<Vec<u8>>{
    let lenght = 3usize.pow(size as u32);
    let mut table: Vec<Vec<u8>> = Vec::with_capacity(lenght);
    for i in 0..lenght {
        let bits =  calc_number_to_diffrent_system(3,i);
        let bits_size = bits.len();

        let bits = "0".repeat(size-bits_size) + &bits;
        table.push(bits.as_bytes().to_vec());
    }
    table
}
fn create_operation_possibities(binary_table: Vec<Vec<u8>>) ->Vec<Vec<Operation>> {
    binary_table.iter().map(|line| {
        let line:Vec<Operation> = line.iter().map(|bit| {
            match bit {
                b'0' => Operation::Add,
                b'1' => Operation::Mul,
                b'2' => Operation::Concatenation,
                _ => unreachable!()
            }
        }).collect();
        line
    }).collect()
    
}
fn calc_number_to_diffrent_system(base: usize, mut number: usize) -> String{
    let mut converted_number:Vec<usize> = Vec::new();
    while number>=base {
        converted_number.push(number%base);
        number /= base;
    }
    converted_number.push(number);

    converted_number.reverse();

    converted_number.iter().map(|i| i.to_string() ).collect::<String>()
}
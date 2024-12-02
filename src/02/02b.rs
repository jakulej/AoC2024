fn main() {
    let score = include_bytes!("example.txt")
        .split(|b| b == &b'\n')
        .filter(|raport| {
            let mut raport_levels: Vec<isize> = std::str::from_utf8(raport)
                .unwrap()
                .split_whitespace()
                .map(|string| string.parse::<isize>().unwrap())
                .collect();

            let first_descending: bool = raport_levels[0] - raport_levels[1] > 0;
            let mut is_mistake_happend:bool = false;
            
            for mut i in 0..raport_levels.len() - 1 {
                print!(".");
                let result = raport_levels[i] - raport_levels[i + 1];
                print!(" _{:?}: {:?}",i ,result);
                let is_safe:bool = first_descending == (result>0) && result.abs() >= 1 && result.abs() <=3;

                if !is_safe {
                    print!("!");
                    if is_mistake_happend{
                        print!("_\n");
                        return false;
                    }else{
                        print!("Deleting");
                        is_mistake_happend = true;
                        raport_levels.remove(i + 1);
                        i  = i - 1;
                    }
                }
            }
            print!("Correct \n");
            true
        })
        .count();
    print!("{:?}",score);
}
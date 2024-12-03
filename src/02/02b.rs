fn main() {
    let score = include_bytes!("example2.txt")
        .split(|b| b == &b'\n')
        .filter(|raport| {
            let mut raport_levels: Vec<isize> = std::str::from_utf8(raport)
                .unwrap()
                .split_whitespace()
                .map(|string| string.parse::<isize>().unwrap())
                .collect();

            let first_descending: bool = raport_levels[0] - raport_levels[1] > 0;
            let mut is_mistake_happend:bool = false;
            let mut i = 0;
            while i < raport_levels.len() - 1 {
                print!(".");
                let result = raport_levels[i] - raport_levels[i + 1];
                print!(" _{:?}: {:?}",i ,result);
                let is_safe:bool = first_descending == (result>0) && result.abs() >= 1 && result.abs() <=3;

                if !is_safe {
                    print!("!");
                    if is_mistake_happend{
                        print!("Bad _\n");
                        return false;
                    }else{
                        print!("Deleting");
                        is_mistake_happend = true;
                        if i > 0 {
                            raport_levels.remove(i + 1);
                            i -=1;
                        } else {
                            if !(first_descending == (raport_levels[1] - raport_levels[2] > 0)){
                                raport_levels.remove(0);
                            }
                        }
                    }
                }
                i +=1;
            }
            print!("Correct \n");
            true
        })
        .count();
    print!("{:?}",score);
}

fn is_descending(raport_levels: Vec<isize> ) -> bool{
    
    true
}
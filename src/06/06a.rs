const OBSTACLE: u8 = b'#';
const GUARD: u8 = b'^';
enum GuardFacing {
    Up ,
    Down,
    Left,
    Right
}
impl GuardFacing {
    pub fn get_direction_vec(&self) -> (isize, isize) {
        match self {         //(x,y)
            GuardFacing::Up => (0,-1),
            GuardFacing::Down => (0,1),
            GuardFacing::Left => (-1,0),
            GuardFacing::Right => (1,0)
        }
    }
    pub fn rotate(&self) -> GuardFacing {
        match self {
            GuardFacing::Up => GuardFacing::Right,
            GuardFacing::Down => GuardFacing::Left,
            GuardFacing::Left => GuardFacing::Up,
            GuardFacing::Right => GuardFacing::Down
        }
    }
}


fn main () {
    let mut map: Vec<Vec<u8>> = include_str!("example.txt")
        .split("\n")
        .map(|line| line.as_bytes().to_vec())
        .collect();

    //(x,y)
    let mut guard_coords = find_guard(&map).unwrap();
    let mut guard_facing:GuardFacing = GuardFacing::Up; 

    let mut is_guard_on_map:bool = true;
    

    loop {
        map[guard_coords.1][guard_coords.0] = b'X';
        let x_to_check = (guard_coords.0 as isize + guard_facing.get_direction_vec().0) as usize;
        let y_to_check = (guard_coords.1 as isize + guard_facing.get_direction_vec().1) as usize;
        let cords = (x_to_check,y_to_check);
        if !is_on_map(&cords, &map){
            break;
        }
        if is_obstacle(&map, &cords) {
            guard_facing = guard_facing.rotate();
        }
        else {
            guard_coords = cords;
        }

    }
    let x_sum = map.iter().map(|y| {
        y.iter().filter(|x| x == &&b'X').count()
    }).sum::<usize>();
    print!("{:?}",x_sum);

}

fn find_guard(map: &Vec<Vec<u8>>) -> Option<(usize,usize)> {
    for x in 0..map[0].len() {
       for y in 0..map.len() {
           if map[y][x] == GUARD {
            return Some((x,y));
           }
       } 
    }
    None
}
fn is_obstacle(map: &Vec<Vec<u8>>, position: &(usize,usize)) -> bool {
    map[position.1][position.0] == OBSTACLE 

}

fn is_on_map(cords: &(usize,usize), map: &Vec<Vec<u8>>) -> bool {
   !(cords.0 >= map[0].len() || cords.1 >= map.len())
}

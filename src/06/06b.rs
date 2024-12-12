const OBSTACLE: u8 = b'#';
const GUARD: u8 = b'^';
#[derive(PartialEq, Eq,Clone)]
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
    let mut map: Vec<Vec<u8>> = include_str!("input.txt")
        .split("\n")
        .map(|line| line.as_bytes().to_vec())
        .collect();

    //(x,y)
    let mut guard_coords = find_guard(&map).unwrap();
    let mut guard_facing:GuardFacing = GuardFacing::Up; 

    let mut counter = 0;
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
            
            if map[y_to_check][x_to_check] != b'X'{
                let guard_start_params = (guard_coords,guard_facing.clone());
                if is_loop(&create_map_with_obstacle(&map, &cords), guard_start_params){
                    counter+=1;
                }
            }

            guard_coords = cords;
        }
    }
    print!("{:?}",counter);

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

fn is_loop(map: &Vec<Vec<u8>>, guard_start_params: ((usize,usize),GuardFacing)) -> bool {

    let mut obstacle_metted: Vec<((usize,usize),GuardFacing)> = Vec::new();
    let mut guard_coords = guard_start_params.0.clone();
    let mut guard_facing = guard_start_params.1.clone();

    loop {
        let x_to_check = (guard_coords.0 as isize + &guard_facing.get_direction_vec().0) as usize;
        let y_to_check = (guard_coords.1 as isize + &guard_facing.get_direction_vec().1) as usize;
        let coords = (x_to_check,y_to_check);

        if !is_on_map(&coords, &map){
            return false;
        }
        
        //Move guard
        if is_obstacle(&map, &coords) {

            let obstacle = (guard_coords.clone(),guard_facing.clone());
            if obstacle_metted.contains(&obstacle) {
                return true;
            }
            obstacle_metted.push(obstacle);

            guard_facing = guard_facing.rotate();
            //println!("Rotate,guard cords: {:?}",guard_coords);
        }
        else {
            guard_coords = coords;
        }
        
        //Check if guard returned to start position
        if guard_start_params.0 == guard_coords && guard_start_params.1 == guard_facing {
            return true;
        }
        //println!("Loop ends, guard cords: {:?}",guard_coords);
    }
}
fn create_map_with_obstacle(map: &Vec<Vec<u8>>, obstacle_pos: &(usize,usize)) -> Vec<Vec<u8>> {
    let mut map_with_obstacle = map.clone();
    map_with_obstacle[obstacle_pos.1][obstacle_pos.0] = b'#';
    map_with_obstacle
}
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
        match self {
            GuardFacing::Up => (0,1),
            GuardFacing::Down => (0,-1),
            GuardFacing::Left => (-1,0),
            GuardFacing::Right => (1,0)
        }
    }
}


fn main () {
    let map: Vec<Vec<u8>> = include_bytes!("example.txt")
        .split(|b| b == &b'\n')
        .map(|line| line.to_vec())
        .collect();
    let guard_coords = find_guard(map).unwrap();
    let guard_facing:GuardFacing = GuardFacing::Up; 


}

fn find_guard(map: Vec<Vec<u8>>) -> Option<(usize,usize)> {
    for x in 0..map[0].len() {
       for y in 0..map.len() {
           if map[y][x] == GUARD {
            return Some((x,y));
           }
       } 
    }
    None
}
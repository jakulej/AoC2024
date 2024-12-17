struct Coords {
    x: usize,
    y: usize
}
const END_TRACK:u8 = 9u8;
const BEGIN_TRACK:u8 = 0u8;


fn main () {
    let map: Vec<Vec<u8>> = include_str!("example.txt")
    .split("\n")
    .map(|line| {
        let vec:Vec<u8> = line.as_bytes().iter().map(|b| b - b'0').collect();
        vec
    })
    .collect();




    println!("{:?}",map);
}
fn find_tracks(map:&Vec<Vec<u8>> ,current_coords: Coords, previous_height: usize) -> Option<Vec<Coords>>{


    if map[current_coords.y][current_coords.x] ==  END_TRACK{
        let mut tracks:Vec<Coords> = Vec::new();
        tracks.push(current_coords);
        
        return Some(tracks);
    }

    None
}
fn is_height_increased(actual_height: &usize, previous_height: &usize) -> bool {
    *actual_height as isize - *previous_height as isize == 1 
}
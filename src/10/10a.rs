use std::{ops::Add, u8};

#[derive(PartialEq, Clone, Copy,Debug)]
struct Coords {
    x: usize,
    y: usize,
}
const END_TRACK: u8 = 9u8;
const BEGIN_TRACK: u8 = 0u8;

fn main() {
    let map: Vec<Vec<u8>> = include_str!("input.txt")
        .split("\r\n")
        .map(|line| {
            let vec: Vec<u8> = line.as_bytes().iter().map(|b| b - b'0').collect();
            vec
        })
        .collect();
    let score = map.iter().enumerate().map(|(y, line)| {
        line.iter()
            .enumerate()
            .filter(|(_, number)| number == &&0u8)
            .map(|(x, _)| {
                let current_coords = Coords { x, y };
                count_tracks(&map, current_coords)
            }).sum::<usize>()
    }).sum::<usize>();

    println!("{:?}", score);
}
fn count_tracks(map: &Vec<Vec<u8>>, current_coords: Coords) -> usize {
    let mut tracks: Vec<Coords> = Vec::new();

    find_tracks(map, current_coords, u8::MAX, &mut tracks);

    tracks.iter().count()
}
fn find_tracks(
    map: &Vec<Vec<u8>>,
    current_coords: Coords,
    previous_height: u8,
    tracks: &mut Vec<Coords>,
) -> Option<Coords> {
    if !is_on_map(map, &current_coords) {
        return None;
    }

    let actual_height = map[current_coords.y][current_coords.x];
    if !is_height_increased(&actual_height, &previous_height) {
        return None;
    }

    if actual_height == END_TRACK {
        return add_track(tracks, current_coords);
    }

    neighbors_to_visit(&current_coords)
        .iter()
        .for_each(|neighbour_cords| {
            find_tracks(map, *neighbour_cords, actual_height, tracks);
        });
    None
}
fn neighbors_to_visit(current_coords: &Coords) -> Vec<Coords> {
    let mut neighbors_to_visit: Vec<Coords> = Vec::from([
        current_coords.clone(),
        current_coords.clone(),
        current_coords.clone(),
        current_coords.clone(),
    ]);

    neighbors_to_visit[0].x = neighbors_to_visit[0].x.add(1); // right
    neighbors_to_visit[1].x = neighbors_to_visit[1].x.wrapping_sub(1); //left
    neighbors_to_visit[2].y = neighbors_to_visit[2].y.add(1); //down
    neighbors_to_visit[3].y = neighbors_to_visit[3].y.wrapping_sub(1); //up

    neighbors_to_visit
}
fn add_track(tracks: &mut Vec<Coords>, end_track: Coords) -> Option<Coords> {
    if !tracks.contains(&end_track) {
        tracks.push(end_track);
        return Some(end_track);
    }

    None
}

fn is_height_increased(actual_height: &u8, previous_height: &u8) -> bool {
    (*actual_height as isize - *previous_height as isize == 1) || *previous_height == u8::MAX
}
fn is_on_map(map: &Vec<Vec<u8>>, coords: &Coords) -> bool {
    let x_size = map[0].len();
    let y_size = map.len();

    !(coords.x >= x_size || coords.y >= y_size)
}

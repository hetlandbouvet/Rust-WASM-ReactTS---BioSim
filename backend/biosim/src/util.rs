
use std::{collections::HashMap};

use rand::Rng;

use crate::landscape::{*};

pub fn landscape_from_repr(landscape_repr: HashMap<Coord, String>) -> HashMap<Coord, Option<LandCell>> {
    let mut landscape: HashMap<Coord, Option<LandCell>> = HashMap::new();

    for (coord, letter) in landscape_repr.iter() {
        match letter as &str {
            "W" => landscape.insert(*coord, None),
            "L" => landscape.insert(*coord, Some(LandCell::new(LandCellType::Lowland, *coord, find_neighbors(*coord, &landscape_repr)))),
            "H" => landscape.insert(*coord, Some(LandCell::new(LandCellType::Highland, *coord, find_neighbors(*coord, &landscape_repr)))),
            "D" => landscape.insert(*coord, Some(LandCell::new(LandCellType::Desert, *coord, find_neighbors(*coord, &landscape_repr)))),
            other => panic!("Only W, L, H and D can be map values, got {}", other)
        };
    }
    return landscape;
}

pub fn landscape_repr_from_string(map_string: String) -> HashMap<Coord, String> {
    let mut repr: HashMap<Coord, String> = HashMap::new();
    let split_string = map_string.split("\n");
    let map_rows = split_string.collect::<Vec<&str>>();

    for (i, row) in map_rows.iter().enumerate() {
        let split_row = row.as_bytes();
        for (j, letter_byte) in split_row.iter().enumerate() {
            let letter = String::from_utf8(Vec::from([*letter_byte])).unwrap();
            repr.insert(Coord(i as u32, j as u32), letter);   
        }
    }
    return repr;
}

fn find_neighbors(coord: Coord, landscape_repr: &HashMap<Coord, String>) -> Vec<Coord> {
    let mut land_cell_neighbors: Vec<Coord> = vec!();
    
    let neighbor_coords: Vec<Coord> = vec!(
        Coord(coord.0, coord.1 + 1), 
        Coord(coord.0, coord.1 - 1), 
        Coord(coord.0 + 1, coord.1), 
        Coord(coord.0 - 1, coord.1)
    );

    for c in neighbor_coords {
        if landscape_repr.get(&c).unwrap() != "W" {
            land_cell_neighbors.push(c)
        }
    }

    land_cell_neighbors

}

pub fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    let random: f64 = rng.gen::<f64>();
    assert!(random <= 1. && random > 0.);
    return random;
}

pub fn q(sgn: f64, x: f64, x_half: f64, phi: f64) -> f64 {
    const E: f64 = core::f64::consts::E;
    
    return 1.0 / (1.0 + E.powf(sgn * phi * (x - x_half)));
}
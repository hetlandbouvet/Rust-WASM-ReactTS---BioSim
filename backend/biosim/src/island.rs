use core::panic;
use std::iter::Map;

use rand::rngs::ThreadRng;

use crate::animal::*;
use crate::landscape::*;
use std::str;
use std::fmt;


#[derive(Debug)]
pub struct Island<'a> {
    landscape: Vec<Vec<Option<LandCell<'a>>>>,
    landscape_repr: Vec<Vec<String>>,
    num_carns: u32,
    num_herbs: u32,
    herb_pop_matrix: Vec<Vec<u32>>,
    carn_pop_matrix: Vec<Vec<u32>>,
}


pub fn map_from_string(map_repr: &Vec<Vec<String>>) -> Vec<Vec<Option<LandCell<'static>>>> {
    let mut landscape: Vec<Vec<Option<LandCell>>> = vec!();

    for (x, row) in map_repr.iter().enumerate() {
        let mut landscape_row:Vec<Option<LandCell>> = vec!();
        for (y, letter) in row.iter().enumerate() {
            let letter = letter;
            let coord = Coord(x as i32, y as i32);
            match letter as &str {
                "W" => landscape_row.push(None),
                "L" => landscape_row.push(Some(LandCell::new(&LandCellType::Lowland, coord))),
                "H" => landscape_row.push(Some(LandCell::new(&LandCellType::Highland, coord))),
                "D" => landscape_row.push(Some(LandCell::new(&LandCellType::Desert, coord))),
                other => panic!("Only W, L, H and D can be map values, got {}", other)
            }
        }
        landscape.push(landscape_row);
    }
    return landscape;
}

pub fn map_repr_from_string(map_string: &String) -> Vec<Vec<String>> {
    let mut repr: Vec<Vec<String>> = vec!();
    let split_string = map_string.split("\n");
    let map_rows = split_string.collect::<Vec<&str>>();

    for row in map_rows.iter() {
        let mut row_items: Vec<String> = vec!();
        let split_row = row.as_bytes();
        for letter_byte in split_row.iter() {
            let letter = String::from_utf8(Vec::from([*letter_byte])).unwrap();
            row_items.push(letter);
        }
        repr.push(row_items);   
    }
    return repr;
}

pub trait IslandTraits {
    fn new(map_string: String) -> Island<'static>;
    fn add_animals(&mut self, herb_count: u32, carn_count: u32);
    fn del_animals(&mut self, herb_count: u32, carn_count: u32);
    fn set_neighbors(&mut self);
    fn get_num_animals(&self) -> u32;
    fn get_num_herbs(&self) -> u32;
    fn get_num_carns(&self) -> u32;
    fn get_unique_rows(&self) -> Vec<usize>;
    fn get_unique_cols(&self) -> Vec<usize>;
    fn update_pop_matrix(&mut self);
    fn get_land_cells(&mut self) -> Vec<&LandCell>;
    // fn animal_weight(&self);
    // fn animal_ages(&self);
    // fn animal_fitness(&self);
}

impl IslandTraits for Island<'_> {
    fn new(map_string: String) -> Island<'static> {
        let map_repr = map_repr_from_string(&map_string);
        Island {
            landscape: map_from_string(&map_repr),
            landscape_repr: map_repr,
            num_carns: 0,
            num_herbs: 0,
            herb_pop_matrix: Vec::new(),
            carn_pop_matrix: Vec::new(),
        }
    }
    fn get_land_cells(&mut self) -> Vec<&LandCell> {
        let mut land_cells: Vec<&LandCell> = vec!();

        for (x, row) in self.landscape_repr.iter().enumerate() {
            for (y, letter) in row.iter().enumerate() {
                let letter = letter;
                let coord = Coord(x as i32, y as i32);
                match letter as &str {
                    "W" => continue,
                    "L" => land_cells.push(&LandCell::new(&LandCellType::Lowland, coord)),
                    "H" => land_cells.push(&LandCell::new(&LandCellType::Highland, coord)),
                    "D" => land_cells.push(&LandCell::new(&LandCellType::Desert, coord)),
                    other => panic!("Only W, L, H and D can be map values, got {}", other)
                }
            }
        }
        return land_cells;
    }
    fn add_animals(&mut self, herb_count: u32, carn_count: u32) {
        self.num_herbs += herb_count;
        self.num_carns += carn_count;
    }
    fn del_animals(&mut self, herb_count: u32, carn_count: u32) {
        self.num_herbs -= herb_count;
        self.num_carns -= carn_count;
    }
    fn get_num_animals(&self) -> u32 {
        self.num_carns + self.num_herbs
    }
    fn get_num_carns(&self) -> u32 {
        self.num_carns
    }
    fn get_num_herbs(&self) -> u32 {
        self.num_herbs
    }
    fn get_unique_cols(&self) -> Vec<usize> {
        (0..self.landscape_repr[0].len()).collect()
    }
    fn get_unique_rows(&self) -> Vec<usize> {
        (0..self.landscape_repr.len()).collect()
    }
    fn set_neighbors(&mut self) {
        let landscape_clone = self.landscape_repr.clone();
        for cell in self.get_land_cells()[..].iter_mut() {
            let mut land_neighbors = vec!();
            let location: Coord = cell.get_location();
            let neighbor_coords: Vec<(i32, i32)> = vec!(
                (location.0, location.1 + 1), 
                (location.0, location.1 - 1), 
                (location.0 + 1, location.1), 
                (location.0 - 1, location.1)
            );

            for c in neighbor_coords {
                let x: usize = c.0 as usize;
                let y: usize = c.1 as usize; 
                if landscape_clone[x][y] == String::from("W") {
                    continue
                } else {
                    land_neighbors.push(Coord(c.0, c.1))
                }
            }
            cell.set_land_cell_neighbors(land_neighbors);
        }
    }
    fn update_pop_matrix(&mut self) {
        
    }

}


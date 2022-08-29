
use crate::animal::*;
use crate::island::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;


#[derive(Debug)]
pub enum LandCellType {
    Lowland,
    Highland,
    Desert
}

#[derive(Debug, Copy, Clone)]
pub struct Coord(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct LandCell<'a> {
    fodder: f64,
    max_fodder: f64,
    cell_type: &'a LandCellType,
    location: Coord,
    herbivores: Vec<Herbivore>,
    carnivores: Vec<Carnivore>,
    land_cell_neighbors: Vec<Coord>,
}

#[derive(Debug)]
pub struct AnimalList {
    herbivores: Vec<Herbivore>,
    carnivores: Vec<Carnivore>
}

struct MaxFodder {
    highland: f64,
    lowland: f64,
    desert: f64
}
 

fn get_max_fodder(cell_type: &LandCellType) -> f64 {
    let max_fodder_config: MaxFodder = MaxFodder {highland: 300., lowland: 800., desert: 0.};

    match cell_type {
        LandCellType::Lowland => max_fodder_config.lowland,
        LandCellType::Highland => max_fodder_config.highland,
        LandCellType::Desert => max_fodder_config.desert
    }
}

pub trait CellTraits<'a> {
    fn set_fodder(&mut self, fodder: f64);
    // fn remove_animals(&mut self, animal_list: AnimalList);
    fn get_fodder(&self) -> f64;
    fn get_max_fodder(&self) -> f64;
    fn get_location(&self) -> Coord;
    fn add_animals(&mut self, animal_list: AnimalList);
    fn randomize_herbs(&mut self);
    fn randomize_carns(&mut self);
    fn herb_count(&self) -> usize;
    fn carn_count(&self) -> usize;
    fn set_land_cell_neighbors(&mut self, neighbors: Vec<Coord>);
    fn get_shuffled_herbs(&self) -> Vec<Herbivore>;
    fn get_sorted_carnivores(&self) -> Vec<Carnivore>;
    fn is_empty(&self) -> bool;
}

impl LandCell<'_> {
    pub fn new<'a>(cell_type: &'static LandCellType, location: Coord) -> LandCell<'static> {
        LandCell { 
            cell_type,
            location,
            fodder: get_max_fodder(&cell_type),
            max_fodder: get_max_fodder(&cell_type),
            herbivores: Vec::new(),
            carnivores: Vec::new(),
            land_cell_neighbors: Vec::new(),
        }
    }
}

impl CellTraits<'_> for LandCell<'_> {

    fn get_fodder(&self) -> f64 {
        return self.fodder;
    }
    fn get_location(&self) -> Coord {
        self.location
    }
    fn get_max_fodder(&self) -> f64 {
        return self.max_fodder;
    }
    fn add_animals(&mut self, animal_list: AnimalList) {
        for herb in animal_list.herbivores {
            self.herbivores.push(herb);
        }
        for carn in animal_list.carnivores {
            self.carnivores.push(carn);
        }
    }
    fn carn_count(&self) -> usize {
        self.carnivores.len()
    }
    fn herb_count(&self) -> usize {
        self.herbivores.len()
    }
    fn is_empty(&self) -> bool {
        self.fodder == 0.
    }
    fn randomize_herbs(&mut self) {
        let mut rng = thread_rng();
        self.herbivores.shuffle(&mut rng);
    }
    fn randomize_carns(&mut self) {
        let mut rng = thread_rng();
        self.carnivores.shuffle(&mut rng);
    }
    // fn remove_animals(&mut self, animal_list: AnimalList) {
    //     for herb in animal_list.herbivores {
    //         self.herbivores.drop(herb);
    //     }
    //     for carn in animal_list.carnivores {
    //         self.carnivores.push(carn);
    //     }
    // }
    fn set_fodder(&mut self, fodder: f64) {
        self.fodder = fodder;
    }
    fn set_land_cell_neighbors(&mut self, neighbors: Vec<Coord>) {
        self.land_cell_neighbors = neighbors;
    }
    fn get_sorted_carnivores(&self) -> Vec<Carnivore> {
        let mut sorted_carns = self.carnivores.clone();
        sorted_carns
    }
    fn get_shuffled_herbs(&self) -> Vec<Herbivore> {
        let mut shuffled_herbs = self.herbivores.clone();
        let mut rng = rand::thread_rng();
        shuffled_herbs.shuffle(&mut rng);
        shuffled_herbs
    }
}


impl fmt::Display for LandCell<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} ({:?})",
            self.cell_type, self.location
        )
    }
}

#[derive(Debug)]
pub struct WaterCell;

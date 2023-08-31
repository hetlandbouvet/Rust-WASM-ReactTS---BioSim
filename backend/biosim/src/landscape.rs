
use std::{fmt, collections::HashMap};

use rand::seq::SliceRandom;

use crate::animal::{*};


#[derive(Debug, Copy, Clone)]
pub enum LandCellType {
    Lowland,
    Highland,
    Desert
}

#[derive(Debug, Clone)]
pub struct AnimalList {
    pub herbivores: Vec<Herbivore>,
    pub carnivores: Vec<Carnivore>
}


impl AnimalList {
    pub fn animal_count(&self) -> AnimalCount {
        return AnimalCount { herbivores: self.herbivores.len() as u32, carnivores: self.carnivores.len() as u32 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AnimalCount {
    pub herbivores: u32,
    pub carnivores: u32
}

impl AnimalCount {
    pub fn total(&self) -> u32 {
        self.herbivores + self.carnivores
    }
    pub fn update(&mut self, animal_count: AnimalCount) {
        self.herbivores += animal_count.herbivores;
        self.carnivores += animal_count.carnivores;
    }
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Eq)]
pub struct Coord(pub u32, pub u32);

#[derive(Debug, Clone)]
pub struct LandCell {
    fodder: f64,
    max_fodder: f64,
    cell_type: LandCellType,
    location: Coord,
    herbivores: Vec<Herbivore>,
    carnivores: Vec<Carnivore>,
    land_cell_neighbors: Vec<Coord>
}


struct MaxFodder {
    highland: f64,
    lowland: f64,
    desert: f64
}


fn get_max_fodder(cell_type: LandCellType) -> f64 {
    let max_fodder_config: MaxFodder = MaxFodder {highland: 300., lowland: 700., desert: 0.};

    match cell_type {
        LandCellType::Lowland => max_fodder_config.lowland,
        LandCellType::Highland => max_fodder_config.highland,
        LandCellType::Desert => max_fodder_config.desert
    }
}

impl LandCell {
    pub fn new(cell_type: LandCellType, location: Coord, land_cell_neighbors: Vec<Coord>) -> Self {
        let max_fodder = get_max_fodder(cell_type);
        Self { 
            cell_type,
            location,
            max_fodder,
            land_cell_neighbors,
            fodder: max_fodder,
            herbivores: Vec::new(),
            carnivores: Vec::new(),
        }
    }
    pub fn feeding(&mut self) -> u32 {
        let shuffled_herbs: &mut Vec<Herbivore> = self.herbivores.as_mut();
        shuffled_herbs.shuffle(&mut rand::thread_rng());
        for herb in shuffled_herbs.iter_mut() {
            let fodder_left = herb.eat_fodder(self.fodder);
            self.fodder = fodder_left;
        }

        let mut herbs_killed_total: u32 = 0;
        if self.herbivores.len() > 0 {
            let herbivores: &mut Vec<Herbivore> = self.herbivores.as_mut();
            herbivores.sort_by(|a, b| a.health.fitness.partial_cmp(&b.health.fitness).unwrap());
            let carnivores: &mut Vec<Carnivore> = &mut self.carnivores.as_mut();
            carnivores.sort_by(|a, b| a.health.fitness.partial_cmp(&b.health.fitness).unwrap().reverse());
            for carn in self.carnivores.iter_mut() {
                let herbs_killed = carn.kill_prey(self.herbivores.as_mut());
                herbs_killed_total += herbs_killed;
                self.herbivores.retain(| herb | herb.is_alive());
            }
        }
        herbs_killed_total
    }

    pub fn aging(&mut self) {
        for herb in self.herbivores.iter_mut() {
            herb.aging();
        }
        for carn in self.carnivores.iter_mut() {
            carn.aging();
        }
    }

    pub fn lose_weight(&mut self) {
        for herb in self.herbivores.iter_mut() {
            herb.lose_weight();
        }
        for carn in self.carnivores.iter_mut() {
            carn.lose_weight();
        }
    }

    pub fn death(&mut self) -> AnimalCount {
        let mut dead_herbs: u32 = 0;
        let mut dead_carns: u32 = 0;
        for herb in self.herbivores.iter_mut() {
            herb.death();
            if !herb.health.is_alive {
                dead_herbs += 1;
            }
        }
        for carn in self.carnivores.iter_mut() {
            carn.death();
            if !carn.health.is_alive {
                dead_carns += 1;
            }
        }
        self.herbivores.retain(| herb | herb.is_alive());
        self.carnivores.retain(| carn | carn.is_alive());
        
        AnimalCount { herbivores: dead_herbs, carnivores: dead_carns }
    }

    pub fn procreate(&mut self) -> AnimalCount {
        let n_herbs = self.herb_count();
        let n_carns = self.carn_count();
        let mut new_animals = AnimalList { herbivores: vec!(), carnivores: vec!() };
        for herb in self.herbivores.iter_mut() {
            let birth_result = herb.give_birth(n_herbs);
            if birth_result.given_birth {
                new_animals.herbivores.push(Herbivore::new(birth_result.birth_weight.unwrap(), 0));
            }
        }
        for carn in self.carnivores.iter_mut() {
            let birth_result = carn.give_birth(n_carns);
            if birth_result.given_birth {
                new_animals.carnivores.push(Carnivore::new(birth_result.birth_weight.unwrap(), 0));
            }
        }
        let animal_count = new_animals.animal_count();
        self.add_animals(new_animals);
        animal_count
    }

    pub fn migrate(&mut self) -> Option<HashMap<Coord, AnimalList>>{
        let neighbors = &self.land_cell_neighbors;
        if neighbors.len() > 0 {
            let mut move_map: HashMap<Coord, AnimalList> = HashMap::new();  // Make AnimalList
            // Migrate herbivores
            for herb in self.herbivores.iter_mut() {
                if herb.migrate() {
                    let mut cloned_herb = herb.clone();
                    cloned_herb.health.has_moved = false;
                    let selected_coord = neighbors.choose(&mut rand::thread_rng()).unwrap();
                    if move_map.contains_key(&selected_coord) {
                        move_map.get_mut(&selected_coord).unwrap().herbivores.push(cloned_herb)
                    } else {
                        move_map.insert(*selected_coord, AnimalList{herbivores: vec!(cloned_herb), carnivores: vec!()});
                    }
                }
            }
            // Migrate carnivores
            for carn in self.carnivores.iter_mut() {
                if carn.migrate() {
                    let mut cloned_carn = carn.clone();
                    cloned_carn.health.has_moved = false;
                    let selected_coord = neighbors.choose(&mut rand::thread_rng()).unwrap();
                    if move_map.contains_key(&selected_coord) {
                        move_map.get_mut(&selected_coord).unwrap().carnivores.push(cloned_carn)
                    } else {
                        move_map.insert(*selected_coord, AnimalList{herbivores: vec!(), carnivores: vec!(cloned_carn)});
                    }
                }
            }
            return Some(move_map);
        }
        None
    }

    pub fn remove_moved(&mut self) {
        self.carnivores.retain(|carn| !carn.health.has_moved);
        self.herbivores.retain(|herb| !herb.health.has_moved);
    }

    pub fn get_location(&self) -> Coord {
        self.location
    }
    pub fn reset_fodder(&mut self) {
        self.fodder = self.max_fodder;
    }
    pub fn carn_count(&self) -> u32 {
        self.carnivores.len() as u32
    }
    pub fn herb_count(&self) -> u32 {
        self.herbivores.len() as u32
    }
    pub fn animal_count(&self) -> AnimalCount {
        let herb_count = self.herb_count();
        let carn_count = self.carn_count();
        AnimalCount { herbivores: herb_count, carnivores: carn_count }
    }
    pub fn carn_avg_age(&self) -> f64 {
        let mut age_sum: f64 = 0.;
        if self.carn_count() > 0 {
            for carn in self.carnivores.iter() {
                age_sum += carn.health.age as f64;
            }
            return age_sum / self.carn_count() as f64;
        }
        0.
    }
    pub fn carn_avg_weight(&self) -> f64 {
        let mut weight_sum: f64 = 0.;
        if self.carn_count() > 0 {
            for carn in self.carnivores.iter() {
                weight_sum += carn.health.weight as f64;
            }
            return weight_sum / self.carn_count() as f64;
        }
        0.
    }
    pub fn herb_avg_age(&self) -> f64 {
        let mut age_sum: f64 = 0.;
        if self.herb_count() > 0 {
            for herb in self.herbivores.iter() {
                age_sum += herb.health.age as f64;
            }
            return age_sum / self.herb_count() as f64;
        }
        0.
    }
    pub fn add_animals(&mut self, animal_list: AnimalList) {
        for herb in animal_list.herbivores {
            self.herbivores.push(herb)
        }
        for carn in animal_list.carnivores {
            self.carnivores.push(carn)
        }
    }
}


impl fmt::Display for LandCell {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} ({:?})",
            self.cell_type, self.location
        )
    }
}


use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::landscape::*;
use crate::util::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimYear {
    pub year: u32,
    pub num_herbs: u32,
    pub num_carns: u32
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SimResult {
    pub res: Vec<SimYear>
}

#[derive(Debug)]
pub struct BioSim {
    landscape: HashMap<Coord, Option<LandCell>>,
    animal_count: AnimalCount
}


impl<'a> BioSim {
    pub fn new(map_str: String) -> Self {
        let landscape_repr = landscape_repr_from_string(map_str);
        let landscape = landscape_from_repr(landscape_repr);
        Self {
            landscape,
            animal_count: AnimalCount { herbivores: 0, carnivores: 0 }
        }
    }

    pub fn run_year_cycle(&mut self) {
        let mut move_maps: HashMap<Coord, HashMap<Coord, AnimalList>> = HashMap::new();
        for (_, cell) in self.landscape.iter_mut() {
            if cell.is_some() {
                let cell_ref = cell.as_mut().unwrap();
                if cell_ref.animal_count().total() > 0 {
                    cell_ref.reset_fodder();
                    cell_ref.feeding();
                    cell_ref.procreate();
                    cell_ref.aging();
                    cell_ref.lose_weight();
                    cell_ref.death();
                    let move_map = cell_ref.migrate();
                    if move_map.is_some() {
                        move_maps.insert(cell_ref.get_location(), move_map.unwrap());
                    }
                }
            }
        }
        // Move animals
        for (origin, move_map) in move_maps {
            for (destination, animal_list) in move_map {
                let origin_cell = self.landscape.get_mut(&origin).unwrap().as_mut();
                origin_cell.unwrap().remove_moved();
                let destination_cell = self.landscape.get_mut(&destination).unwrap().as_mut();
                destination_cell.unwrap().add_animals(animal_list);
            }
        }
        let mut animal_count = AnimalCount { herbivores: 0, carnivores: 0 };
        for (_, cell) in self.landscape.iter() {
            if cell.is_some() {
                animal_count.update(cell.as_ref().unwrap().animal_count());
            }
        }

        self.set_animal_count(animal_count);
    }



    pub fn get_num_animals(&self) -> u32 {
        self.animal_count.total()
    }
    pub fn get_animal_count(&self) -> AnimalCount {
        self.animal_count
    }
    pub fn get_num_carns(&self) -> u32 {
        self.animal_count.carnivores
    }
    pub fn get_num_herbs(&self) -> u32 {
        self.animal_count.herbivores
    }
    fn set_animal_count(&mut self, new_count: AnimalCount) {
        // Should use struct
        self.animal_count = new_count
    }


    pub fn herb_pop_matrix(&self) -> HashMap<Coord, u32> {
        let mut matrix: HashMap<Coord, u32> = HashMap::new();
        for (coord, cell) in self.landscape.iter() {
            if cell.is_some() {
                let ref_cell = cell.as_ref().unwrap();
                matrix.insert(*coord, ref_cell.herb_count());
            }
        }
        matrix
    }

    pub fn carn_pop_matrix(&self) -> HashMap<Coord, u32> {
        let mut matrix: HashMap<Coord, u32> = HashMap::new();
        for (coord, cell) in self.landscape.iter() {
            if cell.is_some() {
                let ref_cell = cell.as_ref().unwrap();
                matrix.insert(*coord, ref_cell.carn_count());
            }
        }
        matrix
    }
    pub fn carn_avg_age(&self) -> f64 {
        let mut avg_age_sum: f64 = 0.;
        let mut cell_count: u32 = 0;

        for (_, cell) in self.landscape.iter() {
            if cell.is_some() {
                cell_count += 1;
                avg_age_sum += cell.as_ref().unwrap().carn_avg_age();
            }
        }
        avg_age_sum / cell_count as f64
    }
    pub fn carn_avg_weight(&self) -> f64 {
        let mut avg_weight_sum: f64 = 0.;
        let mut cell_count: u32 = 0;

        for (_, cell) in self.landscape.iter() {
            if cell.is_some() {
                cell_count += 1;
                avg_weight_sum += cell.as_ref().unwrap().carn_avg_weight();
            }
        }
        avg_weight_sum / cell_count as f64
    }
    pub fn herb_avg_age(&self) -> f64 {
        let mut avg_age_sum: f64 = 0.;
        let mut cell_count: u32 = 0;
        for (_, cell) in self.landscape.iter() {
            if cell.is_some() {
                cell_count += 1;
                avg_age_sum += cell.as_ref().unwrap().herb_avg_age();
            }
        }
        avg_age_sum / cell_count as f64
    }
    pub fn add_animals(&mut self, animal_map: HashMap<Coord, AnimalList>) {
        for (coord, list) in animal_map {
            let animal_count = list.animal_count();
            self.set_animal_count(animal_count);
            match self.landscape.get_mut(&coord) {
                Some(cell) => cell.as_mut().unwrap().add_animals(list),
                None => panic!("Incorrect coordinates")
            }
        }
    }
}


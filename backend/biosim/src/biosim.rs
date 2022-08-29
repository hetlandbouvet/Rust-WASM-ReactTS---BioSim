use rand::rngs::ThreadRng;

use crate::animal::*;
use crate::island::*;
use crate::landscape::*;

#[derive(Debug)]
pub struct BioSim<'a> {
    island: Island<'a>,
}

pub trait BioSimTraits {
    fn new(map_str: String) -> BioSim<'static>;
    fn run_year_cycle(&mut self);
}

fn feeding(cell: &mut LandCell) {
        
    // Reset cell fodder
    cell.set_fodder(cell.get_max_fodder());

    // Randomize herbs
    let shuffled_herbs = cell.get_shuffled_herbs();

    for mut herb in shuffled_herbs {
        herb.eat_fodder(cell);
    }
}

impl BioSimTraits for BioSim<'_> {
    fn new(map_str: String) -> BioSim<'static>{
        BioSim {
            island: Island::new(map_str),
        }
    }

        // Feed herbs


        // Sort carnivores

        // Feed carnivores
    fn run_year_cycle(&mut self) {
        for land_cell in self.island.get_land_cells() {
            println!("{:?}", land_cell);
        }
    }
}

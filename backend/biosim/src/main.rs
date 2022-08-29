mod animal;
mod landscape;
mod island;
mod biosim;

use biosim::*;
use animal::*;
use landscape::*;
use island::*;
use std::{fmt, cmp};

fn main() {

    let map_string = String::from("WWWW\nWLLW\nWWWW");
    let mut sim = BioSim::new(map_string);
    sim.run_year_cycle();
    println!("{:#?}", sim);
    // let mut island = Island::new(&map_string);
    // island.set_neighbors();
    // println!("{:#?}", island);
    // let mut carn_1 = Carnivore::new(20.,1);
    // let mut carn_2 = Carnivore::new(20., 4);
    // let mut carn_3 = Carnivore::new(20., 2);
    // let mut carn_list = vec!(carn_1, carn_2, carn_3);
    // println!("{:#?}", carn_list);

}

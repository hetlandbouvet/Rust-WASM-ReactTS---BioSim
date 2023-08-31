mod animal;
mod landscape;
mod biosim;
mod util;

use biosim::*;
use animal::*;
use landscape::*;
use std::{collections::HashMap};
use std::fs::File;
use plotters::prelude::*;


fn main() {
    let mut biosim = BioSim::new(String::from(
"\
WWWWWWWWWWWWWWWWWWWWW\n\
WWWWWWWWHWWWWLLLLLLLW\n\
WHHHHHLLLLWWLLLLLLLWW\n\
WHHHHHHHHHWWLLLLLLWWW\n\
WHHHHHLLLLLLLLLLLLWWW\n\
WHHHHHLLLDDLLLHLLLWWW\n\
WHHLLLLLDDDLLLHHHHWWW\n\
WWHHHHLLLDDLLLHWWWWWW\n\
WHHHLLLLLDDLLLLLLLWWW\n\
WHHHHLLLLDDLLLLWWWWWW\n\
WWHHHHLLLLLLLLWWWWWWW\n\
WWWHHHHLLLLLLLWWWWWWW\n\
WWWWWWWWWWWWWWWWWWWWW"
// "\
// WWWW\n\
// WLLW\n\
// WWWW
// "
    ));

    let mut sim_result: SimResult = SimResult{ res: Vec::new() };

    let mut animal_map = HashMap::new();
    let mut herb_init: Vec<Herbivore> = vec!();
    let carn_init: Vec<Carnivore> = vec!();

    let sim_len: f64 = 200.;

    for _ in 0..10 {
        herb_init.push(Herbivore::new(2., 5));
    }
    // for _ in 0..2 {
    //     carn_init.push(Carnivore::new(20., 5));
    // }
    animal_map.insert(Coord(10, 10), AnimalList{ 
        herbivores: herb_init, 
        carnivores: carn_init
    });
    biosim.add_animals(animal_map);

    let root_drawing_area = BitMapBackend::new("0.1.png", (2000, 1000))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
    .build_cartesian_2d(0 as f64..sim_len, 0 as f64..20_000.)
    .unwrap();

    println!("Total herbs: {}", biosim.get_num_herbs());
    for y in 0..sim_len as u32 {
    
        if y % 10 == 0 {
            chart.draw_series(LineSeries::new(
                sim_result.res.iter().map(|x| (x.year as f64, x.num_herbs as f64)),
                &RED
            )).unwrap();
            sim_result.res.push(SimYear { year: y, num_herbs: biosim.get_num_herbs(), num_carns: biosim.get_num_carns()});
            ::serde_json::to_writer(&File::create("../../client/public/result.json").unwrap(), &sim_result).unwrap();
            println!("--- saved ---");

        }
        println!("--- {} ---", y);
        println!("Total: {:?}", biosim.get_num_animals());
        println!("{:?}", biosim.get_animal_count());
        biosim.run_year_cycle();
    }
}

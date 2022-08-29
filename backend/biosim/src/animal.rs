
use probability::prelude::*;
use rand::prelude::*;
use std::fmt;

use crate::landscape::*;

#[derive(Debug, Clone)]
struct P {
    w_birth: f64,
    sigma_birth: f64,
    beta: f64,
    eta: f64,
    a_half: f64,
    phi_age: f64,
    w_half: f64,
    phi_weight: f64,
    mu: f64,
    gamma: f64,
    zeta: f64,
    xi: f64,
    omega: f64,
    f: f64,
    delta_phi_max: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Health {
    weight: f64,
    pub age: u8,
    has_moved: bool,
    death_prob: Option<f64>,
    _fitness: Option<f64>,
    _fitness_valid: Option<bool>,
}

// Animal
#[derive(Debug)]
pub struct BirthResult {
    given_birth: bool,
    birth_weight: Option<f64>,
}

fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    let random: f64 = rng.gen();
    assert!(random <= 1. && random > 0.);
    return random;
}

fn q(sgn: f64, x: f64, x_half: f64, phi: f64) -> f64 {
    const E: f64 = core::f64::consts::E;
    
    return 1.0 / (1.0 + E.powf(sgn * phi * (x - x_half)));
}
pub trait AnimalTraits<T> {
    fn new(weight: f64, age: u8) -> Self;

    fn aging(&mut self) {
        match self {
            Herbivore => self.health.age += 1,
        }
    }
    fn migrate(&mut self) -> bool {
        let move_prob = self.p.mu * self.fitness();

        if random_float() < move_prob {
            true
        } else {
            false
        }
    }
    fn birth_weight(&self) -> f64 {
        let mut source = source::default();
        let distribution = Gaussian::new(self.p.w_birth, self.p.sigma_birth);
        let sampler = Independent(&distribution, &mut source);
        let samples = sampler.take(1).collect::<Vec<_>>();
        return *samples.get(0).unwrap();
    }
    fn death(&mut self) -> bool {
        if self.health.weight <= 0. {
            true
        } else {
            let death_prob = self.p.omega * (1. - self.fitness());
            if random_float() < death_prob {
                true
            } else {
                false
            }
        }
    }
    fn fitness(&mut self) -> f64 {
        if self.health._fitness == None || self.health._fitness_valid == Some(false) {
            self.health._fitness = Some(
                q(1., self.health.age as f64, self.p.a_half, self.p.phi_age)
                    * q(-1., self.health.weight, self.p.w_half, self.p.phi_weight),
            );
            self.health._fitness_valid = Some(true);
        }
        self.health._fitness.unwrap()
    }
    fn lose_weight(&mut self) {
        self.health.weight -= self.health.weight * self.p.eta;
        self.health._fitness_valid = Some(false);
    }
    fn give_birth(&mut self, n_same: u32) -> BirthResult {
        let mut give_birth = false;

        let birth_prob = self.p.gamma * self.fitness() * (n_same - 1) as f64;

        if self.health.weight < (self.p.zeta * (self.p.w_birth + self.p.sigma_birth)) {
            return BirthResult {
                given_birth: give_birth,
                birth_weight: None,
            };
        } else if birth_prob >= 1. {
            give_birth = true;
        } else if birth_prob > 0. && birth_prob < 1. {
            if random_float() < birth_prob {
                give_birth = true;
            } else {
                give_birth = false
            }
        } else {
            give_birth = false
        }

        if give_birth {
            let birth_weight = Some(self.birth_weight());
            if birth_weight.unwrap() < self.health.weight {
                self.health.weight -= self.p.xi * birth_weight.unwrap();
                self.health._fitness_valid = Some(false);
                return BirthResult {
                    given_birth: true,
                    birth_weight: Some(birth_weight).unwrap(),
                };
            } else {
                return BirthResult {
                    given_birth: false,
                    birth_weight: None,
                };
            }
        } else {
            return BirthResult {
                given_birth: false,
                birth_weight: None,
            };
        }
    }
}

// Carnivore
#[derive(Debug, Clone)]
pub struct Carnivore {
    health: Health,
    p: P,
}

impl Carnivore {
    pub fn kill_prey(&mut self, sorted_herbivores: Vec<Carnivore>) -> Vec<Carnivore> {
        let mut consumption_weight = 0.;
        let mut herbs_killed: Vec<Carnivore> = Vec::new();
        let fitness = self.fitness();

        for herb in sorted_herbivores {
            if consumption_weight < self.p.f {
                let mut kill_prey = false;
                let fitness_diff = fitness - herb.health._fitness.unwrap();
                
                if fitness_diff > 0. && fitness_diff < self.p.delta_phi_max.unwrap() {
                    let kill_prob = fitness_diff / self.p.delta_phi_max.unwrap();
                    if random_float() <= kill_prob {
                        kill_prey = true;
                    }
                } else {
                    kill_prey = true;
                }

                if kill_prey {
                    self.health._fitness_valid = Some(false);
                    consumption_weight += herb.health.weight;
                    herbs_killed.push(herb);
                }
            }
        }
        if consumption_weight > self.p.f {
            consumption_weight = self.p.f;
        }

        self.health.weight += consumption_weight * self.p.beta;

        return herbs_killed;
    }
}

impl AnimalTraits<Carnivore> for Carnivore {
    fn new(weight: f64, age: u8) -> Self {
        Self {
            health: Health {
                weight,
                age,
                has_moved: false,
                death_prob: None,
                _fitness: None,
                _fitness_valid: None,
            },
            p: P {
                w_birth: 6.0,
                sigma_birth: 1.0,
                beta: 0.75,
                eta: 0.125,
                a_half: 40.0,
                phi_age: 0.3,
                w_half: 4.0,
                phi_weight: 0.4,
                mu: 0.4,
                gamma: 0.8,
                zeta: 3.5,
                xi: 1.1,
                omega: 0.8,
                f: 50.0,
                delta_phi_max: Some(10.0),
            },
        }
    }
}

impl fmt::Display for Carnivore {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Carnivore (Age: {:?}, Weight: {:?}, Fitness: {:?})",
            self.health.age, self.health.weight, self.health._fitness
        )
    }
}

// HERBIVORES

#[derive(Debug, Clone)]
pub struct Herbivore {
    health: Health,
    p: P,
}


impl Herbivore {
    pub fn eat_fodder(&mut self, cell: &mut LandCell) {
        let consumption_amount = self.p.f;
        
        if consumption_amount < cell.get_max_fodder() {
            self.health.weight += self.p.beta * consumption_amount;
            cell.set_fodder(cell.get_fodder() - consumption_amount);
        } else if consumption_amount > cell.get_max_fodder() && cell.get_fodder() > 0. {
            self.health.weight += self.p.beta * cell.get_fodder();
            cell.set_fodder(0.);
        }
        self.health._fitness_valid = Some(false);
    }
}


impl AnimalTraits<Herbivore> for Herbivore {
    fn new(weight: f64, age: u8) -> Self {
        Self {
            health: Health {
                weight,
                age,
                has_moved: false,
                death_prob: None,
                _fitness: None,
                _fitness_valid: None,
            },
            p: P {
                w_birth: 8.0,
                sigma_birth: 1.5,
                beta: 0.9,
                eta: 0.05,
                a_half: 40.0,
                phi_age: 0.1,
                w_half: 10.0,
                phi_weight: 0.1,
                mu: 0.25,
                gamma: 0.2,
                zeta: 3.5,
                xi: 1.2,
                omega: 0.4,
                f: 10.0,
                delta_phi_max: None,
            },
        }
    }
}

impl fmt::Display for Herbivore {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Herbivore (Age: {:?}, Weight: {:?}, Fitness: {:?})",
            self.health.age, self.health.weight, self.health._fitness
        )
    }
}
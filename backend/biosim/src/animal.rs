use probability::prelude::*;
use std::fmt;
use crate::util::*;

// ----------------------------------------------------
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub is_alive: bool,
    pub weight: f64,
    pub age: u32,
    pub has_moved: bool,
    pub fitness: f64,
    fitness_valid: bool,
}

// Animal
#[derive(Debug)]
pub struct BirthResult {
    pub given_birth: bool,
    pub birth_weight: Option<f64>,
}

// ----------------------------------------------------

pub trait AnimalTraits {
    fn aging(&mut self);
    fn migrate(&mut self) -> bool;
    fn birth_weight(&self) -> f64;
    fn death(&mut self);
    fn get_fitness(&mut self) -> f64;
    fn set_fitness(&mut self);
    fn lose_weight(&mut self);
    fn give_birth(&mut self, n_same: u32) -> BirthResult;
    fn is_alive(&self) -> bool;
}

// ----------------------------------------------------
// Carnivore
#[derive(Debug, Clone, Copy)]
pub struct Carnivore {
    pub health: Health,
    p: P,
}

impl<'a> Carnivore {
    pub fn new(weight: f64, age: u32) -> Self {
        let mut new_carn = Self {
            health: Health {
                weight,
                age,
                is_alive: true,
                has_moved: false,
                fitness: 0.,
                fitness_valid: true,
            },
            p: P {
                w_birth: 6.0,
                sigma_birth: 1.0,
                beta: 0.75,
                eta: 0.125,
                a_half: 70.0,
                phi_age: 0.5,
                w_half: 4.0,
                phi_weight: 0.4,
                mu: 0.4,
                gamma: 0.8,
                zeta: 3.5,
                xi: 1.1,
                omega: 0.3,
                f: 6.0,
                delta_phi_max: Some(9.0),
            },
        };
        new_carn.set_fitness();
        new_carn
    }
    pub fn kill_prey(&mut self, sorted_herbivores: &mut Vec<Herbivore>) -> u32 {
        let mut consumption_weight = 0.;
        let mut herbs_killed: u32 = 0;
        for herb in sorted_herbivores.iter_mut() {
            if consumption_weight < self.p.f {
                let mut kill_prey = false;
                let fitness_diff = self.get_fitness() - herb.get_fitness();

                if fitness_diff > 0. && fitness_diff < self.p.delta_phi_max.unwrap() {
                    let kill_prob = fitness_diff / self.p.delta_phi_max.unwrap();
                    if random_float() <= kill_prob {
                        kill_prey = true;
                    }
                } else {
                    kill_prey = true;
                }

                if kill_prey {
                    herbs_killed += 1;
                    self.health.fitness_valid = false;
                    consumption_weight += herb.health.weight;
                    herb.kill();
                }
            }
        }
        if consumption_weight > self.p.f {
            consumption_weight = self.p.f;
        }

        self.health.weight += consumption_weight * self.p.beta;
        herbs_killed
    }
}

impl AnimalTraits for Carnivore {
    fn aging(&mut self) {
        self.health.age += 1
    }
    fn migrate(&mut self) -> bool {
        let move_prob = self.p.mu * self.get_fitness();

        if random_float() < move_prob {
            self.health.has_moved = true;
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
    fn death(&mut self) {
        if self.health.weight <= 0. {
            self.health.is_alive = false;
        } else {
            let death_prob = self.p.omega * (1. - self.get_fitness());
            if random_float() < death_prob {
                self.health.is_alive = false;
            }
        }
    }
    fn get_fitness(&mut self) -> f64 {
        if self.health.fitness == 0. || self.health.fitness_valid == false {
            self.health.fitness = q(1., self.health.age as f64, self.p.a_half, self.p.phi_age)
                * q(-1., self.health.weight, self.p.w_half, self.p.phi_weight);
            self.health.fitness_valid = true;
        }
        self.health.fitness
    }
    fn set_fitness(&mut self) {
        self.health.fitness = q(1., self.health.age as f64, self.p.a_half, self.p.phi_age)
            * q(-1., self.health.weight, self.p.w_half, self.p.phi_weight);
    }
    fn lose_weight(&mut self) {
        self.health.weight -= self.health.weight * self.p.eta;
        self.health.fitness_valid = false;
    }
    fn give_birth(&mut self, n_same: u32) -> BirthResult {
        let mut give_birth = false;

        let birth_prob = self.p.gamma * self.get_fitness() * (n_same - 1) as f64;

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
                self.health.fitness_valid = false;
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
    fn is_alive(&self) -> bool {
        self.health.is_alive
    }
}

impl fmt::Display for Carnivore {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Carnivore (Age: {:?}, Weight: {:?}, Fitness: {:?})",
            self.health.age, self.health.weight, self.health.fitness
        )
    }
}

// ----------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Herbivore {
    pub health: Health,
    p: P,
}

impl<'a> Herbivore {
    pub fn new(weight: f64, age: u32) -> Self {
        let mut new_herb = Self {
            health: Health {
                weight,
                age,
                is_alive: true,
                has_moved: false,
                fitness: 0.,
                fitness_valid: true,
            },
            p: P {
                w_birth: 8.0,
                sigma_birth: 1.5,
                beta: 0.9,
                eta: 0.05,
                a_half: 40.0,
                phi_age: 0.6,
                w_half: 10.0,
                phi_weight: 0.1,
                mu: 0.25,
                gamma: 0.2,
                zeta: 3.2,
                xi: 1.8,
                omega: 0.4,
                f: 10.0,
                delta_phi_max: None,
            },
        };
        new_herb.set_fitness();
        new_herb
    }
    pub fn eat_fodder(&mut self, fodder: f64) -> f64 {
        let mut fodder_left = fodder;
        let consumption_amount = self.p.f;
        if consumption_amount <= fodder {
            self.health.weight += self.p.beta * consumption_amount;
            fodder_left -= consumption_amount;
        } else if consumption_amount > fodder && fodder > 0. {
            self.health.weight += self.p.beta * fodder;
            fodder_left = 0.;
        }
        self.health.fitness_valid = false;

        fodder_left
    }
    pub fn kill(&mut self) {
        self.health.is_alive = false;
    }
}

impl AnimalTraits for Herbivore {
    fn aging(&mut self) {
        self.health.age += 1;
    }
    fn migrate(&mut self) -> bool {
        let move_prob = self.p.mu * self.get_fitness();

        if random_float() < move_prob {
            self.health.has_moved = true;
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
    fn death(&mut self) {
        if self.health.weight <= 0. {
            self.health.is_alive = false;
        } else {
            let death_prob = self.p.omega * (1. - self.get_fitness());
            if random_float() < death_prob {
                self.health.is_alive = false;
            }
        }
    }
    fn get_fitness(&mut self) -> f64 {
        if self.health.fitness == 0. || self.health.fitness_valid == false {
            self.health.fitness = q(1., self.health.age as f64, self.p.a_half, self.p.phi_age)
                * q(-1., self.health.weight, self.p.w_half, self.p.phi_weight);
            self.health.fitness_valid = true;
        }
        self.health.fitness
    }
    fn set_fitness(&mut self) {
        self.health.fitness = q(1., self.health.age as f64, self.p.a_half, self.p.phi_age)
            * q(-1., self.health.weight, self.p.w_half, self.p.phi_weight);
    }
    fn lose_weight(&mut self) {
        self.health.weight -= self.health.weight * self.p.eta;
        self.health.fitness_valid = false;
    }
    fn give_birth(&mut self, n_same: u32) -> BirthResult {
        let mut give_birth = false;

        let birth_prob = self.p.gamma * self.get_fitness() * (n_same - 1) as f64;

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
                self.health.fitness_valid = false;
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

    fn is_alive(&self) -> bool {
        self.health.is_alive
    }
}

impl fmt::Display for Herbivore {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Herbivore (Age: {:?}, Weight: {:?}, Fitness: {:?})",
            self.health.age, self.health.weight, self.health.fitness
        )
    }
}

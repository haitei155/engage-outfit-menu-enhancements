use unity::prelude::*;

#[unity::class("App", "Random")]
pub struct Random {
    pub seed1: u32,
    pub seed2: u32,
    pub seed3: u32,
    pub seed4: u32,
}

impl Random {
    // Game's default rngs
    pub fn get_combat() -> &'static Random { unsafe { random_get_combat(None)} }
    pub fn get_game() -> &'static Random { unsafe { random_get_game(None)} }
    pub fn get_system() -> &'static Random { unsafe { random_get_system(None) } }
    pub fn get_kill_bonus() -> &'static Random { unsafe { random_get_kill_bonus(None)} }
    pub fn get_spot() -> &'static Random { unsafe { random_get_spot(None)} }
    pub fn get_hub() -> &'static Random { unsafe { random_get_hub(None) } }
    pub fn get_hub_item() -> &'static Random { unsafe { random_get_hub_item(None) } }

    // Get a random value 
    pub fn get_value(&self, num: i32) -> i32 {
        unsafe { random_get_value(self, num, None) }
    }

    // random value between min to max
    pub fn get_min_max(&self, min: i32, max: i32) -> i32 {
        unsafe { random_get_min_max(self, min, max, None) }
    }

    pub fn probability_100(&self, percent: f32) -> bool {
        unsafe { random_is_prob_100(self, percent, None) }
    }
    pub fn ctor(&self, seed: u32){
        unsafe { random_ctor_seed(self, seed, None); }
    }
    pub fn initialize(&self, value: u32) {
        unsafe { random_initialize(self, value, None); }
    }
    pub fn value(&self) -> i32 {
        unsafe { random_get_value2(self, None)}
    }

}
#[unity::from_offset("App", "Random", "get_Game")]
fn random_get_game(method_info: OptionalMethod) -> &'static Random;

#[unity::from_offset("App", "Random", "get_Combat")]
fn random_get_combat(method_info: OptionalMethod) -> &'static Random;

#[unity::from_offset("App", "Random", "get_Hub")]
fn random_get_hub(method_info: OptionalMethod) -> &'static Random;

#[unity::from_offset("App", "Random", "get_HubItem")]
fn random_get_hub_item(method_info: OptionalMethod) -> &'static Random;

#[unity::from_offset("App", "Random", "get_KillBonus")]
fn random_get_kill_bonus(method_info: OptionalMethod) -> &'static Random;

#[unity::from_offset("App", "Random", "get_System")]
fn random_get_system(method_info: OptionalMethod) -> &'static Random;

#[unity::from_offset("App", "Random", "get_Spot")]
fn random_get_spot(method_info: OptionalMethod) -> &'static Random;

#[skyline::from_offset(0x02375170)]
fn random_get_value(this: &Random, num: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x023751b0)]
fn random_get_min_max(this: &Random, min: i32, max: i32, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Random", "IsProbability100")]
fn random_is_prob_100(this: &Random, percent: f32, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02374900)]
fn random_initialize(this: &Random, value: u32, method_info: OptionalMethod);

#[skyline::from_offset(0x02374f10)]
fn random_ctor_seed(this: &Random, value: u32, method_info: OptionalMethod);

#[skyline::from_offset(0x023748d0)]
fn random_get_value2(this: &Random, method_info: OptionalMethod) -> i32;

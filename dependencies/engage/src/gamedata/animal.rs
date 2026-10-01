pub use unity::prelude::*;
use super::*;

#[unity::class("App", "AnimalData")]
pub struct AnimalData {}
impl Gamedata for AnimalData {}

impl AnimalData {
    pub fn can_capture(&self) -> bool { unsafe { is_capture_animal(self, None) }}
    pub fn increment_capture_by_amount(&self, amount: i32) { unsafe { inc_animal_capture_num(self, amount, None); }}
    pub fn increment_capture(&self) { unsafe { inc_animal_capture_num(self, 1, None); }}
    pub fn get_by_pid(pid: &Il2CppString) -> Option<&AnimalData> { unsafe { get_animal_by_pid(pid, None) } }
}

#[unity::from_offset("App", "AnimalData", "GetByPID")]
fn get_animal_by_pid(pid: &Il2CppString, method_info: OptionalMethod) -> Option<&AnimalData>;

#[unity::from_offset("App", "HubUtil", "IsCaptureAnimal")]
fn is_capture_animal(animial: &AnimalData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubUtil", "IncAnimalCaptureNum")]
fn inc_animal_capture_num(animal: &AnimalData, number: i32, method_info: OptionalMethod);
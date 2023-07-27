use crate::{states::drone_state::{Quality, SwarmState}, vector::Vector};

use super::inventory::Inventory;

#[derive(Debug, Clone)]
pub struct Drone {
    position: Vector,
    fuel: Quality,
    inventory: Inventory,
    swarm_state: SwarmState
}


impl Drone {
    pub fn get_swarm_state(&self) -> SwarmState {
        self.swarm_state.clone()
    }
}
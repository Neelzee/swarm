use crate::drone_lib::drone::Drone;

use super::drone_state::{ExternalDroneState, DroneResource, Quality, SwarmState};

pub struct EDStateContext<'a> {
    drone: &'a Drone
}

#[derive(Debug, Clone)]
pub struct EDStateMachine {
    start_state: ExternalDroneState,

    current_state: ExternalDroneState
}


impl EDStateMachine {
    pub fn new() -> EDStateMachine {
        todo!()
    }

    pub fn from_state(current_state: ExternalDroneState) -> EDStateMachine {
        EDStateMachine { start_state: ExternalDroneState::Idle, current_state }
    }

    pub fn change_state(&mut self, context: EDStateContext) -> EDStateMachine {
        match self.current_state {
            ExternalDroneState::Scavenging(dr) => match dr {
                DroneResource::Fuel(q) => todo!(),

                DroneResource::Expansion(q) => todo!(),

                DroneResource::Replication(q) => todo!(),
            }

            ExternalDroneState::Consuming(dr) => match dr {
                DroneResource::Fuel(q) => self.clone(),

                DroneResource::Expansion(q) => match context.drone.get_swarm_state() {
                    SwarmState::Starving => EDStateMachine::from_state(ExternalDroneState::Scavenging(DroneResource::Fuel(Quality::Poor))),

                    SwarmState::Defending | SwarmState::Attacking => EDStateMachine::from_state(ExternalDroneState::Consuming(DroneResource::Fuel(Quality::Poor))),
                    
                    SwarmState::Chilling => self.clone(),
                }

                DroneResource::Replication(q) => todo!(),
            }

            ExternalDroneState::Idle => match context.drone.get_swarm_state() {
                SwarmState::Starving => EDStateMachine::from_state(ExternalDroneState::Scavenging(DroneResource::Fuel(Quality::Poor))),

                SwarmState::Defending | SwarmState::Attacking => EDStateMachine::from_state(ExternalDroneState::Consuming(DroneResource::Fuel(Quality::Poor))),

                SwarmState::Chilling => self.clone(),  // No change
            }
        }
    }
}
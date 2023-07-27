#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Quality {
    Poor,
    Good,
    Great,
    Best
}

#[derive(Debug, Clone, Copy)]
pub enum DroneResource {
    Fuel(Quality),
    Expansion(Quality),
    Replication(Quality)
}

#[derive(Debug, Clone, Copy)]
pub enum ExternalDroneState {
    Scavenging(DroneResource),
    Consuming(DroneResource),
    Idle
}

#[derive(Debug, Clone, Copy)]
pub enum InternalDroneState {
    Scavenging(DroneResource),
    Building(DroneResource),
    Replicating(DroneResource),
    Scanning(DroneResource),
    Idling
}


#[derive(Debug, Clone, Copy)]
pub enum SwarmState {
    Starving,
    Defending,
    Attacking,
    Chilling
}
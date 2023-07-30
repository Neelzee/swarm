module Drone.Drone where

import Data.Map (Map)

data DroneMood = Hungry | Content

data Drone = Drone {
	id::Int
	, name::String
	, Position::[Int]
	, internalMap::Map [Int] Tile
	, droneMood::DroneMood
	, inventory::Inventory
}


type Inventory = [(TileType, Int)]
 
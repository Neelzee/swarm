module World.Board where

import Data.Map ( Map )

data Board = Board {
  dimensions::[Int]
  , board::Map [Int] Tile
}

data Quality = Poor | Good | Great

data TileType =
  Food Quality
  , Resource Quality
  , Block

data Tile = Tile {
  type::Maybe TileType
}
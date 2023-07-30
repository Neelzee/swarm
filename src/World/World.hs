module World.World where

import World.Board (Board)

data World = World {
  season::Season
  , temperature::Temperature
  , precipitation::Precipitation
  , board::Board
}

data Season = Spring | Summer | Fall | Winter

data Severity = High | Medium | Low

newtype Temperature = Temperature Severity

newtype Precipitation = Precipitation Severity
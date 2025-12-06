module Advent.Y2017.Day12 (day12a, day12b) where

import Data.Graph       (Graph, components, graphFromEdges, reachable)
import Data.List.Split  (splitOn)

day12a, day12b :: String -> String
day12a input = show $ length $ reachable (graph input) 0
day12b input = show $ length $ components (graph input)

graph :: String -> Graph
graph input = let (g, _, _) = graphFromEdges (map parse $ lines input) in g

parse :: String -> (Int, Int, [Int])
parse line = (i, i, connections)
    where (x:xs:_)    = splitOn " <-> " line
          connections = map read $ splitOn "," xs
          i           = read x

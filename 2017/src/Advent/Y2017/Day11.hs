module Advent.Y2017.Day11 (day11a, day11b) where

import Data.List.Split (splitOn)
import Math.Geometry.Vector
import Math.Geometry.Distance

type Move = String

day11a, day11b :: String -> String
day11a input = show $ solve1 $ splitOn "," input
day11b input = show $ solve2 $ splitOn "," input

solve1 :: [Move] -> Int
solve1 moves = chebyshev origin (foldl move origin moves)

solve2 :: [Move] -> Int
solve2 moves = maximum $ map (chebyshev origin) (scanl move origin moves)

move :: Vector -> Move -> Vector
move v "nw" = plus v [-1,  1,  0]
move v "n"  = plus v [ 0,  1, -1]
move v "ne" = plus v [ 1,  0, -1]
move v "sw" = plus v [-1,  0,  1]
move v "s"  = plus v [ 0, -1,  1]
move v "se" = plus v [ 1, -1,  0]
move _ _    = error "Unable to process move"
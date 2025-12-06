module Advent.Y2017.Day14 (day14a, day14b) where

import Advent.Y2017.Day10   (knotHash)
import Data.Graph           (components, graphFromEdges)
import Data.Maybe           (isJust, fromJust)
import Text.Printf          (printf)

day14a, day14b :: String -> String
day14a input = show $ length . filter (== '1') $ concat $ buildGrid input
day14b input = show $ length . components $ g
    where (g, _, _) = graphFromEdges $ graphEntries $ buildGrid input

buildGrid :: String -> [String]
buildGrid input = map (knotHashToBinary . knotHash) (map (\x -> input ++ ('-' : show x)) gridSide)

knotHashToBinary :: [Int] -> String
knotHashToBinary input = concat $ map (printf "%08b") $ input

gridSize :: Int
gridSize = 127

gridSide :: [Int]
gridSide = [0..gridSize]

graphEntries :: [String] -> [(Int, Int, [Int])]
graphEntries grid = map fromJust $ filter isJust $ map (graphEntry grid) (map cellLoc [0..((gridSize+1)*(gridSize+1) - 1)])

graphEntry :: [String] -> (Int,Int) -> Maybe (Int, Int, [Int])
graphEntry grid (x,y)
    | not $ isActive grid (x,y) = Nothing
    | otherwise               = Just (cellId (x,y), cellId (x,y), map cellId activeNeighbours)
    where activeNeighbours = filter (isActive grid) (neighbours (x,y))

cellId :: (Int,Int) -> Int
cellId (x,y) = x + y * (gridSize+1)

cellLoc :: Int -> (Int,Int)
cellLoc n = (n `mod` (gridSize+1), n `div` (gridSize+1))

isActive :: [String] -> (Int, Int) -> Bool
isActive grid (x,y) = grid !! y !! x == '1'

neighbours :: (Int,Int) -> [(Int,Int)]
neighbours (x,y) = filter withinGrid [(x, y-1), (x-1, y), (x+1, y), (x, y+1)]

withinGrid :: (Int,Int) -> Bool
withinGrid (x,y) = x >= 0 && x <= gridSize && y >= 0 && y <= gridSize

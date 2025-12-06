module Advent.Y2017.Day03 (day03a, day03b) where

import Data.List
import Data.Maybe
import Math.Geometry.Vector
import Math.Geometry.Distance

day03a, day03b :: String -> String
day03a input = show $ part1 $ read input
day03b input = show $ part2 $ read input

type SpiralGrid = [Vector]

data Direction = U | D | R | L deriving (Show, Eq)
data Move      = Move { direction :: Direction , steps :: Int } deriving (Show)
data Square    = Square { point :: Vector, value :: Int } deriving (Show)

originSquare :: Square
originSquare = Square { point = origin, value = 1 }

part1 :: Int -> Int
part1 n = manhattan origin (last (take n generateSpiralGrid))

generateSpiralGrid :: SpiralGrid
generateSpiralGrid = concat $ scanl applyMove [origin] generateMoves

applyMove :: [Vector] -> Move -> [Vector]
applyMove points move = walk move (last points)

walk :: Move -> Vector -> [Vector]
walk move v
    | d == U = map (\i -> plus  v [0, i]) [1..n]
    | d == D = map (\i -> minus v [0, i]) [1..n]
    | d == R = map (\i -> plus  v [i, 0]) [1..n]
    | d == L = map (\i -> minus v [i, 0]) [1..n]
    | otherwise = error "Unable to process move"
    where d = direction move
          n = steps move

generateMoves :: [Move]
generateMoves = iterate getNextMove initialMove

initialMove :: Move
initialMove = Move R 1

getNextMove :: Move -> Move
getNextMove previousMove
    | previousDirection == R = Move U previousSteps
    | previousDirection == U = Move L (previousSteps + 1)
    | previousDirection == L = Move D previousSteps
    | previousDirection == D = Move R (previousSteps + 1)
    | otherwise = error "Unable to process move"
    where
          previousDirection = direction previousMove
          previousSteps     = steps previousMove

part2 :: Int -> Int
part2 n = value . fromJust
            $ find (\x' -> value x' > n)
            $ generateSpiralGrid' (drop 1 $ generateSpiralGrid) [originSquare]

generateSpiralGrid' :: [Vector] -> [Square] -> [Square]
generateSpiralGrid' [] _ = []
generateSpiralGrid' (x':xs) prevSquares = newPoint : generateSpiralGrid' xs (newPoint : prevSquares)
    where newPoint = (getNextSquare x' prevSquares)

getNextSquare :: Vector -> [Square] -> Square
getNextSquare p prev = Square { point = p
                              , value = sum . map value $ (filter (\s -> neighbours p (point s)) prev)
                              }

neighbours :: Vector -> Vector -> Bool
neighbours a b = (diffX == 0 || diffX == 1) && (diffY == 0 || diffY == 1)
    where (diffX:diffY:_) = absolute $ minus a b

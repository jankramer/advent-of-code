module Advent.Y2017.Day01 (day01a, day01b) where

import Data.Char

day01a, day01b :: String -> String
day01a input = show $ solve (map digitToInt input) 1
day01b input = show $ solve (map digitToInt input) (length input `div` 2)

solve :: [Int] -> Int -> Int
solve digits offset = sum . map fst . filter matches . generatePairs offset $ digits

generatePairs :: Int -> [a] -> [(a, a)]
generatePairs offset input  = zip (map getFirstDigit range) (map getSecondDigit range)
    where range             = [0..length input-1]
          getFirstDigit i   = input !! i
          getSecondDigit i  = cycle input !! (i + offset)

matches :: Eq a => (a, a) -> Bool
matches (x, y) = x == y

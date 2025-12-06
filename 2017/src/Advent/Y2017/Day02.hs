module Advent.Y2017.Day02 (day02a, day02b) where

import Data.List (sort)

day02a, day02b :: String -> String
day02a input = show $ solve rowChecksum1 $ buildSpreadsheet input
day02b input = show $ solve rowChecksum2 $ buildSpreadsheet input

solve :: ([Int] -> Int) -> [[Int]] -> Int
solve checksumFn spreadsheet = sum . map checksumFn $ spreadsheet

buildSpreadsheet :: (Read a, Ord a) => String -> [[a]]
buildSpreadsheet input = map (reverse . sort . map read . words) . lines $ input

rowChecksum1 :: [Int] -> Int
rowChecksum1 values = head values - last values

rowChecksum2 :: [Int] -> Int
rowChecksum2 []        = error "Unable to compute checksum on empty row"
rowChecksum2 (x:xs)
    | null divisibles = rowChecksum2 xs
    | otherwise       = x `div` head divisibles
    where divisibles  = filter (\y -> x `mod` y == 0) xs

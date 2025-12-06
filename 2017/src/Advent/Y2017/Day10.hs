module Advent.Y2017.Day10 (day10a, day10b, knotHash) where

import Data.Bits        (xor)
import Data.Char        (ord)
import Data.List.Split  (splitOn)
import Text.Printf      (printf)

day10a, day10b :: String -> String
day10a input = show $ product $ take 2 $ solve list (map read $ splitOn "," input) 0 0
day10b input = concat $ map formatHex $ knotHash input

knotHash :: String -> [Int]
knotHash input = calcHash $ solve list lengths 0 0
    where lengths = concat . replicate 64 $ map ord input ++ [17,31,73,47,23]

listSize :: Int
listSize = 256

list :: [Int]
list        = [0 .. (listSize-1)]

solve :: [Int] -> [Int] -> Int -> Int -> [Int]
solve l []     _   _    = l
solve l (y:ys) pos skip = solve newList ys newPos (skip + 1)
  where newList = go l pos y
        newPos  = (pos + y + skip) `mod` listSize

go :: [Int] -> Int -> Int -> [Int]
go l pos len = begin ++ end
  where (end, begin)    = splitAt (listSize - pos) $ (reverse sublist) ++ rest
        (sublist, rest) = splitAt len $ take listSize $ drop pos $ cycle l

calcHash :: [Int] -> [Int]
calcHash x
  | length x > 16 = hash : calcHash (drop 16 x)
  | otherwise     = [hash]
  where hash = foldl1 xor $ take 16 x

formatHex :: Int -> String
formatHex x = printf "%02x" x

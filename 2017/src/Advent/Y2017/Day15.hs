module Advent.Y2017.Day15 where

import Data.Bits
import Data.Maybe (fromJust, isJust)
import Text.Read (readMaybe)

type Generator = Int -> Int

factors, moduli :: (Int, Int)
factors = (16807, 48271)
moduli  = (4, 8)

divisor :: Int
divisor = 2147483647

day15a, day15b :: String -> String
day15a x = show $ countMatches (generators1 factors)        40000000 $ parse x
day15b x = show $ countMatches (generators2 factors moduli) 5000000  $ parse x

countMatches :: (Generator, Generator) -> Int -> (Int, Int) -> Int
countMatches generators n startingValues = length
    $ filter rightBitsMatch $ take n
    $ pairs generators startingValues

rightBitsMatch :: (Int, Int) -> Bool
rightBitsMatch (a, b) = (a .&. 0xFFFF) == (b .&. 0xFFFF)

pairs :: (Generator, Generator) -> (Int, Int) -> [(Int, Int)]
pairs (genA, genB) (startA, startB) = zip (iterate genA startA) (iterate genB startB)

generators1 :: (Int, Int) -> (Generator, Generator)
generators1 (aFactor, bFactor) = (next1 aFactor, next1 bFactor)

next1 :: Int -> Generator
next1 factor prev = (prev * factor) `mod` divisor

generators2 :: (Int, Int) -> (Int, Int) -> (Generator, Generator)
generators2 (aFactor, bFactor) (aMod, bMod) = (next2 aFactor aMod, next2 bFactor bMod)

next2 :: Int -> Int -> Generator
next2 factor modulus prev
  | next `mod` modulus == 0 = next
  | otherwise               = next2 factor modulus next
  where next = (prev * factor) `mod` divisor

-- >>> parse "Generator A starts with 123\nGenerator B starts with 456"
-- (123,456)
parse :: String -> (Int, Int)
parse input = (a, b)
  where (a:b:_) = [fromJust x | x <- map readMaybe $ words input, isJust x]

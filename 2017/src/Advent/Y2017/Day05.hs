module Advent.Y2017.Day05 (day05a, day05b) where

import           Data.Sequence (Seq)
import qualified Data.Sequence as Sequence

day05a, day05b :: String -> String
day05a input = show $ countSteps (+1) 0 0 (instructions input)
day05b input = show $ countSteps (\x -> if x>= 3 then x - 1 else x + 1) 0 0 (instructions input)

instructions :: Read a => String -> Seq a
instructions input = Sequence.fromList $ map read $ lines input

countSteps :: (Int -> Int) -> Int -> Int -> Seq Int -> Int
countSteps newPositionFn counter position instr
    | position < 0 || position >= Sequence.length instr = counter
    | otherwise = countSteps
        newPositionFn
        (counter+1)
        (position + currentInstruction)
        (Sequence.update position (newPositionFn currentInstruction) instr)
    where currentInstruction = instr `Sequence.index` position

module Advent.Y2017.Day08 (day08a, day08b) where

import           Data.Char          (toTitle)
import           Data.List          (intercalate, maximum)
import           Data.List.Split    (splitOn)
import           Data.Map           (Map)
import qualified Data.Map as Map

type Registers   = Map Register Int
type Register    = String
data Operation   = Inc Int | Dec Int deriving (Read)
data Instruction = Instruction Register Operation Condition
data Condition   = Condition String (Int -> Bool)

day08a, day08b :: String -> String
day08a input = show $ last $ maximums (map read $ lines input)
day08b input = show $ maximum $ maximums (map read $ lines input)

maximums :: [Instruction] -> [Int]
maximums instructions = [foldl max 0 x | x <- (reductions instructions), x /= Map.empty]

reductions :: [Instruction] -> [Registers]
reductions instructions = scanl processInstruction Map.empty instructions

processInstruction :: Registers -> Instruction -> Registers
processInstruction regs (Instruction key operation condition) =
    if conditionHolds regs condition
       then updateRegister regs key operation
       else regs

conditionHolds :: Registers -> Condition -> Bool
conditionHolds regs (Condition cmpTo cmpFn) = cmpFn (curVal cmpTo regs)

updateRegister :: Registers -> Register -> Operation -> Registers
updateRegister regs key op = Map.insert key (newVal op) regs
  where
    newVal (Inc n) = (curVal key regs) + n
    newVal (Dec n) = (curVal key regs) - n

curVal :: Register -> Registers -> Int
curVal = Map.findWithDefault 0

instance Read Instruction where
  readsPrec _ str = [(Instruction register operation condition, "")]
    where (x:xs)    = words str                                                  -- ("a":["inc","1","if","bb","<","5"])
          (y:ys)    = splitOn ["if"] xs                                          -- (["inc","1"]:[["bb","<","5"]])
          (z:zs)    = (head ys)                                                  -- ("bb":["<","5"])
          register  = x
          operation = read $ titleCase $ intercalate " " y
          condition = Condition z (readComparison (head zs) (read $ last zs))

readComparison :: Ord a => String -> (a -> a -> Bool)
readComparison "<"  = \x y -> (<)   y x
readComparison "<=" = \x y -> (<=)  y x
readComparison "==" = \x y -> (==)  y x
readComparison "!=" = \x y -> (/=)  y x
readComparison ">=" = \x y -> (>=)  y x
readComparison ">"  = \x y -> (>)   y x
readComparison x    = error $ "Unknown comparison operator: " ++ x

titleCase :: String -> String
titleCase [] = []
titleCase (x:xs) = toTitle x : xs
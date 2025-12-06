module Advent.Y2017.Day18 where

import           Data.Char       (isAlpha)
import           Data.Map        (Map)
import qualified Data.Map as Map
import           Data.Maybe      (fromJust, isJust)
import           Data.Sequence   (Seq)
import qualified Data.Sequence as Seq

day18a, day18b :: String -> String
day18a input = show $ execute (0, Map.empty, Nothing) (Seq.fromList $ map words $ lines input)
day18b input = input -- todo

type Registers = Map String Int
type Instructions = Seq [String]

execute :: (Int, Registers, Maybe Int) -> Instructions -> Int
execute a@(pos, _, rcv) instructions
    | head instruction == "rcv" && isJust rcv = fromJust rcv
    | otherwise                               = execute (nPos, nRegs, nRcv) instructions
    where instruction = Seq.index instructions pos
          (nPos, nRegs, nRcv) = update a instruction

update :: (Int, Registers, Maybe Int) -> [String] -> (Int, Registers, Maybe Int)
update (pos, regs, _) ("snd":x:_)   = (pos+1, regs, if xVal > 0 then Just xVal else Nothing)
    where xVal = resolve regs x
update (pos, regs, rcv) ("set":x:y:_) = (pos+1, Map.insert x (resolve regs y) regs, rcv)
update (pos, regs, rcv) ("add":x:y:_) = (pos+1, Map.insert x (resolve regs x + resolve regs y) regs, rcv)
update (pos, regs, rcv) ("mul":x:y:_) = (pos+1, Map.insert x (resolve regs x * resolve regs y) regs, rcv)
update (pos, regs, rcv) ("mod":x:y:_) = (pos+1, Map.insert x (resolve regs x `mod` resolve regs y) regs, rcv)
update (pos, regs, rcv) ("jgz":x:y:_) = (nPos, regs, rcv)
    where nPos = if resolve regs x > 0 then pos + (read y) else pos + 1
update _ _ = error "Could not parse instruction"

resolve :: Registers -> String -> Int
resolve regs x = if all isAlpha x
                   then Map.findWithDefault 0 x regs
                   else read x

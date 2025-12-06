module Advent.Y2017.Day06 (day06a, day06b) where

import              Data.Maybe
import              Data.List
import              Data.Sequence (Seq)
import qualified    Data.Sequence as Sequence

type ReallocationStack  = [BlockAllocation]
type BlockAllocation    = Seq MemoryBank
data MemoryBank         = MemoryBank Index NumBlocks deriving (Show, Eq)
type Index              = Int
type NumBlocks          = Int
instance Ord MemoryBank where
    (MemoryBank posA valA) `compare` (MemoryBank posB valB)
        | valA > valB || (valA == valB && posA < posB) = GT
        | otherwise                                    = LT

day06a, day06b :: String -> String
day06a input = show $ length $ stack input
day06b input = show $ succ $ fromJust $ let (x:xs) = (stack input) in x `elemIndex` xs

stack :: String -> ReallocationStack
stack input = head $ dropWhile (not . headReoccurs) (iterate pushNextReallocation [initialAllocation])
    where initialAllocation = readInitialAllocation input

readInitialAllocation :: String -> BlockAllocation
readInitialAllocation input = Sequence.mapWithIndex (\i x -> MemoryBank i x) (Sequence.fromList numBlocks)
    where numBlocks = map read $ words input

pushNextReallocation :: ReallocationStack -> ReallocationStack
pushNextReallocation [] = error "Unable to push on empty stack"
pushNextReallocation a@(current:_) = (reallocate current) : a

reallocate :: BlockAllocation -> BlockAllocation
reallocate alloc = redistBlocks initial maxBankValue ((maxBankIndex+1) `mod` (length alloc))
    where (MemoryBank maxBankIndex maxBankValue) = maximum alloc
          initial = Sequence.update maxBankIndex (MemoryBank maxBankIndex 0) alloc

redistBlocks :: BlockAllocation -> NumBlocks -> Int -> BlockAllocation
redistBlocks currentAlloc remainingBlocks position
    | remainingBlocks == 0 = currentAlloc
    | otherwise            = redistBlocks nextAlloc (remainingBlocks-1) ((position + 1) `mod` length currentAlloc)
    where nextValue = let (MemoryBank _ x) = Sequence.index currentAlloc position in succ x
          nextAlloc = Sequence.update position (MemoryBank position nextValue) currentAlloc

headReoccurs :: Eq a => [a] -> Bool
headReoccurs [] = False
headReoccurs (x:xs) = x `elem` xs

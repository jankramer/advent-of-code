module Advent.Y2017.Day17 where

day17a, day17b :: String -> String
day17a input = show $ finalBuffer !! (pos+1)
    where (pos, finalBuffer) = head $ drop 2017 $ partA (read input)
day17b input = show $ valAtIndex1
    where (_,_,valAtIndex1) = partB (read input) 50000000 (2,1,1)

partA :: Int -> [(Int,[Int])]
partA stepSize = scanl go (0,[0]) [1..]
  where go (pos, buffer) i = updateBuffer stepSize buffer pos i

updateBuffer :: Int -> [Int] -> Int -> Int -> (Int, [Int])
updateBuffer stepSize buffer pos i = (newPos, a ++ (newVal : b))
  where (a, b) = splitAt newPos buffer
        newPos = ((pos + stepSize) `mod` length buffer) + 1
        newVal = i

partB :: Int -> Int -> (Int,Int,Int) -> (Int,Int,Int)
partB step targetSize a@(size, _, _)
    | size >= targetSize = a
    | otherwise          = partB step targetSize (calcValAtIdx1 step a)

calcValAtIdx1 :: Int -> (Int,Int,Int) -> (Int,Int,Int)
calcValAtIdx1 step (size, pos, valAt1) = (nSize, nPos, nValAt1)
  where nSize    = size + 1
        nPos     = succ $ (pos + step) `mod` size
        nValAt1  = if nPos == 1 then size else valAt1

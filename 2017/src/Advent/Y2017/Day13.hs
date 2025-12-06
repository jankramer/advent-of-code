module Advent.Y2017.Day13 (day13a, day13b) where

type Firewall = [Layer]
type Layer    = (Int, Int)

day13a, day13b :: String -> String
day13a input = show $ totalSeverity $ map parse $ lines input
day13b input = show $ minDelay $ map parse $ lines input

totalSeverity :: Firewall -> Int
totalSeverity fw = sum [d * r | (d, r) <- fw, scannerAtTop d r]

minDelay :: Firewall -> Int
minDelay fw = head [delay | delay <- [0..], undetected fw delay]

scannerAtTop :: Int -> Int -> Bool
scannerAtTop _ 1 = True
scannerAtTop time range = time `mod` (2 * (range - 1)) == 0

undetected :: Firewall -> Int -> Bool
undetected fw delay = and [not $ scannerAtTop (delay + d) r | (d, r) <- fw]

parse :: String -> Layer
parse line = let (depth:range:_) = words line in (read $ init depth, read range)

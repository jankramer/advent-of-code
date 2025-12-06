module Advent.Y2017.Day16 where

import           Data.Foldable         (toList)
import           Data.Maybe            (fromJust)
import qualified Data.Sequence as Seq
import           Data.Sequence         (Seq)
import           Data.List.Split       (splitOn)

day16a, day16b :: String -> String
day16a input = toList $ danceA defaultPrograms (splitOn "," input)
day16b input = toList $ danceB defaultPrograms (splitOn "," input) 1000000000

type Programs  = Seq Char
type DanceMove = String

defaultPrograms :: Programs
defaultPrograms = Seq.fromList ['a'..'p']

danceA :: Programs -> [DanceMove] -> Programs
danceA programs danceMoves = foldl move programs danceMoves

danceB :: Programs -> [DanceMove] -> Int -> Programs
danceB programs danceMoves n = states !! numMoves
  where states    = scanl move programs (cycle danceMoves)
        cycleSize = succ . length $ takeWhile (/= programs) $ drop 1 states
        numMoves  = n `mod` cycleSize

-- | Apply dance move to program state
--
-- Examples:
--
-- >>> move (Seq.fromList "abcde") "s2"
-- fromList "deabc"
--
-- >>> move (Seq.fromList "abcde") "x1/2"
-- fromList "acbde"
--
-- >>> move (Seq.fromList "abcde") "pe/b"
-- fromList "aecdb"
move :: Seq Char -> String -> Seq Char

move input ('s':xs) = end Seq.>< front
  where (front,end) = Seq.splitAt (Seq.length input - (read xs)) input

move input ('x':xs) = Seq.update bPos aVal (Seq.update aPos bVal input)
  where (a:b:_) = splitOn "/" xs
        aPos    = read a
        bPos    = read b
        aVal    = Seq.index input aPos
        bVal    = Seq.index input bPos

move input ('p':a:'/':b:_) = Seq.update bPos a (Seq.update aPos b input)
  where aPos = fromJust $ Seq.elemIndexL a input
        bPos = fromJust $ Seq.elemIndexL b input

move _ _ = error "Unable to parse move"

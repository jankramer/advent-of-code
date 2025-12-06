module Advent.Y2017.Day19 where

import           Data.Array          (Array, array)
import qualified Data.Array as Array
import           Data.Char           (isAlpha)
import           Data.List           (reverse)
import           Data.Maybe

type Grid  = Array Point Char
type Point = (Int,Int)

day19a, day19b :: String -> String
day19a input = reverse chars
    where grid = buildGrid input
          (_, chars) = walkGrid grid (startingPoints grid, [])

day19b input = show $ length points
    where grid = buildGrid input
          (points, _) = walkGrid grid (startingPoints grid, [])

walkGrid :: Grid -> ([Point], [Char]) -> ([Point], [Char])
walkGrid g (points@(cur:prev:_), chars)
    | isJust next = walkGrid g ((fromJust next):points, newChars)
    | otherwise   = (points, newChars)
    where curVal = g Array.! cur
          newChars = if isAlpha curVal then curVal : chars else chars
          next = nextPoint g cur prev
walkGrid _ _ = error "Need more starting points to bootstrap"

nextPoint :: Grid -> Point -> Point -> Maybe Point
nextPoint grid point@(y,x) previous@(yPrev,xPrev)
    | val /= '+'      = if withinBounds (Array.bounds grid) next' && grid Array.! next' /= ' ' 
                            then Just next' 
                            else Nothing
    | length next > 0 = Just (head next)
    | otherwise       = Nothing
    where val   = grid Array.! point
          next  = [p | p <- neighbors grid point, p /= previous, grid Array.! p /= ' ']
          next' = (y + (y-yPrev), x + (x-xPrev))

withinBounds :: (Point,Point) -> Point -> Bool
withinBounds ((yMin,xMin),(yMax,xMax)) (y,x) = x >= xMin && x <= xMax && y >= yMin && y <= yMax

neighbors :: Grid -> Point -> [Point]
neighbors grid (y,x) = filter (withinBounds $ Array.bounds grid) possible
    where possible = [(y, x+1), (y, x-1), (y+1, x), (y-1, x)]

startingPoints :: Grid -> [Point]
startingPoints grid = [(y+1, x), firstNonEmpty] 
    where (firstNonEmpty@(y, x), _) = head $ filter (\(_,v) -> v /= ' ') $ Array.assocs grid

buildGrid :: String -> Grid
buildGrid input = array ((0,0),(ySize-1, xSize-1)) values
    where xSize = length $ head $ lines input
          ySize = length $ lines input
          values = [((y,x), (input !! (y * (xSize + 1) + x))) | x <- [0..(xSize-1)], y <- [0..(ySize-1)]]

example :: String
example = "\
\     |          \n\
\     |  +--+    \n\
\     A  |  C    \n\
\ F---|----E|--+ \n\
\     |  |  |  D \n\
\     +B-+  +--+ "

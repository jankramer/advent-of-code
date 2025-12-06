module Advent.Y2017.Day20 where

import Data.Function (on)
import Data.List (sortBy)
import Data.List.Split  (splitOn)
import Math.Geometry.Vector
import Math.Geometry.Distance

day20a, day20b :: String -> String
day20a input = show $ minimum $ zip distances [0..(length distances-1)]
    where positions = map position $ run tickA 1000 $ loadParticles input
          distances = map (manhattan $ origin) $ positions

day20b input = show $ length $ run tickB 1000 $ loadParticles input

data Particle = Particle { position :: Vector
                         , velocity :: Vector
                         , acceleration :: Vector
                         } deriving (Show, Eq)


run :: ([Particle] -> [Particle]) -> Int -> [Particle] -> [Particle]
run fn n particles = head $ drop n $ iterate fn particles

tickA :: [Particle] -> [Particle]
tickA particles = map tick particles

tickB :: [Particle] -> [Particle]
tickB particles = removeCollisions $ sortBy (compare `on` position) $ map tick particles

tick :: Particle -> Particle
tick (Particle p v a) = Particle (plus p newV) newV a
    where newV = (plus v a)

removeCollisions :: [Particle] -> [Particle]
removeCollisions [] = []
removeCollisions (x:rest) = if numSameElements > 0
    then removeCollisions $ drop numSameElements rest
    else x : (removeCollisions rest)
    where numSameElements = length $ takeWhile (\y -> position x == position y) rest

loadParticles :: String -> [Particle]
loadParticles input = map parseParticle $ lines input

parseParticle :: String -> Particle
parseParticle str = Particle p v a
    where (p:v:a:_) = map parseVector $ words str

parseVector :: String -> Vector
parseVector (_:'=':'<':xs) = map read $ splitOn "," ys
    where (ys:_) = splitOn ">" xs

parseVector _ = error "Unable to read vector"

module Math.Geometry.Vector where

type Vector = [Int]

nullVector :: Int -> Vector
nullVector n = take n $ repeat 0

origin :: Vector
origin = nullVector 3

plus, minus, times :: Vector -> Vector -> Vector
plus  a b = zipWith (+) a b
minus a b = zipWith (-) a b
times a b = zipWith (*) a b

absolute :: Vector -> Vector
absolute = map abs
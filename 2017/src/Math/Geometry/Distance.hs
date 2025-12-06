module Math.Geometry.Distance where

import Math.Geometry.Vector

manhattan :: Vector -> Vector -> Int
manhattan a b = sum $ absolute $ minus a b

chebyshev :: Vector -> Vector -> Int
chebyshev a b = maximum $ absolute $ minus a b
module Advent.Y2017.Day09 (day09a, day09b) where

day09a, day09b :: String -> String
day09a input = show $ let (ans, _) = parse input [] (0,0) in ans
day09b input = show $ let (_, ans) = parse input [] (0,0) in ans

parse :: String -> [String] -> (Int, Int) -> (Int, Int)
parse [] _ score = score

-- Ignore any characters that are preceded by !
parse ('!':_:xs) zs score = parse xs zs score

-- Close garbage group
parse ('>':xs) (('<':_):zs) score = parse xs zs score

-- Ignore any other characters within garbage group
parse (_:xs) a@(('<':_):_) (score, count) = parse xs a (score, count + 1)

-- Open garbage group
parse ('<':xs) ys score = parse xs ("<":ys) score

-- Open new group
parse ('{':xs) ys score = parse xs ("{":ys) score

-- Close group when current group starts with '{'
parse ('}':xs) (('{':_):zs) (score, count) =
    parse xs zs ((score + 1 + (length zs)), count)

-- Ignore the rest
parse (_:xs) zs score = parse xs zs score

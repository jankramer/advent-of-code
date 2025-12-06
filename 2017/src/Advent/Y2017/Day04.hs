module Advent.Y2017.Day04 (day04a, day04b) where

import Data.List

day04a, day04b :: String -> String
day04a input = show $ length $ filter (not . containsDuplicates) . map words $ lines input
day04b input = show $ length $ filter (not . containsAnagrams) . map words $ lines input

containsDuplicates :: [String] -> Bool
containsDuplicates passwords = (length $ nub passwords) < length passwords

containsAnagrams :: [String] -> Bool
containsAnagrams [] = False
containsAnagrams (x:xs) = length (filter (isAnagram x) xs) > 0 || containsAnagrams xs

isAnagram :: [Char] -> [Char] -> Bool
isAnagram x y = (sort x) == (sort y)

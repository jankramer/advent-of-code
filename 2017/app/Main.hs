module Main where

import Advent.Y2017       (solve17)
import System.Environment (getArgs)
import Text.Printf        (printf)

main :: IO ()
main = getArgs >>= parse

parse :: [String] -> IO ()
parse (y:d:p:_) = solve (read y) (read d) (head p)
parse _         = usage

solve :: Int -> Int -> Char -> IO ()
solve year day part = putStrLn <$> solver year day part =<< getInput year day

getInput :: Int -> Int -> IO String
getInput year day = readFile $ printf "resources/inputs/%d/day%02d.txt" year day

solver :: Int -> Int -> Char -> String -> String
solver 2017 = solve17
solver _    = error "Not implemented yet"

usage :: IO ()
usage = do
    putStrLn "Usage: aoc <year> <day> <a|b>"
    putStrLn "Solves the Advent of Code puzzle of the given year and day."

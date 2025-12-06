module Advent.Y2017.Day07 (day07a, day07b) where

import Data.Char
import Data.List
import Data.List.Split
import Data.Maybe
import Data.Map (Map)
import qualified Data.Map as Map
import Data.Ord
import Data.Tree (Tree (Node, rootLabel, subForest))
import qualified Data.Tree as Tree

type Tower = Tree Program
data Program = Program { name :: String
                       , weight :: Int
                       , children :: [String]
                       } deriving (Show, Eq)

day07a, day07b :: String -> String
day07a input = solve $ towerFromInput input
day07b input = show $ solve' $ towerFromInput input

towerFromInput :: String -> Tower
towerFromInput input = buildTower $ Map.fromList $ map (\x -> (name x, x)) programList
    where programList = map parseLine $ lines input

solve :: Tower -> String
solve tower = name $ rootLabel tower

solve' :: Tower -> Int
solve' tower = calculateProperWeight tower (findUnbalancedTower tower)

calculateProperWeight :: Tower -> Tower -> Int
calculateProperWeight tower unbalancedTower@(Node unbalancedRoot _) = (weight unbalancedRoot) - weightDifference
    where siblings = subForest $ parent tower unbalancedRoot
          firstBalancedSibling = head $ siblings \\ [unbalancedTower]
          weightDifference = treeWeight unbalancedTower - treeWeight firstBalancedSibling

findUnbalancedTower :: Tower -> Tower
findUnbalancedTower tree
    | isNothing unbalancedSubtree   = tree
    | otherwise                     = findUnbalancedTower $ fromJust unbalancedSubtree
    where unbalancedSubtree = findUnbalancedSubtree tree

findUnbalancedSubtree :: Tower -> Maybe (Tower)
findUnbalancedSubtree (Node _ subtrees)
    | length unbalancedSubTrees == 0 = Nothing
    | length unbalancedSubTrees == 1 = let (_, tree) = (head unbalancedSubTrees) in Just tree
    | otherwise                      = error "Multiple unbalanced subtrees"
    where weightedTrees     = map (\x -> (treeWeight x, x)) subtrees
          weights           = map fst weightedTrees
          unbalancedSubTrees = filter (\(w, _) -> length (elemIndices w weights) == 1) weightedTrees

treeWeight :: Tower -> Int
treeWeight (Node program subtrees) = (weight program) + (sum $ map treeWeight subtrees)

buildTower :: Map String Program -> Tower
buildTower towerMap = maximumBy (comparing towerDepth) allSubtowers
    where allSubtowers = (map (buildSubtower towerMap)) $ (Map.elems towerMap)
          towerDepth tower = length $ Tree.levels tower

buildSubtower :: Map String Program -> Program -> Tower
buildSubtower programMap program  = Node program subtowers
    where findProgram name' = fromJust $ Map.lookup name' programMap
          subtowers        = map (\name' -> buildSubtower programMap (findProgram name') ) (children program)

parseLine :: String -> Program
parseLine str = Program name' weight'' children''
    where ((name':weight':_):children') = map words $ splitOn "->" str
          weight'' = read $ filter isDigit weight' :: Int
          children'' = map (filter isAlpha) $ concat children'

-- Tree helpers

findSubtreeByLabel :: Eq a => Tree a -> a -> Tree a
findSubtreeByLabel tree label = fromJust $ find matchingLabel (subForest tree)
    where matchingLabel subtree = rootLabel subtree == label

parent :: Tower -> Program -> Tower
parent fullTree searchNode
    | length path == 2 = fullTree
    | length path >= 2 = parent (findSubtreeByLabel fullTree nextNode) searchNode
    | otherwise        = error "Expected path length to be at least 2"
    where path = head $ pathsToNode searchNode fullTree
          nextNode = head $ drop 1 path

-- Source: https://stackoverflow.com/questions/21400111/how-to-find-path-to-a-node-in-a-haskell-data-tree
pathsToNode :: Eq a => a -> Tree a -> [[a]]
pathsToNode x (Node y ns) = [[x] | x == y] ++ map (y:) (pathsToNode x =<< ns)

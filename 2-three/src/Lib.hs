module Lib
    ( someFunc
    ) where

import System.IO  
import Control.Monad

maxY :: [String] -> Int
maxY lines = length lines


wrapX :: Int -> String -> Int
wrapX current line
    | current < length line = current
    | current >= length line = current - (length line)


findTree :: Int -> String -> Int
findTree x line
    | line!!x == '#' = 1
    | otherwise = 0


stepLine :: Int -> Int -> [String] -> Int -> Int -> Int
stepLine x y lines xTraversal yTraversal
    | y >= maxY lines = 0
    | otherwise = (findTree x (lines !! y)) + (stepLine (wrapX (x + xTraversal) (lines !! y)) (y + yTraversal) lines xTraversal yTraversal)


firstSolution :: [String] -> Int
firstSolution input = stepLine 0 0 input 3 1


secondSolution :: [String] -> Int -> Int -> Int
secondSolution input xTraversal yTraversal = stepLine 0 0 input xTraversal yTraversal

someFunc :: IO ()
someFunc = do
    content <- readFile "input.txt"
    let linesOfFiles = lines content
    let first = firstSolution linesOfFiles
    let second1 = secondSolution linesOfFiles 1 1
    let second2 = secondSolution linesOfFiles 5 1
    let second3 = secondSolution linesOfFiles 7 1
    let second4 = secondSolution linesOfFiles 1 2
    putStrLn (show first)
    putStrLn (show (first * second1 * second2 * second3 * second4))
    -- putStrLn second
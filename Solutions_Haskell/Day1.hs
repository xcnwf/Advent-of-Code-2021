import System.IO  
import Control.Monad
import Data.Typeable

main = do content <- read_file
          print (part1 content)
          print (part2 content)

read_file :: IO [Int]
read_file = do contents <- readFile "../Inputs/d1.txt"
               let singlewords = lines contents
               print (typeOf singlewords)
               let list = f singlewords
               print (typeOf list)
               return list

f :: [String] -> [Int]
f = map read

part1 :: [Int] -> Int
part1 content = foldl (\val (x,x1) -> val + if x1>x then 1 else 0) 0 (zip (content) (tail content))

part2 :: [Int] -> Int
part2 content = foldl (\val (x,x1) -> val + if x1>x then 1 else 0) 0 (zip (content) (drop 3 content))
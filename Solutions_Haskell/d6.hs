import Reader ( read_file )
import qualified Data.HashMap.Strict as HM
import qualified Data.Bifunctor

{- HLINT ignore "Use camelCase" -}

main = do
    content <- read_file "d6" (map (read :: String -> Int) . split)
    let fishes = foldl parse [0,0,0,0,0,0,0,0,0] $ head content 
    print fishes
    putStrLn ("Part1 : "++show (part1 fishes))
    putStrLn ("Part2 : "++show (part2 fishes))

split str = case break (==',') str of
                (a, _comma:b) -> a : split b
                (a, _empty)   -> [a]

parse acc n = let (fishes,_) = unzip $ map (\(x,idx) -> (if idx == n then x + 1 else x,idx)) $ zip acc $ iterate succ 0
    in fishes

run_simulation_week weeks fishes =
    let f fishes = zipWith (+) fishes (drop 7 fishes) ++ zipWith (+) (take 5 fishes) (drop 2 fishes) ++ (take 2 . drop 5 $ fishes)
    in iterate f fishes !! weeks

get_count remaining_days fishes = sum fishes + sum (take remaining_days fishes)

part1 :: [Int] -> Int
part1 fishes =
    let (weeks, days) = divMod 80 7
    in get_count days . run_simulation_week weeks $ fishes

part2 :: [Int] -> Int
part2 fishes =
    let (weeks, days) = divMod 256 7
    in get_count days . run_simulation_week weeks $ fishes

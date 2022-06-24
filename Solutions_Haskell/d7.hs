import Reader ( read_file )
import Data.List (foldl', sort)

{- HLINT ignore "Use camelCase" -}

main = do
    content <- read_file "d7" (map (read :: String -> Int) . split)
    let crabs = head content 
    --print crabs
    putStrLn ("Part1 : "++show (part1 crabs))
    putStrLn ("Part2 : "++show (part2 crabs))

split str = case break (==',') str of
                (a, _comma:b) -> a : split b
                (a, _empty)   -> [a]


calc_mean_inf :: [Int] -> Int
calc_mean_inf l = let (total, count) = foldl' (\(total, count) pos -> (total+pos, count+1)) (0,0) l
                      (mean, rest) = divMod total count
    in mean

calc_median :: [Int] -> Int
calc_median l = let (mid_pos, odd) = divMod (length l) 2
                    sorted = sort l 
    in if odd == 0 then sorted !! mid_pos else div (sorted !! mid_pos + sorted !! mid_pos + 1) 2

part1 :: [Int] -> Int
part1 poses =
    let best_pos = calc_median poses
    in sum . map (abs . (best_pos-)) $ poses

calc_integer_sum n = div (n * (n+1)) 2 

part2 :: [Int] -> Int
part2 poses = 
    let best_pos = calc_mean_inf poses
        score1 = sum . map (calc_integer_sum . abs . (best_pos-)) $ poses
        score2 = sum . map (calc_integer_sum . abs . (best_pos+1-)) $ poses
    in min score1 score2
 

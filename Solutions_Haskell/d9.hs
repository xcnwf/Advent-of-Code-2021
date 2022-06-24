import Reader ( read_file )
import Data.List (foldl', sort, zip5, zipWith5)
import Data.Char (digitToInt)

{- HLINT ignore "Use camelCase" -}

main = do
    mat <- read_file "d9_example" (map digitToInt)
    print $ find_lowest_points mat 
    putStrLn ("Part1 : "++show (part1 mat))
    {-putStrLn ("Part2 : "++show (part2 crabs)) -}


is_lowest_point (p,left_p,right_p,up_p,down_p) = p < left_p && p<right_p && p < down_p && p < up_p

tailWith :: t -> [t] -> [t]
tailWith = 
    let doTailWith b el l = case l of
            [] -> if b then error "WTF" else [el]
            x:xs -> let queue = doTailWith False el xs in if b then queue else x:queue
    in doTailWith True

fst5 (x, _, _, _, _) = x

find_lowest_points :: [[Int]] -> [[(Int, Bool)]]
find_lowest_points mat =
    let height = length mat
        width = length $ head mat
        down_mat = tailWith (replicate width 9) mat
        up_mat = replicate width 9 : init mat
        right_mat = map (\l -> 9:init l) mat
        left_mat = map (tailWith 9) mat
        all_mat = zipWith5 zip5 mat left_mat right_mat up_mat down_mat
    in (map.map) (\x -> (fst5 x, is_lowest_point x)) all_mat


part1 :: [[Int]] -> Int
part1 poses = sum . map ((+1).fst) . filter snd . concat $ find_lowest_points poses


{-part2 :: [Int] -> Int
part2 poses = 
    let best_pos = calc_mean_inf poses
        score1 = sum . map (calc_integer_sum . abs . (best_pos-)) $ poses
        score2 = sum . map (calc_integer_sum . abs . (best_pos+1-)) $ poses
    in min score1 score2 -}


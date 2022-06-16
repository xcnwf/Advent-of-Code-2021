import Reader
import Data.Char(digitToInt)


main = do content <- read_file "d3" (map digitToInt)
          --print (take 10 content)
          print (part1 content)
          {- print (part2 content) -}

present_bits :: (Int->Int->Bool) -> [[Int]] -> [Int]
present_bits f content = let length_arr = length content in
    map (\x ->
        if f x (div length_arr 2) then 1 else 0
    ) (foldl (zipWith (+)) (repeat 0) content)

most_present_bits = present_bits (>=)
least_present_bits = present_bits (<)

bits_to_int :: [Int] -> Int
bits_to_int = foldl (\val x -> 2*val + x) 0

part1 :: [[Int]] -> Int
part1 content = 
    let arr = most_present_bits content in
        (bits_to_int arr) * (bits_to_int (map (1-) arr))

{- part2 :: [[Int]] -> Int
part2 content = 
    let f l acc comp = case l of 
                [] -> print "error" >> []
                x:[] -> (reverse acc)++x
                _ -> let c = head (comp l)
                     in f (map (\el -> tail el) ((filter (\el -> head el == c)) l)) (c:acc) comp
    in 
    let (arr1, arr2) = (f content [] most_present_bits, f content [] least_present_bits) in
        (foldl (\val x -> 2*val + x) 0 arr1) * (foldl (\val x -> 2*val + x) 0 arr2) -}
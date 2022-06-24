{-# OPTIONS_GHC -Wno-unrecognised-pragmas #-}
{-# HLINT ignore "Use camelCase" #-}
import Reader
import Data.Either
import Data.Maybe

main = do
    content <- read_file "d4" id
{-  let (nbs, boards) = parse_bingo_input content
    print (head (bingo_mark 83 boards))
    print (map bingo_sum ((bingo_mark 83 boards))) -}

    putStrLn ("Part1 : "++show (part1 content))
    putStrLn ("Part2 : "++show (part1 content))

type Board = [[(Int, Bool)]]

split str = case break (==',') str of
                (a, _comma:b) -> a : split b
                (a, _empty)   -> [a]

parse_bingo_board_lines :: [String] -> Board -> Int -> Board
parse_bingo_board_lines lines board i =
    if i > 4 then reverse board
    else let nb = map ((\x -> (x, False)) . (read :: String -> Int)) (words (head lines))
         in parse_bingo_board_lines (tail lines)  (nb:board) (i+1)

parse_bingo_boards :: [String] -> [Board] -> [Board]
parse_bingo_boards input boards =
    case input of
        [] -> boards
        [_] -> boards --Last line of file is empty
        _ -> parse_bingo_boards (drop 6 input) (
            -- add next bingo to the list
            parse_bingo_board_lines (tail input{- First line is empty -}) [] 0:boards)

parse_bingo_input :: [String] -> ([Int], [Board])
parse_bingo_input input = let bingo_numbers = map (read :: String -> Int) (split (head input)) in
    (bingo_numbers, reverse (parse_bingo_boards (tail input) []))

-- No diagonals. Left => Not Winner / Winner <== Right
bingo_check :: Board -> Either Board Board
bingo_check board =
    let line_check = any (all snd) board
    in if line_check || or (foldl (\acc line -> zipWith (&&) (map snd line) acc) (repeat True) board) then Right board else Left board

bingo_mark :: Int -> [[[(Int,Bool)]]] -> [[[(Int,Bool)]]]
bingo_mark num = (map.map.map) (\(x, b) -> if x == num then (x,True) else (x,b))

bingo_sum :: Board -> Int
bingo_sum = foldl (\acc l2 -> acc + foldl (\acc (x,b) -> acc + (if b then 0 else x)) 0 l2) 0 --Sum of unchecked

part1 :: [String] -> Int
part1 content =
    let (nbs, boards) = parse_bingo_input content
        f num boards = let next_boards = bingo_mark num boards in
                (next_boards, listToMaybe (map ((*num).bingo_sum) (rights (map bingo_check next_boards))))
    in
    let (_, m) = foldl (\ (brd, m) num -> case m of
            Just _ -> ([],m)
            Nothing -> f num brd
            ) (boards, Nothing) nbs
    in fromMaybe 0 m

part2 :: [String] -> Int
part2 content =
    let (nbs, boards) = parse_bingo_input content
        f num boards = let next_boards = bingo_mark num boards
                           filter_boards = lefts (map bingo_check next_boards)
            in (filter_boards, if null filter_boards then Just (num * bingo_sum (head next_boards)) else Nothing)
    in
    let (brd, m) = foldl (\ (brd, m) num -> case m of
            Just _ -> ([],m)
            Nothing -> f num brd
            ) (boards, Nothing) nbs
    in fromMaybe 0 m
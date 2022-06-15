import Reader

main = do content <- read_file "d1" (read :: String -> Int)
          print (part1 content)
          print (part2 content)

part1 :: [Int] -> Int
part1 content = foldl (\val (x,x1) -> val + if x1>x then 1 else 0) 0 (zip (content) (tail content))

part2 :: [Int] -> Int
part2 content = foldl (\val (x,x1) -> val + if x1>x then 1 else 0) 0 (zip (content) (drop 3 content))
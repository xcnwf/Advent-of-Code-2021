import Reader

main = do content <- read_file "d2" read_func
          print (take 10 content)
          print (part1 content)
          print (part2 content)

read_func :: (String -> (String, Int))
read_func s = case (words s) of
                s1:i1:[] -> (s1, (read :: String -> Int) i1)
                _ -> ("forward", 0)

part1 :: [(String, Int)] -> Int
part1 content = let (x,depth,_) = foldl (\(x,depth,aim) (instr,val) -> 
                                                                    case instr of
                                                                        "forward" -> (x+val, depth, aim)
                                                                        "up" -> (x, depth-val, aim)
                                                                        "down" -> (x, depth+val, aim)
                                                                    ) (0,0,0) content
    in x * depth

part2 :: [(String, Int)] -> Int
part2 content = let (x,depth,_) = foldl (\(x,depth,aim) (instr,val) -> 
                        case instr of
                            "forward" -> (x+val, depth+aim*val, aim)
                            "up" -> (x, depth, aim-val)
                            "down" -> (x, depth, aim+val)
                        ) (0,0,0) content
                        in x * depth
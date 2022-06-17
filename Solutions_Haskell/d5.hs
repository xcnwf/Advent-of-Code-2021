import Reader ( read_file )
import qualified Data.HashMap.Strict as HM
import qualified Data.Bifunctor

{- HLINT ignore "Use camelCase" -}

main = do
    content <- read_file "d5" parse_vents
    putStrLn ("Part1 : "++show (part1 content))
    putStrLn ("Part2 : "++show (part2 content))

type Point = (Int,Int)
type Line = (Point,Point)

split str = case break (==',') str of
                (a, _comma:b) -> a : split b
                (a, _empty)   -> [a]

parse_point :: String -> Point
parse_point str =
    case split str of
        [p1, p2] -> ((read :: String->Int) p1, (read :: String->Int) p2)
        _ -> (0,0)

parse_vents :: String -> Line
parse_vents input =
    case words input of
        [s1, _, s2] -> (parse_point s1, parse_point s2)
        _ -> ((0,0),(0,0))

add_points :: HM.HashMap Point Int -> Line -> HM.HashMap Point Int
add_points points ((x1,y1),(x2,y2))
    -- Vertical
    | x1 == x2 =
        let y_min = min y1 y2
            y_max = max y1 y2
        in foldl (\acc y -> HM.insertWith (+) (x1,y) 1 acc) points [y_min..y_max]
        --Horizontal
    | y1 == y2 =
            let x_min = min x1 x2
                x_max = max x1 x2
            in foldl (\acc x -> HM.insertWith (+) (x,y1) 1 acc) points [x_min..x_max]
    | otherwise = points


part1 :: [Line] -> Int
part1 lines =
    let points = foldl add_points HM.empty lines
    in (HM.size . HM.filter (>=2)) points

part2 :: [Line] -> Int
part2 lines =
    let points = foldl add_points HM.empty lines
        points2 = foldl add_diagonals points lines
    in (HM.size . HM.filter (>=2)) points2

add_diagonals :: HM.HashMap Point Int -> Line -> HM.HashMap Point Int
add_diagonals points ((x1,y1),(x2,y2))
    | abs(x2-x1) == abs(y2-y1) =
        let (delta_x, delta_y) = (x2-x1,y2-y1)
            dir = Data.Bifunctor.bimap (signum delta_x +) (signum delta_y +)
        in foldl (\acc (x,y) -> HM.insertWith (+) (x,y) 1 acc) points (take (abs delta_x + 1) (iterate dir (x1,y1)))
    | otherwise = points

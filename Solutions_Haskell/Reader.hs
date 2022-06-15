module Reader where
import System.IO  
import Control.Monad

read_file :: String -> (String -> a) -> IO [a]
read_file filename f = do contents <- readFile ("../Inputs/"++filename++".txt")
                          let lines_arr = lines contents
                          let list = map f lines_arr
                          return list
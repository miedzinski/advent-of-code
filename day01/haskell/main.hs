import Data.Char

part1 :: [Int] -> Int
part1 xs = sum $ zipWith (\a b -> if a == b then a else 0) xs (tail xs ++ [head xs])

part2 :: [Int] -> Int
part2 xs =
    sum $ zipWith (\a b -> if a == b then a + b else 0) (fst halves) (snd halves)
        where halves = splitAt ((length xs) `div` 2) xs

main :: IO ()
main = do
    let nums = map digitToInt (read)
    putStrLn $ "part1: " ++ show (part1 nums)
    putStrLn $ "part2: " ++ show (part2 nums)

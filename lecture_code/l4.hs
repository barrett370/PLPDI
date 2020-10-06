
data Arith = T
           | F
           | Ite Arith Arith Arith
           | Zero
           | Succ Arith
           | Pred Arith
           | Iszero Arith
               deriving (Show)

size :: Arith -> Integer
size t
  = case t of
        T -> 1
        F -> 1
        (Ite a b c) -> 1 + size a + size b + size c
        Zero -> 1
        Succ a -> 1 + size a
        Pred a -> 1 + size a
        Iszero a -> 1 + size a

max2 :: Integer -> Integer -> Integer
max2 a b
  | a < b = b
  | otherwise = a

max3 :: Integer -> Integer -> Integer -> Integer
max3 a b c = max2 (max2 a b) c

depth :: Arith -> Integer

depth t
  = case t of
        T -> 1
        F -> 1
        Zero -> 1
        Ite a b c -> 1 + max3 (depth a) (depth b) (depth c)
        Succ a -> 1 + depth a
        Pred a -> 1 + depth a
        Iszero a -> 1 + depth a

width :: Arith -> Integer
width t
  = case t of
        T -> 1
        F -> 1
        Ite a b c -> width a + width b + width c
        Zero -> 1
        Succ a -> width a
        Pred a -> width a
        Iszero a -> width a

eval :: Arith -> Arith
eval t
  = case t of
        T -> T
        F -> F
        Zero -> Zero
        Ite a b c -> case eval a of
                         T -> eval b
                         F -> eval c
                         x -> Ite x b c
        --Or exception 
        Succ a -> Succ a
        Pred a -> case eval a of
                      Zero -> Zero
                      Succ x -> x
                      x -> Pred x
        --Or exception 
        Iszero a -> case eval a of
                        Zero -> T
                        Succ x -> F
                        x -> Iszero x
                        --Or exception 

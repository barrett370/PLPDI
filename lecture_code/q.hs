
module Queue(Queue, new, enq, deq, peek) where
data Queue a =  Q [a] deriving Show

new :: Queue a
new = Q []

enq :: a -> Queue a -> Queue a
enq x (Q q) = Q( q ++ [ x ])


deq :: Queue a -> Queue a
deq (Q []) = Q ( [] )
deq (Q (_:t)) = Q t 

peek :: Queue a -> Maybe a
peek (Q []) = Nothing 
peek (Q (x:_)) = Just x

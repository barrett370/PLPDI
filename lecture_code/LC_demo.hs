{-# LANGUAGE UnicodeSyntax #-}

{-- You will find QUESTIONs at the end of this file --}

module Main where

import Data.Either
import Text.ParserCombinators.Parsec
import Test.QuickCheck


-- datatype
data LC =
  Var String
  | Lam String LC
  | App LC LC
  deriving (Show)


--parser
def :: LC
def = Var "error"

mkAppAcc :: LC -> [LC] -> LC
mkAppAcc t [] = t
mkAppAcc t (x:l) = mkAppAcc (App t x) l

mkApp :: [LC] -> LC
mkApp [] = def
mkApp (x:l) = mkAppAcc x l

atom :: Parser LC
atom =
    (do char '('
        spaces
        e <- term
        spaces
        char ')'
        return e)
    <|>
    (do char 'λ'
        spaces
        v <- many1 letter
        spaces
        char '.'
        t <- term
        return (Lam v t))
    <|>
    (do char '\\'
        spaces
        v <- many1 letter
        spaces
        char '.'
        t <- term
        return (Lam v t))
    <|>
    (do spaces
        v <- many1 letter
        spaces
        return (Var v))

term :: Parser LC
term =
    (do l <- many1 atom
        return (mkApp l))
    <|>
    (do spaces
        a <- atom
        spaces
        return a)

str2lc :: String -> LC
str2lc s = fromRight def (parse term "" s)


-- testing
sarbitrary :: Gen String
sarbitrary = arbitrary

instance Arbitrary LC where
  arbitrary =
    oneof [do v <- sarbitrary
              return (Var v),
           do v <- sarbitrary
              t <- arbitrary
              return (Lam v t),
           do t1 <- arbitrary
              t2 <- arbitrary
              return (App t1 t2)]

stuck :: LC -> Bool
stuck (Var _) = True
stuck (Lam _ _) = False
stuck (App a _) = stuck a

norm :: LC -> Bool
norm (Var _) = True
norm (Lam _ _) = True
norm (App a _) = stuck a
{-- You can use this norm function to test your call-by-value (cbv) or call-by-name (cbn)
    interpreters as follows:
     - start ghci
     - load this file by typing: ":l LC" in ghci
     - execute for example: quickCheck (\x -> norm (cbv x))
--}


-- Capture avoiding functions
fvars :: LC -> [String]
fvars (Var w) = [w]
fvars (Lam v t) = filter (\x -> v /= x) (fvars t)
fvars (App t u) = (fvars t)++(fvars u)

vars :: LC -> [String]
vars (Var w) = [w]
vars (Lam v t) = v:(vars t)
vars (App t u) = (vars t)++(vars u)

swap1 :: String -> String -> String -> String
swap1 w a b = if w == a then b else if w == b then a else w

swap :: LC -> String -> String -> LC
swap (Var v) a b = Var (swap1 v a b)
swap (Lam v t) a b = Lam (swap1 v a b) (swap t a b)
swap (App t u) a b = App (swap t a b) (swap u a b)

fresh :: String -> [String] -> String
fresh v vs = if elem v vs then fresh (v ++ "'") vs else v


-- Substitution function
subst :: LC -> String -> LC -> LC
subst (Var w) v b =
  if v == w then b else Var w
subst (Lam w t) v b =
  if v == w then Lam w t
  else if elem w (fvars b) then
         let vs = v:w:(vars t)++(vars b) in
         let w' = fresh w vs in
         Lam w' (subst (swap t w w') v b)
       else Lam w (subst t v b)
subst (App t u) v b = App (subst t v b) (subst u v b)


{-- QUESTION 1: write a call-by-value interpreter for the lambda-calculus --}

cbv :: LC -> LC 
cbv (Var x) = Var x 
cbv (Lam x t) = Lam x t
cbv (App t u) 
  = case cbv t of
    Lam x b -> let a = cbv u in cbv (subst b x a)
    z -> App z u 

{-- QUESTION 2: write a call-by-name interpreter for the lambda-calculus --}


-- A few terms
term1 :: LC
term1 = str2lc "x"

term2 :: LC
term2 = str2lc "(λx.λy.x)(λx.x)"

term3 :: LC
term3 = str2lc "(λx.λy.x)y"

term4 :: LC
term4 = str2lc "λ x . x x"

term5 :: LC
term5 = str2lc "(λx.λy.y)((λx. x x)(λx. x x))"

term6 :: LC 
term6 = str2lc "\\ x . xx"
main :: IO ()
main = print "done"

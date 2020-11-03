
module Parser(expr, apply) where 
    import Data.Char
    data Parser a = Parser (String -> [(a,String)])

    item :: Parser Char
    item = Parser (\cs -> case cs of
                            "" -> []
                            (c:cs) -> [(c,cs)])


    parse (Parser p) = p 

    class Monad m where
        return :: a -> m a
        (>>=) :: m a -> (a -> m b) -> m b

    instance Parser.Monad Parser where
        return a = Parser (\cs -> [(a,cs)])
        p >>= f = Parser (\cs -> concat [parse (f a) cs' |
                                    (a,cs') <- parse p cs])


    class Parser.Monad m => MonadZero m where
        zero :: m a

    class MonadZero m => MonadPlus m where 
        (++) :: m a -> m a -> m a

    instance MonadZero Parser where
        zero = Parser (\cs -> [])

    instance MonadPlus Parser where
        p ++ q = Parser (\cs -> parse p cs Prelude.++ parse q cs)


    (+++) :: Parser a -> Parser a -> Parser a
    p +++ q = Parser (\cs -> case parse (p Parser.++ q) cs of
                            [] -> []
                            (x:xs) -> [x])

    sat :: (Char -> Bool) -> Parser Char


    sat p = item Parser.>>= \c ->
            if p c then Parser.return c else zero

    char :: Char -> Parser Char
    char c = sat (c ==)

    string :: String -> Parser String
    string "" = Parser.return "" 
    string (c:cs) = char c Parser.>>= \_ -> string cs Parser.>>= \_ -> Parser.return (c:cs)

    many :: Parser a -> Parser [a]
    many p = many1 p +++ Parser.return []

    many1 :: Parser a -> Parser [a]
    many1 p = p Parser.>>= \a ->
                many p Parser.>>= \as ->
                Parser.return (a:as)

    sepby :: Parser a -> Parser b -> Parser [a]
    p `sepby` sep = (p `sepby` sep) +++ Parser.return []

    sepby1 :: Parser a -> Parser b -> Parser [a]
    p `sepby1` sep = p Parser.>>= \a ->
                        many (sep Parser.>>= \_ -> p) Parser.>>= \as ->
                        Parser.return (a:as)

    chain1 :: Parser a -> Parser (a->a->a) -> a -> Parser a
    chain1 p op a = (p `chainl1` op) +++ Parser.return a

    chainl1 :: Parser a -> Parser (a -> a -> a) -> Parser a
    p `chainl1` op = p Parser.>>= \a -> rest a where 
                        rest a = (op Parser.>>= \f ->
                                    p Parser.>>= \b ->
                                    rest (f a b))
                                    +++ Parser.return a 

    space :: Parser String
    space = many (sat isSpace)

    token :: Parser a -> Parser a
    token p = p Parser.>>= \a -> space Parser.>>= \_ -> Parser.return a

    symb :: String -> Parser String
    symb cs = token (string cs)

    apply :: Parser a -> String -> [(a,String)]
    apply p = parse ( space Parser.>>= \_ -> p)

    expr :: Parser Int
    expr = term `chainl1` addop

    term = factor `chainl1` mulop

    factor = digit +++ (symb "(" Parser.>>= \_ -> expr Parser.>>= \n -> symb ")" Parser.>>= \_ -> Parser.return n)

    digit = (token (sat isDigit) Parser.>>= \x -> Parser.return (ord x - ord '0'))

    addop :: Parser (Int -> Int -> Int)
    addop = (symb "+" Parser.>>= \_ -> Parser.return (+)) +++ (symb "-" Parser.>>= \_ -> Parser.return (-))

    mulop:: Parser (Int -> Int -> Int)
    mulop = (symb "*" Parser.>>= \_ -> Parser.return (*)) +++ (symb "/" Parser.>>= \_ -> Parser.return (div))


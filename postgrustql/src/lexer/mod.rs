// location of the token in source code
#[derive(Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct TokenLocation {
    pub line: usize,
    pub col: usize,
}

impl TokenLocation {
    pub fn new() -> Self {
        TokenLocation { col: 0, line: 0 }
    }
    pub fn new_with_col_and_line(col: usize, line: usize) -> Self {
        TokenLocation { col, line }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TokenKind {
    String,
    Identifier,
    Keyword,
    Symbol,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TokenContainer {
    pub token: Token,
    pub loc: TokenLocation,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Cursor {
    pub pointer: usize,
    pub loc: TokenLocation,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Token {
    // Keywords
    As,
    From,
    Into,
    Values,
    Insert,
    Select,
    Create,
    Where,
    Table,
    Drop,
    And,
    Or,
    Not,
    True,
    Join,
    False,
    Inner,
    Left,
    Right,
    Is,
    SmallInt,
    Int,
    BigInt,
    Real,
    Double,
    Precision,
    DoublePrecision,
    Varchar,
    Char,
    Text,
    Bool,
    Unique,
    Index,
    On,
    Primary,
    Key,
    Null,
    Alter,
    Delete,
    Update,
    Constraint,
    Foreign,
    Distinct,
    Order,
    By,
    OrderBy,
    Asc,
    Desc,
    Offset,
    Limit,
    Outer,
    Full,

    // Symbols
    Semicolon,
    Asterisk,
    Comma,
    LeftParenthesis,
    RightParenthesis,
    Equal,
    NotEqual,
    Concat,
    Plus,
    Minus,
    Slash,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Modulo,
    Exponentiation,
    SquareRoot,
    CubeRoot,
    Factorial,
    FactorialPrefix,
    AbsoluteValue,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    BitwiseShiftLeft,
    BitwiseShiftRight,
    TypeCast,
    Dot,

    // Values
    IdentifierValue { value: String },
    StringValue { value: String },
    NumericValue { value: String },
    BoolValue { value: bool },

    // Default
    Empty,

    // Comment
    Comment,
}

impl Token {
    pub fn binding_power(&self) -> u32 {
        match self {
            Token::And => {
                return 1;
            }
            Token::Or => {
                return 1;
            }

            Token::Equal => {
                return 2;
            }
            Token::NotEqual => {
                return 2;
            }

            Token::LessThan => {
                return 3;
            }
            Token::GreaterThan => {
                return 3;
            }

            // For some reason these are grouped separately
            Token::LessThanOrEqual => {
                return 4;
            }
            Token::GreaterThanOrEqual => {
                return 4;
            }

            Token::Plus => {
                return 5;
            }
            Token::Minus => {
                return 5;
            }

            Token::Concat => {
                return 6;
            }
            Token::Asterisk => {
                return 6;
            }
            Token::Slash => {
                return 6;
            }
            Token::Modulo => {
                return 6;
            }
            Token::Exponentiation => {
                return 6;
            }

            // Unary ops
            Token::SquareRoot => {
                return 7;
            }
            Token::CubeRoot => {
                return 7;
            }
            Token::Factorial => {
                return 7;
            }
            Token::FactorialPrefix => {
                return 7;
            }

            // Cast
            Token::TypeCast => {
                return 8;
            }

            _ => {
                return 0;
            }
        };
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Token::Semicolon
            | Token::Asterisk
            | Token::Comma
            | Token::LeftParenthesis
            | Token::RightParenthesis
            | Token::Equal
            | Token::NotEqual
            | Token::Concat
            | Token::Plus
            | Token::Minus
            | Token::Slash
            | Token::LessThan
            | Token::LessThanOrEqual
            | Token::GreaterThan
            | Token::GreaterThanOrEqual
            | Token::BitwiseAnd
            | Token::BitwiseNot
            | Token::BitwiseOr
            | Token::BitwiseXor
            | Token::AbsoluteValue
            | Token::BitwiseShiftLeft
            | Token::BitwiseShiftRight
            | Token::SquareRoot
            | Token::CubeRoot
            | Token::Factorial
            | Token::FactorialPrefix
            | Token::Exponentiation
            | Token::Modulo
            | Token::TypeCast
            | Token::Dot => {
                return true;
            }
            _ => {}
        }

        false
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            Token::As
            | Token::From
            | Token::Into
            | Token::Values
            | Token::Insert
            | Token::Select
            | Token::Create
            | Token::Where
            | Token::Table
            | Token::Drop
            | Token::And
            | Token::Or
            | Token::Not
            | Token::True
            | Token::False
            | Token::Join
            | Token::Left
            | Token::Right
            | Token::Inner
            | Token::Is
            | Token::Limit
            | Token::Offset
            | Token::Int
            | Token::BigInt
            | Token::SmallInt
            | Token::Real
            | Token::DoublePrecision
            | Token::Double
            | Token::Precision
            | Token::Varchar
            | Token::Text
            | Token::Char
            | Token::Bool
            | Token::Unique
            | Token::Index
            | Token::On
            | Token::Primary
            | Token::Key
            | Token::Null
            | Token::Alter
            | Token::Delete
            | Token::Update
            | Token::Constraint
            | Token::Foreign
            | Token::Distinct
            | Token::Order
            | Token::By
            | Token::OrderBy
            | Token::Outer
            | Token::Full => {
                return true;
            }
            _ => {}
        }

        false
    }

    pub fn is_datatype(&self) -> bool {
        match self {
            Token::Int
            | Token::BigInt
            | Token::SmallInt
            | Token::Text
            | Token::Varchar
            | Token::Real
            | Token::DoublePrecision
            | Token::Char
            | Token::Bool => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LexingError {
    General { msg: String, loc: TokenLocation },
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LexingError::General { msg, loc: _ } => msg.clone(),
            }
        )
    }
}

// for storing SQL reserved keywords
pub type Keyword = &'static str;

pub const WHERE_KEYWORD: Keyword = "where";
pub const SELECT_KEYWORD: Keyword = "select";
pub const FROM_KEYWORD: Keyword = "from";
pub const AS_KEYWORD: Keyword = "as";
pub const TABLE_KEYWORD: Keyword = "table";
pub const CREATE_KEYWORD: Keyword = "create";
pub const INSERT_KEYWORD: Keyword = "insert";
pub const INTO_KEYWORD: Keyword = "into";
pub const VALUES_KEYWORD: Keyword = "values";
pub const INT_KEYWORD: Keyword = "int";
pub const TEXT_KEYWORD: Keyword = "text";
pub const DROP_KEYWORD: Keyword = "drop";
pub const BOOL_KEYWORD: Keyword = "boolean";
pub const AND_KEYWORD: Keyword = "and";
pub const OR_KEYWORD: Keyword = "or";
pub const NOT_KEYWORD: Keyword = "not";
pub const TRUE_KEYWORD: Keyword = "true";
pub const FALSE_KEYWORD: Keyword = "false";
pub const UNIQUE_KEYWORD: Keyword = "unique";
pub const INDEX_KEYWORD: Keyword = "index";
pub const ON_KEYWORD: Keyword = "on";
pub const PRIMARY_KEYWORD: Keyword = "primary";
pub const KEY_KEYWORD: Keyword = "key";
pub const NULL_KEYWORD: Keyword = "null";
pub const ALTER_KEYWORD: Keyword = "alter";
pub const DELETE_KEYWORD: Keyword = "delete";
pub const UPDATE_KEYWORD: Keyword = "update";
pub const JOIN_KEYWORD: Keyword = "join";
pub const INNER_KEYWORD: Keyword = "inner";
pub const RIGHT_KEYWORD: Keyword = "right";
pub const LEFT_KEYWORD: Keyword = "left";
pub const CONSTRAINT_KEYWORD: Keyword = "constraint";
pub const FOREIGN_KEYWORD: Keyword = "foreign";
pub const IS_KEYWORD: Keyword = "is";
pub const BIGINT_KEYWORD: Keyword = "bigint";
pub const SMALLINT_KEYWORD: Keyword = "smallint";
pub const REAL_KEYWORD: Keyword = "real";
pub const DOUBLE_KEYWORD: Keyword = "double";
pub const PRECISION_KEYWORD: Keyword = "precision";
pub const VARCHAR_KEYWORD: Keyword = "varchar";
pub const CHAR_KEYWORD: Keyword = "char";
pub const DISTINCT_KEYWORD: Keyword = "distinct";
pub const ORDER_KEYWORD: Keyword = "order";
pub const BY_KEYWORD: Keyword = "by";
pub const ASC_KEYWORD: Keyword = "asc";
pub const DESC_KEYWORD: Keyword = "desc";
pub const OFFSET_KEYWORD: Keyword = "offset";
pub const LIMIT_KEYWORD: Keyword = "limit";
pub const OUTER_KEYWORD: Keyword = "outer";
pub const FULL_KEYWORD: Keyword = "full";
// new
pub const DECIMAL_KEYWORD: Keyword = "decimal";
pub const NUMERIC_KEYWORD: Keyword = "numeric";

// for storing SQL syntax
pub type Symbol = &'static str;

pub const SEMICOLON_SYMBOL: Symbol = ";";
pub const ASTERISK_SYMBOL: Symbol = "*";
pub const COMMA_SYMBOL: Symbol = ",";
pub const LEFT_PARENTHESIS_SYMBOL: Symbol = "(";
pub const RIGHT_PARENTHESIS_SYMBOL: Symbol = ")";
pub const EQUAL_SYMBOL: Symbol = "=";
pub const NOT_EQUAL_SYMBOL: Symbol = "<>";
pub const NOT_EQUAL_SYMBOL_2: Symbol = "!=";
pub const CONCAT_SYMBOL: Symbol = "||";
pub const PLUS_SYMBOL: Symbol = "+";
pub const MINUS_SYMBOL: Symbol = "-";
pub const SLASH_SYMBOL: Symbol = "/";
pub const LESS_THAN_SYMBOL: Symbol = "<";
pub const LESS_THAN_OR_EQUAL_SYMBOL: Symbol = "<=";
pub const GREATER_THAN_SYMBOL: Symbol = ">";
pub const GREATER_THAN_OR_EQUAL_SYMBOL: Symbol = ">=";
pub const MODULO_SYMBOL: Symbol = "%";
pub const EXPONENTIATION_SYMBOL: Symbol = "^";
pub const SQUARE_ROOT_SYMBOL: Symbol = "|/";
pub const CUBE_ROOT_SYMBOL: Symbol = "||/";
pub const FACTORIAL_SYMBOL: Symbol = "!";
pub const FACTORIAL_PREFIX_SYMBOL: Symbol = "!!";
pub const ABS_SYMBOL: Symbol = "@";
pub const BITWISE_AND_SYMBOL: Symbol = "&";
pub const BITWISE_OR_SYMBOL: Symbol = "|";
pub const BITWISE_XOR_SYMBOL: Symbol = "#";
pub const BITWISE_NOT_SYMBOL: Symbol = "~";
pub const BITWISE_SHIFT_LEFT_SYMBOL: Symbol = "<<";
pub const BITWISE_SHIFT_RIGHT_SYMBOL: Symbol = ">>";
pub const TYPE_CAST_SYMBOL: Symbol = "::";
pub const DOT_SYMBOL: Symbol = ".";

impl TokenContainer {
    #[inline]
    pub fn equals(&self, other: &Self) -> bool {
        self.token == other.token
    }

    #[inline]
    pub fn binding_power(&self) -> u32 {
        match self.token {
            Token::And => {
                return 1;
            }
            Token::Or => {
                return 1;
            }

            Token::Equal => {
                return 2;
            }
            Token::NotEqual => {
                return 2;
            }

            Token::LessThan => {
                return 3;
            }
            Token::GreaterThan => {
                return 3;
            }

            // For some reason these are grouped separately
            Token::LessThanOrEqual => {
                return 4;
            }
            Token::GreaterThanOrEqual => {
                return 4;
            }

            Token::Plus => {
                return 5;
            }
            Token::Minus => {
                return 5;
            }

            Token::Concat => {
                return 6;
            }
            Token::Asterisk => {
                return 6;
            }
            Token::Slash => {
                return 6;
            }
            Token::Modulo => {
                return 6;
            }
            Token::Exponentiation => {
                return 6;
            }

            // Unary ops
            Token::SquareRoot => {
                return 7;
            }
            Token::CubeRoot => {
                return 7;
            }
            Token::Factorial => {
                return 7;
            }
            Token::FactorialPrefix => {
                return 7;
            }

            _ => {
                return 0;
            }
        };
    }
}

pub type LexerFn = fn(&Lexer, &str, Cursor) -> Option<(TokenContainer, Cursor)>;

pub struct Lexer {
    // Syntax that should be kept
    symbols: Vec<String>,
    keywords: Vec<String>,
    max_keyword_length: usize,
    max_symbol_length: usize,
}

impl Lexer {
    pub fn new() -> Self {
        let symbols = vec![
            COMMA_SYMBOL.to_string(),
            NOT_EQUAL_SYMBOL.to_string(),
            NOT_EQUAL_SYMBOL_2.to_string(),
            LESS_THAN_OR_EQUAL_SYMBOL.to_string(),
            GREATER_THAN_OR_EQUAL_SYMBOL.to_string(),
            EQUAL_SYMBOL.to_string(),
            LEFT_PARENTHESIS_SYMBOL.to_string(),
            RIGHT_PARENTHESIS_SYMBOL.to_string(),
            SEMICOLON_SYMBOL.to_string(),
            PLUS_SYMBOL.to_string(),
            MINUS_SYMBOL.to_string(),
            ASTERISK_SYMBOL.to_string(),
            SLASH_SYMBOL.to_string(),
            BITWISE_SHIFT_LEFT_SYMBOL.to_string(),
            BITWISE_SHIFT_RIGHT_SYMBOL.to_string(),
            LESS_THAN_SYMBOL.to_string(),
            GREATER_THAN_SYMBOL.to_string(),
            CONCAT_SYMBOL.to_string(),
            MODULO_SYMBOL.to_string(),
            EXPONENTIATION_SYMBOL.to_string(),
            CUBE_ROOT_SYMBOL.to_string(),
            SQUARE_ROOT_SYMBOL.to_string(),
            FACTORIAL_SYMBOL.to_string(),
            FACTORIAL_PREFIX_SYMBOL.to_string(),
            ABS_SYMBOL.to_string(),
            BITWISE_AND_SYMBOL.to_string(),
            BITWISE_OR_SYMBOL.to_string(),
            BITWISE_XOR_SYMBOL.to_string(),
            BITWISE_NOT_SYMBOL.to_string(),
            TYPE_CAST_SYMBOL.to_string(),
            DOT_SYMBOL.to_string(),
        ];
        let keywords = vec![
            SELECT_KEYWORD.to_string(),
            INSERT_KEYWORD.to_string(),
            VALUES_KEYWORD.to_string(),
            TABLE_KEYWORD.to_string(),
            CREATE_KEYWORD.to_string(),
            DROP_KEYWORD.to_string(),
            WHERE_KEYWORD.to_string(),
            FROM_KEYWORD.to_string(),
            TEXT_KEYWORD.to_string(),
            BOOL_KEYWORD.to_string(),
            AND_KEYWORD.to_string(),
            ORDER_KEYWORD.to_string(),
            OR_KEYWORD.to_string(),
            DESC_KEYWORD.to_string(),
            ASC_KEYWORD.to_string(),
            AS_KEYWORD.to_string(),
            TRUE_KEYWORD.to_string(),
            FALSE_KEYWORD.to_string(),
            JOIN_KEYWORD.to_string(),
            INNER_KEYWORD.to_string(),
            LEFT_KEYWORD.to_string(),
            RIGHT_KEYWORD.to_string(),
            OUTER_KEYWORD.to_string(),
            FULL_KEYWORD.to_string(),
            IS_KEYWORD.to_string(),
            LIMIT_KEYWORD.to_string(),
            OFFSET_KEYWORD.to_string(),
            BY_KEYWORD.to_string(),
            DISTINCT_KEYWORD.to_string(),
            INTO_KEYWORD.to_string(),
            INT_KEYWORD.to_string(),
            BIGINT_KEYWORD.to_string(),
            SMALLINT_KEYWORD.to_string(),
            REAL_KEYWORD.to_string(),
            DOUBLE_KEYWORD.to_string(),
            PRECISION_KEYWORD.to_string(),
            VARCHAR_KEYWORD.to_string(),
            CHAR_KEYWORD.to_string(),
            UNIQUE_KEYWORD.to_string(),
            INDEX_KEYWORD.to_string(),
            ON_KEYWORD.to_string(),
            PRIMARY_KEYWORD.to_string(),
            KEY_KEYWORD.to_string(),
            NULL_KEYWORD.to_string(),
            ALTER_KEYWORD.to_string(),
            DELETE_KEYWORD.to_string(),
            UPDATE_KEYWORD.to_string(),
            CONSTRAINT_KEYWORD.to_string(),
            FOREIGN_KEYWORD.to_string(),
        ];
        let max_symbol_length =
            symbols.iter().fold(
                0,
                |acc, item| {
                    if item.len() > acc {
                        item.len()
                    } else {
                        acc
                    }
                },
            );
        let max_keyword_length =
            keywords.iter().fold(
                0,
                |acc, item| {
                    if item.len() > acc {
                        item.len()
                    } else {
                        acc
                    }
                },
            );
        Lexer {
            symbols,
            keywords,
            max_symbol_length,
            max_keyword_length,
        }
    }

    // lex splits an input string into a list of tokens. This process
    // can be divided into following tasks:
    //
    // 1. Instantiating a cursor with pointing to the start of the string
    //
    // 2. Execute all the lexers in series.
    //
    // 3. If any of the lexer generate a token then add the token to the
    // token slice, update the cursor and restart the process from the new
    pub fn lex(&self, source: &str) -> Result<Vec<TokenContainer>, LexingError> {
        let mut tokens = Vec::with_capacity(100);
        let mut cur: Cursor = Cursor {
            pointer: 0,
            loc: TokenLocation { line: 0, col: 0 },
        };

        'lex: while cur.pointer < source.len() {
            if let Some((token, new_cursor)) = self.lex_keyword(source, cur.clone()) {
                cur = new_cursor;

                // Omit empty tokens for valid, but empty syntax like newlines
                if token.token != Token::Empty {
                    if let Some(TokenContainer {
                        token: Token::Order,
                        loc,
                    }) = tokens.last()
                    {
                        if token.token == Token::By {
                            let token_cur = tokens.len() - 1;
                            tokens[token_cur] = TokenContainer {
                                token: Token::OrderBy,
                                loc: loc.clone(),
                            };
                            continue 'lex;
                        }
                    }
                    if let Some(TokenContainer {
                        token: Token::Double,
                        loc,
                    }) = tokens.last()
                    {
                        if token.token == Token::Precision {
                            let token_cur = tokens.len() - 1;
                            tokens[token_cur] = TokenContainer {
                                token: Token::DoublePrecision,
                                loc: loc.clone(),
                            };
                            continue 'lex;
                        }
                    }

                    tokens.push(token);
                }
                continue 'lex;
            } else if let Some((_, new_cursor)) = self.lex_comment(source, cur.clone()) {
                cur = new_cursor;
                continue 'lex;
            } else if let Some((token, new_cursor)) = self.lex_symbol(source, cur.clone()) {
                cur = new_cursor;

                // Omit empty tokens for valid, but empty syntax like newlines
                if token.token != Token::Empty {
                    tokens.push(token);
                }
                continue 'lex;
            } else if let Some((token, new_cursor)) = self.lex_numeric(source, cur.clone()) {
                cur = new_cursor;

                // Omit empty tokens for valid, but empty syntax like newlines
                if token.token != Token::Empty {
                    tokens.push(token);
                }
                continue 'lex;
            } else if let Some((token, new_cursor)) = self.lex_identifier(source, cur.clone()) {
                cur = new_cursor;

                // Omit empty tokens for valid, but empty syntax like newlines
                if token.token != Token::Empty {
                    tokens.push(token);
                }
                continue 'lex;
            } else if let Some((token, new_cursor)) = self.lex_string(source, cur.clone()) {
                cur = new_cursor;

                // Omit empty tokens for valid, but empty syntax like newlines
                if token.token != Token::Empty {
                    tokens.push(token);
                }
                continue 'lex;
            }
            let mut hint = "".to_owned();

            if tokens.len() > 0 {
                hint = "after ".to_owned();
                hint.push_str(format!("{:?}", &tokens[tokens.len() - 1].token).as_str());
            }
            let loc = get_location_from_cursor(source, cur.pointer);
            let error = LexingError::General {
                msg: format!("Unable to lex token {}, at {}:{}", hint, loc.line, loc.col),
                loc,
            };
            return Err(error);
        }
        Ok(tokens)
    }

    pub fn lex_comment(&self, source: &str, ic: Cursor) -> Option<(TokenContainer, Cursor)> {
        let mut cur = ic.clone();
        if source[cur.pointer..].starts_with("/*") {
            cur.pointer += 2;
            let mut char_iter = source[cur.pointer..].chars().peekable();
            while let Some(c) = char_iter.next() {
                cur.pointer += 1;
                if c == '\n' {
                    cur.loc.col = 0;
                    cur.loc.line += 1;
                } else if c == '*' && char_iter.peek() == Some(&'/') {
                    cur.pointer += 2;
                    break;
                }
            }
            return Some((
                TokenContainer {
                    token: Token::Comment,
                    loc: ic.loc,
                },
                cur,
            ));
        }
        if source[cur.pointer..].starts_with("--") {
            cur.pointer += 2;
            let mut char_iter = source[cur.pointer..].chars().peekable();
            while let Some(c) = char_iter.next() {
                cur.pointer += 1;
                if c == '\n' {
                    cur.loc.col = 0;
                    cur.loc.line += 1;
                    break;
                }
            }
            return Some((
                TokenContainer {
                    token: Token::Comment,
                    loc: ic.loc,
                },
                cur,
            ));
        }
        None
    }

    pub fn lex_numeric(&self, source: &str, ic: Cursor) -> Option<(TokenContainer, Cursor)> {
        let mut cur = ic.clone();

        let mut period_found = false;
        let mut exp_marker_found = false;

        let mut char_iter = source[cur.pointer..].chars().peekable();

        while let Some(c) = char_iter.next() {
            cur.loc.col += 1;

            let is_digit = is_char_digit(c);
            let is_period = c == '.';
            let is_exp_marker = c == 'e';

            // Must start with a digit or period
            if cur.pointer == ic.pointer {
                if !is_digit && !is_period {
                    return None;
                }

                period_found = is_period;
                cur.pointer += 1;
                continue;
            }

            if is_period {
                if period_found {
                    return None;
                }

                period_found = true;
                cur.pointer += 1;
                continue;
            }

            if is_exp_marker && char_iter.peek().is_some() {
                if exp_marker_found {
                    return None;
                }

                // No periods allowed after expMarker
                period_found = true;
                exp_marker_found = true;

                // exp_marker must be followed by digits
                if cur.pointer == (source.len() - 1) {
                    return None;
                }

                let c_next = *match char_iter.peek() {
                    None => {
                        return None;
                    }
                    Some(value) => value,
                };
                if c_next == '-' || c_next == '+' {
                    cur.pointer += 1;
                    cur.loc.col += 1;
                    char_iter.next();
                }

                cur.pointer += 1;
                continue;
            }

            if !is_digit {
                break;
            }

            cur.pointer += 1;
        }

        if cur.pointer == ic.pointer {
            return None;
        }
        Some((
            TokenContainer {
                loc: ic.loc,
                token: Token::NumericValue {
                    value: source[ic.pointer..cur.pointer].to_owned(),
                },
            },
            cur,
        ))
    }

    // lexCharacterDelimited looks through a source string starting at the
    // given cursor to find a start- and end- delimiter. The delimiter can
    // be escaped be preceeding the delimiter with itself.
    pub fn lex_character_delimited(
        &self,
        source: &str,
        ic: Cursor,
        delimiter: char,
        kind: TokenKind,
    ) -> Option<(TokenContainer, Cursor)> {
        let mut cur = ic.clone();

        if source[cur.pointer..].len() == 0 {
            return None;
        }

        let first_char = match get_chat_at(source, cur.pointer) {
            None => {
                return None;
            }
            Some(value) => value,
        };

        if first_char != delimiter {
            return None;
        }

        cur.loc.col += 1;
        cur.pointer += 1;

        let mut value: String = "".to_owned();

        let mut char_iter = source[cur.pointer..].chars().peekable();

        while let Some(c) = char_iter.next() {
            if c == delimiter {
                cur.pointer += 1;
                cur.loc.col += 1;
                // SQL escapes are via double characters, not backslash.
                match char_iter.peek() {
                    None => {
                        return Some((
                            TokenContainer {
                                loc: ic.loc,
                                token: if kind == TokenKind::String {
                                    Token::StringValue { value }
                                } else {
                                    Token::IdentifierValue { value }
                                },
                            },
                            cur,
                        ));
                    }
                    Some(char) => {
                        if *char != delimiter {
                            return Some((
                                TokenContainer {
                                    loc: ic.loc,
                                    token: if kind == TokenKind::String {
                                        Token::StringValue { value }
                                    } else {
                                        Token::IdentifierValue { value }
                                    },
                                },
                                cur,
                            ));
                        } else if *char == delimiter {
                            char_iter.next();
                        } else {
                            value.push(delimiter);
                        }
                    }
                }
            }

            value.push(c);
            cur.loc.col += 1;
            cur.pointer += 1;
        }

        None
    }

    pub fn lex_string(&self, source: &str, ic: Cursor) -> Option<(TokenContainer, Cursor)> {
        return self.lex_character_delimited(source, ic, '\'', TokenKind::String);
    }

    // longestMatch iterates through a source string starting at the given
    // cursor to find the longest matching substring among the provided
    // options
    pub fn longest_match(
        &self,
        source: &str,
        ic: Cursor,
        options: &Vec<String>,
        max_length: Option<usize>,
    ) -> String {
        let mut text_match: String = "".to_string();
        let cur = ic.clone();

        let rest_of_text = if let Some(mut max_length) = max_length {
            if cur.pointer + max_length > source.len() {
                max_length = source.len() - cur.pointer;
            }
            source[cur.pointer..(cur.pointer + max_length)].to_lowercase()
        } else {
            source[cur.pointer..].to_lowercase()
        };

        for option in options {
            if option.len() > text_match.len() && rest_of_text.starts_with(option) {
                text_match = option.to_string();
            }
        }
        return text_match;
    }

    pub fn lex_symbol(&self, source: &str, ic: Cursor) -> Option<(TokenContainer, Cursor)> {
        let c = match get_chat_at(source, ic.pointer) {
            None => {
                return None;
            }
            Some(value) => value,
        };
        let mut cur = ic.clone();

        // Will get overwritten later if not an ignored syntax
        cur.pointer += 1;
        cur.loc.col += 1;

        // Syntax that should be thrown away
        if c == '\n' {
            cur.loc.line += 1;
            cur.loc.col = 0;
        }

        match c {
            ' ' | '\n' | '\r' | '\t' => {
                return Some((
                    TokenContainer {
                        token: Token::Empty,
                        loc: TokenLocation { line: 0, col: 0 },
                    },
                    cur,
                ));
            }
            _ => {}
        }

        // Use `ic`, not `cur`
        let symbol_match = self.longest_match(
            source,
            ic.clone(),
            &self.symbols,
            Some(self.max_symbol_length),
        );
        // Unknown character
        if symbol_match == "" {
            return None;
        }
        // != is rewritten as <>: https://www.postgresql.org/docs/9.5/functions-comparison.html
        let kind = match symbol_match.as_str() {
            COMMA_SYMBOL => Token::Comma,
            EQUAL_SYMBOL => Token::Equal,
            NOT_EQUAL_SYMBOL | NOT_EQUAL_SYMBOL_2 => Token::NotEqual,
            ASTERISK_SYMBOL => Token::Asterisk,
            LEFT_PARENTHESIS_SYMBOL => Token::LeftParenthesis,
            RIGHT_PARENTHESIS_SYMBOL => Token::RightParenthesis,
            LESS_THAN_SYMBOL => Token::LessThan,
            LESS_THAN_OR_EQUAL_SYMBOL => Token::LessThanOrEqual,
            GREATER_THAN_SYMBOL => Token::GreaterThan,
            GREATER_THAN_OR_EQUAL_SYMBOL => Token::GreaterThanOrEqual,
            PLUS_SYMBOL => Token::Plus,
            MINUS_SYMBOL => Token::Minus,
            SLASH_SYMBOL => Token::Slash,
            MODULO_SYMBOL => Token::Modulo,
            EXPONENTIATION_SYMBOL => Token::Exponentiation,
            TYPE_CAST_SYMBOL => Token::TypeCast,
            SQUARE_ROOT_SYMBOL => Token::SquareRoot,
            CUBE_ROOT_SYMBOL => Token::CubeRoot,
            FACTORIAL_SYMBOL => Token::Factorial,
            FACTORIAL_PREFIX_SYMBOL => Token::FactorialPrefix,
            ABS_SYMBOL => Token::AbsoluteValue,
            BITWISE_AND_SYMBOL => Token::BitwiseAnd,
            BITWISE_OR_SYMBOL => Token::BitwiseOr,
            BITWISE_XOR_SYMBOL => Token::BitwiseXor,
            BITWISE_NOT_SYMBOL => Token::BitwiseNot,
            BITWISE_SHIFT_LEFT_SYMBOL => Token::BitwiseShiftLeft,
            BITWISE_SHIFT_RIGHT_SYMBOL => Token::BitwiseShiftRight,
            SEMICOLON_SYMBOL => Token::Semicolon,
            DOT_SYMBOL => Token::Dot,
            CONCAT_SYMBOL => Token::Concat,
            _ => {
                return None;
            }
        };

        cur.pointer = ic.pointer + symbol_match.len();
        cur.loc.col = ic.loc.col + symbol_match.len();

        Some((
            TokenContainer {
                loc: ic.loc,
                token: kind,
            },
            cur,
        ))
    }

    pub fn lex_keyword(&self, source: &str, ic: Cursor) -> Option<(TokenContainer, Cursor)> {
        let mut cur = ic.clone();

        let keyword_match = self.longest_match(
            source,
            ic.clone(),
            &self.keywords,
            Some(self.max_keyword_length),
        );
        if keyword_match == "" {
            return None;
        }
        cur.pointer = ic.pointer + keyword_match.len();
        cur.loc.col = ic.loc.col + keyword_match.len();
        // Check if the word continues, thus being an identifier
        if let Some(next_char) = source.chars().nth(cur.pointer) {
            if is_char_valid_for_identifier(next_char) {
                return None;
            }
        }

        let mut kind = match keyword_match.as_str() {
            SELECT_KEYWORD => Token::Select,
            FROM_KEYWORD => Token::From,
            WHERE_KEYWORD => Token::Where,
            AND_KEYWORD => Token::And,
            ORDER_KEYWORD => Token::Order,
            OR_KEYWORD => Token::Or,
            NOT_KEYWORD => Token::Not,
            DESC_KEYWORD => Token::Desc,
            ASC_KEYWORD => Token::Asc,
            AS_KEYWORD => Token::As,
            TRUE_KEYWORD => Token::True,
            FALSE_KEYWORD => Token::False,
            NULL_KEYWORD => Token::Null,
            JOIN_KEYWORD => Token::Join,
            INNER_KEYWORD => Token::Inner,
            LEFT_KEYWORD => Token::Left,
            RIGHT_KEYWORD => Token::Right,
            OUTER_KEYWORD => Token::Outer,
            FULL_KEYWORD => Token::Full,
            IS_KEYWORD => Token::Is,
            LIMIT_KEYWORD => Token::Limit,
            OFFSET_KEYWORD => Token::Offset,
            BY_KEYWORD => Token::By,
            DISTINCT_KEYWORD => Token::Distinct,
            CONSTRAINT_KEYWORD => Token::Constraint,
            ON_KEYWORD => Token::On,
            INT_KEYWORD => Token::Int,
            TEXT_KEYWORD => Token::Text,
            BIGINT_KEYWORD => Token::BigInt,
            SMALLINT_KEYWORD => Token::SmallInt,
            REAL_KEYWORD => Token::Real,
            DOUBLE_KEYWORD => Token::Double,
            PRECISION_KEYWORD => Token::Precision,
            BOOL_KEYWORD => Token::Bool,
            INSERT_KEYWORD => Token::Insert,
            VALUES_KEYWORD => Token::Values,
            INTO_KEYWORD => Token::Into,
            CREATE_KEYWORD => Token::Create,
            TABLE_KEYWORD => Token::Table,
            DROP_KEYWORD => Token::Drop,
            ALTER_KEYWORD => Token::Alter,
            DELETE_KEYWORD => Token::Delete,
            UPDATE_KEYWORD => Token::Update,
            PRIMARY_KEYWORD => Token::Primary,
            KEY_KEYWORD => Token::Key,
            UNIQUE_KEYWORD => Token::Unique,
            INDEX_KEYWORD => Token::Index,
            FOREIGN_KEYWORD => Token::Foreign,
            _ => {
                return None;
            }
        };

        if keyword_match == TRUE_KEYWORD.to_owned() || keyword_match == FALSE_KEYWORD.to_owned() {
            kind = Token::BoolValue {
                value: if keyword_match == TRUE_KEYWORD.to_owned() {
                    true
                } else {
                    false
                },
            };
        }

        if keyword_match == NULL_KEYWORD.to_owned() {
            kind = Token::Null;
        }

        Some((
            TokenContainer {
                loc: ic.loc,
                token: kind,
            },
            cur,
        ))
    }

    pub fn lex_identifier(&self, source: &str, ic: Cursor) -> Option<(TokenContainer, Cursor)> {
        // Handle separately if is a double-quoted identifier
        let token_result =
            self.lex_character_delimited(source, ic.clone(), '"', TokenKind::Identifier);
        if token_result.is_some() {
            return token_result;
        }
        if let Some(res) =
            self.lex_character_delimited(source, ic.clone(), '"', TokenKind::Identifier)
        {
            return Some(res);
        }

        let mut cur = ic.clone();
        let c = match get_chat_at(source, ic.pointer) {
            None => {
                return None;
            }
            Some(value) => value,
        };

        // Other characters count too, but ignoring non-ascii for now
        if !is_char_alphabetical(c) {
            return None;
        }

        cur.pointer += 1;
        cur.loc.col += 1;

        let mut value: String = format!("{}", c);

        for c in source[cur.pointer..].chars() {
            // Other characters count too, big ignoring non-ascii for now
            if is_char_valid_for_identifier(c) {
                value.push(c);
                cur.pointer += 1;
                cur.loc.col += 1;
                continue;
            }

            break;
        }

        if value.len() == 0 {
            return None;
        }

        Some((
            TokenContainer {
                // Unquoted identifiers are case-insensitive
                loc: ic.loc,
                token: Token::IdentifierValue {
                    value: value.to_lowercase(),
                },
            },
            cur,
        ))
    }
}

pub fn get_location_from_cursor(source: &str, cursor: usize) -> TokenLocation {
    let rev_pos = source[..(cursor + 1)]
        .chars()
        .rev()
        .collect::<String>()
        .find('\n');
    let mut col = cursor;
    match rev_pos {
        Some(rev_pos) => {
            col = source[..(cursor + 1)].len() - rev_pos;
        }
        _ => {}
    }
    TokenLocation {
        col,
        line: source[..(cursor + 1)].matches('\n').count(),
    }
}

#[inline]
fn get_chat_at(source: &str, position: usize) -> Option<char> {
    source[position..(position + 1)].chars().nth(0)
}

#[inline]
fn is_char_alphabetical(c: char) -> bool {
    (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
}

#[inline]
fn is_char_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[inline]
fn is_char_valid_for_identifier(c: char) -> bool {
    is_char_alphabetical(c) || is_char_digit(c) || c == '$' || c == '_'
}

#[cfg(test)]
mod lexer_tests {
    use super::super::lexer::*;

    struct LexerTest {
        expected_result: bool,
        expected_value: Token,
        value: &'static str,
    }

    fn run_lexer_tests(lex_fn: LexerFn, tests: Vec<LexerTest>, lexer_name: &str) {
        let lexer = Lexer::new();
        let mut found_faults = false;
        let mut error_msg: String = "\n".to_owned();
        for test in tests {
            let lex_result = lex_fn(
                &lexer,
                test.value,
                Cursor {
                    pointer: 0,
                    loc: TokenLocation { col: 0, line: 0 },
                },
            );
            let produced_result;
            match lex_result {
                Some((res, _cur)) => {
                    produced_result = true;
                    if res.token != test.expected_value {
                        found_faults = true;
                        error_msg.push_str(
                            format!(
                                "({}): Expected to find value `{:?}`\
    for following value `{}` but got `{:?}` instead\n\n",
                                lexer_name, test.expected_value, test.value, res.token
                            )
                            .as_str(),
                        );
                    }
                }
                None => {
                    produced_result = false;
                }
            }

            if produced_result != test.expected_result {
                found_faults = true;
                if test.expected_result {
                    error_msg.push_str(
                        format!(
                            "({}): Expected to find a result for following value,\
 but it didn't `{}`\n\n",
                            lexer_name, test.value
                        )
                        .as_str(),
                    );
                } else {
                    error_msg.push_str(
                        format!(
                            "({}): Expected to fail finding a result for following value,\
 but it returned one `{}`\n\n",
                            lexer_name, test.value
                        )
                        .as_str(),
                    );
                }
            }
        }

        if found_faults {
            panic!(error_msg);
        }
    }

    #[test]
    fn test_token_lex_numeric() {
        let numeric_tests = vec![
            // true
            LexerTest {
                expected_result: true,
                value: "105",
                expected_value: Token::NumericValue {
                    value: "105".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "105 ",
                expected_value: Token::NumericValue {
                    value: "105".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "123.",
                expected_value: Token::NumericValue {
                    value: "123.".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "123.145",
                expected_value: Token::NumericValue {
                    value: "123.145".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "1e5",
                expected_value: Token::NumericValue {
                    value: "1e5".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "1.e21",
                expected_value: Token::NumericValue {
                    value: "1.e21".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "1.1e2",
                expected_value: Token::NumericValue {
                    value: "1.1e2".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "1.1e-2",
                expected_value: Token::NumericValue {
                    value: "1.1e-2".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "1.1e+2",
                expected_value: Token::NumericValue {
                    value: "1.1e+2".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "1e-1",
                expected_value: Token::NumericValue {
                    value: "1e-1".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: ".1",
                expected_value: Token::NumericValue {
                    value: ".1".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "4.",
                expected_value: Token::NumericValue {
                    value: "4.".to_owned(),
                },
            },
            // false
            LexerTest {
                expected_result: false,
                value: "e4",
                expected_value: Token::Empty,
            },
            LexerTest {
                expected_result: false,
                value: "1..",
                expected_value: Token::Empty,
            },
            LexerTest {
                expected_result: false,
                value: "1ee4",
                expected_value: Token::Empty,
            },
            LexerTest {
                expected_result: false,
                value: " 1",
                expected_value: Token::Empty,
            },
        ];

        run_lexer_tests(Lexer::lex_numeric, numeric_tests, "lex_numeric");
    }

    #[test]
    fn test_token_lex_string() {
        let string_tests = vec![
            // true
            LexerTest {
                expected_result: true,
                value: "'abc'",
                expected_value: Token::StringValue {
                    value: "abc".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "'a'",
                expected_value: Token::StringValue {
                    value: "a".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "'a b'",
                expected_value: Token::StringValue {
                    value: "a b".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "'a b  c '",
                expected_value: Token::StringValue {
                    value: "a b  c ".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "'a b '''' c'",
                expected_value: Token::StringValue {
                    value: "a b '' c".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "'a''b'",
                expected_value: Token::StringValue {
                    value: "a'b".to_owned(),
                },
            },
            // false
            LexerTest {
                expected_result: false,
                value: "a",
                expected_value: Token::Empty,
            },
            LexerTest {
                expected_result: false,
                value: "",
                expected_value: Token::Empty,
            },
            LexerTest {
                expected_result: false,
                value: "'",
                expected_value: Token::Empty,
            },
            LexerTest {
                expected_result: false,
                value: " 'bpp'",
                expected_value: Token::Empty,
            },
        ];

        run_lexer_tests(Lexer::lex_string, string_tests, "lex_string");
    }

    #[test]
    fn test_token_lex_symbol() {
        let symbol_tests = vec![
            // true
            LexerTest {
                expected_result: true,
                value: "= ",
                expected_value: Token::Equal,
            },
            LexerTest {
                expected_result: true,
                value: "||",
                expected_value: Token::Concat,
            },
            LexerTest {
                expected_result: true,
                value: ",",
                expected_value: Token::Comma,
            },
            // false
            LexerTest {
                expected_result: false,
                value: "a",
                expected_value: Token::Empty,
            },
        ];

        run_lexer_tests(Lexer::lex_symbol, symbol_tests, "lex_symbol");
    }

    #[test]
    fn test_token_lex_identifier() {
        let identifier_tests = vec![
            // true
            LexerTest {
                expected_result: true,
                value: "a",
                expected_value: Token::IdentifierValue {
                    value: "a".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "abc",
                expected_value: Token::IdentifierValue {
                    value: "abc".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "abc ",
                expected_value: Token::IdentifierValue {
                    value: "abc".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "abc ",
                expected_value: Token::IdentifierValue {
                    value: "abc".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "a9$",
                expected_value: Token::IdentifierValue {
                    value: "a9$".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "userName",
                expected_value: Token::IdentifierValue {
                    value: "username".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "\"userName\"",
                expected_value: Token::IdentifierValue {
                    value: "userName".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "indexed_value",
                expected_value: Token::IdentifierValue {
                    value: "indexed_value".to_owned(),
                },
            },
            LexerTest {
                expected_result: true,
                value: "unique_values",
                expected_value: Token::IdentifierValue {
                    value: "unique_values".to_owned(),
                },
            },
            // false
            LexerTest {
                expected_result: false,
                value: "\"",
                expected_value: Token::Empty,
            },
            LexerTest {
                expected_result: false,
                value: "_sddfdff",
                expected_value: Token::Empty,
            },
            LexerTest {
                expected_result: false,
                value: "9dfdfd",
                expected_value: Token::Empty,
            },
            LexerTest {
                expected_result: false,
                value: " abc",
                expected_value: Token::Empty,
            },
        ];

        run_lexer_tests(Lexer::lex_identifier, identifier_tests, "lex_identifier");
    }

    #[test]
    fn test_token_lex_keyword() {
        let keyword_tests = vec![
            // true
            LexerTest {
                expected_result: true,
                value: "select ",
                expected_value: Token::Select,
            },
            LexerTest {
                expected_result: true,
                value: "from",
                expected_value: Token::From,
            },
            LexerTest {
                expected_result: true,
                value: "as",
                expected_value: Token::As,
            },
            LexerTest {
                expected_result: true,
                value: "SELECT",
                expected_value: Token::Select,
            },
            LexerTest {
                expected_result: true,
                value: "into",
                expected_value: Token::Into,
            },
            // false
            LexerTest {
                expected_result: false,
                value: " from",
                expected_value: Token::Empty,
            },
            LexerTest {
                expected_result: false,
                value: "fdfd",
                expected_value: Token::Empty,
            },
        ];

        run_lexer_tests(Lexer::lex_keyword, keyword_tests, "lex_keyword");
    }

    struct LexTest {
        valid: bool,
        input: &'static str,
        tokens: Vec<TokenContainer>,
    }

    #[test]
    fn test_lex() {
        let lex_tests = vec![
            LexTest {
                valid: true,
                input: "select a",
                tokens: vec![
                    TokenContainer {
                        loc: TokenLocation { col: 0, line: 0 },
                        token: Token::Select,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 7, line: 0 },
                        token: Token::IdentifierValue {
                            value: "a".to_owned(),
                        },
                    },
                ],
            },
            LexTest {
                valid: true,
                input: "select true",
                tokens: vec![
                    TokenContainer {
                        loc: TokenLocation { col: 0, line: 0 },
                        token: Token::Select,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 7, line: 0 },
                        token: Token::BoolValue { value: true },
                    },
                ],
            },
            LexTest {
                valid: true,
                input: "select 1",
                tokens: vec![
                    TokenContainer {
                        loc: TokenLocation { col: 0, line: 0 },
                        token: Token::Select,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 7, line: 0 },
                        token: Token::NumericValue {
                            value: "1".to_owned(),
                        },
                    },
                ],
            },
            LexTest {
                valid: true,
                input: "select 'foo' || 'bar';",
                tokens: vec![
                    TokenContainer {
                        loc: TokenLocation { col: 0, line: 0 },
                        token: Token::Select,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 7, line: 0 },
                        token: Token::StringValue {
                            value: "foo".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 13, line: 0 },
                        token: Token::Concat,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 16, line: 0 },
                        token: Token::StringValue {
                            value: "bar".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 21, line: 0 },
                        token: Token::Semicolon,
                    },
                ],
            },
            LexTest {
                valid: true,
                input: "CREATE TABLE u (id INT, name TEXT)",
                tokens: vec![
                    TokenContainer {
                        loc: TokenLocation { col: 0, line: 0 },
                        token: Token::Create,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 7, line: 0 },
                        token: Token::Table,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 13, line: 0 },
                        token: Token::IdentifierValue {
                            value: "u".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 15, line: 0 },
                        token: Token::LeftParenthesis,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 16, line: 0 },
                        token: Token::IdentifierValue {
                            value: "id".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 19, line: 0 },
                        token: Token::Int,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 22, line: 0 },
                        token: Token::Comma,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 24, line: 0 },
                        token: Token::IdentifierValue {
                            value: "name".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 29, line: 0 },
                        token: Token::Text,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 33, line: 0 },
                        token: Token::RightParenthesis,
                    },
                ],
            },
            LexTest {
                valid: true,
                input: "insert into users values (545, 232)",
                tokens: vec![
                    TokenContainer {
                        loc: TokenLocation { col: 0, line: 0 },
                        token: Token::Insert,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 7, line: 0 },
                        token: Token::Into,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 12, line: 0 },
                        token: Token::IdentifierValue {
                            value: "users".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 18, line: 0 },
                        token: Token::Values,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 25, line: 0 },
                        token: Token::LeftParenthesis,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 26, line: 0 },
                        token: Token::NumericValue {
                            value: "545".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 30, line: 0 },
                        token: Token::Comma,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 32, line: 0 },
                        token: Token::NumericValue {
                            value: "232".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 36, line: 0 },
                        token: Token::RightParenthesis,
                    },
                ],
            },
            LexTest {
                valid: true,
                input: "SELECT id FROM users;",
                tokens: vec![
                    TokenContainer {
                        loc: TokenLocation { col: 0, line: 0 },
                        token: Token::Select,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 7, line: 0 },
                        token: Token::IdentifierValue {
                            value: "id".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 10, line: 0 },
                        token: Token::From,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 15, line: 0 },
                        token: Token::IdentifierValue {
                            value: "users".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 20, line: 0 },
                        token: Token::Semicolon,
                    },
                ],
            },
            LexTest {
                valid: true,
                input: "SELECT id, name FROM users;",
                tokens: vec![
                    TokenContainer {
                        loc: TokenLocation { col: 0, line: 0 },
                        token: Token::Select,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 7, line: 0 },
                        token: Token::IdentifierValue {
                            value: "id".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 9, line: 0 },
                        token: Token::Comma,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 11, line: 0 },
                        token: Token::IdentifierValue {
                            value: "name".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 16, line: 0 },
                        token: Token::From,
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 21, line: 0 },
                        token: Token::IdentifierValue {
                            value: "users".to_owned(),
                        },
                    },
                    TokenContainer {
                        loc: TokenLocation { col: 26, line: 0 },
                        token: Token::Semicolon,
                    },
                ],
            },
        ];

        let mut found_faults = false;
        let mut err_msg = "\n".to_owned();

        for test in lex_tests {
            let lexer = Lexer::new();
            let lex_result = lexer.lex(test.input);

            match lex_result {
                Ok(result) => {
                    if !test.valid {
                        found_faults = true;
                        err_msg.push_str(
                            &format!(
                                "For Input `{}` a failure was expected, but it passed\n\n",
                                test.input
                            )
                            .to_owned(),
                        );
                    }
                    if result.len() != test.tokens.len() {
                        found_faults = true;
                        err_msg.push_str(
                            &format!(
                                "For Input `{}` a result with `{}` tokens was expected, but one with `{}` was received\n\n",
                                test.input,
                                test.tokens.len(),
                                result.len(),
                            )
                            .to_owned(),
                        );
                    } else {
                        for i in 0..test.tokens.len() {
                            let test_token = &result[i];
                            let expected_token = &test.tokens[i];

                            if test_token.token != expected_token.token {
                                found_faults = true;
                                err_msg.push_str(
                                &format!(
                                    "For Input `{}` the token at position {} was expected to have a kind of `{:?}`\
, but one with kind `{:?}` was received\n\n",
                                    test.input,
                                    i,
                                    expected_token.token,
                                    test_token.token
                                )
                                .to_owned(),
                            );
                            }

                            if test_token.token != expected_token.token {
                                found_faults = true;
                                err_msg.push_str(
                                &format!(
                                    "For Input `{}` the token at position {} was expected to have a value of `{:?}`\
, but one with value `{:?}` was received\n\n",
                                    test.input,
                                    i,
                                    expected_token.token,
                                    test_token.token
                                )
                                .to_owned(),
                            );
                            }

                            if test_token.loc != expected_token.loc {
                                found_faults = true;
                                err_msg.push_str(
                                &format!(
                                    "For Input `{}` the token at position {} was expected to have a location of `{}:{}`\
, but one with location `{}:{}` was received\n\n",
                                    test.input,
                                    i,
                                    expected_token.loc.col,
                                    expected_token.loc.line,
                                    test_token.loc.col,
                                    test_token.loc.line,
                                )
                                .to_owned(),
                            );
                            }
                        }
                    }
                }
                Err(err) => {
                    found_faults = true;
                    if test.valid {
                        err_msg.push_str(err.to_string().as_str());
                    }
                }
            }
        }

        if found_faults {
            panic!(err_msg);
        }
    }
}

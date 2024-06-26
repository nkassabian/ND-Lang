#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN,
    COLON,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    POW,
    MODULO,

    // One or two character tokens.
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,
    PLUSEQUALS,
    MINUSEQUALS,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUNC,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    LET,
    CONST,
    I32,
    // SUPER,
    // THIS,
    TRUE,
    HAVE,
    VAR,
    WHILE,
    EOF,
}

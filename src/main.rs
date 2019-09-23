/// 位置情報。 .0から.1までの区間を表す
/// たとえばLoc(4, 6) なら入力文字の5文字目から7文字目までの区間を表す(0始まり)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

// Locに便利メソッドを実装しておく
impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

/// アノテーション。値に様々なデータを持たせたもの。ここではLocを持たせている。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TokenKind {
    /// [0-9][0-9]*
    Number(u64),
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    /// (
    LParen,
    /// )
    RParen,
}

// TokenKindにアノテーションをつけたものをTokenとして定義しておく
type Token = Annot<TokenKind>;

impl Token {
    fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }

    fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }

    fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }

    fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }

    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }

    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

/// 字句解析エラー
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;

impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }

    fn eof(loc: Loc) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}

/// 字句解析器
fn lex (input: &str) -> Result<Vec<Token>, LexError> {
    // 解析結果を保存するベクタ
    let mut tokens = Vec::new();

    // 入力
    let input = input.as_bytes();

    // 位置を管理する値
    let mut pos = 0;

    // サブレキサを呼んだあとposを更新するマクロ
    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }};
    }

    while pos < input.len() {
        // ここでそれぞれの関数にinputとposを渡す
        match input[pos]{
            b'0'...b'9' => lex_a_token!(lex_number(input, pos)),
            b'+' => lex_a_token!(lex_plus(input, pos)),
            b'-' => lex_a_token!(lex_minus(input, pos)),
            b'*' => lex_a_token!(lex_asterisk(input, pos)),
            b'/' => lex_a_token!(lex_slash(input, pos)),
            b'(' => lex_a_token!(lex_lparen(input, pos)),
            b')' => lex_a_token!(lex_rparen(input, pos)),

            // 空白を扱う
            b' ' | b'\n' | b'\t' => {
                let((),p) = skip_spaces(input, pos)?;
                pos = p;
            }

            // それ以外はエラー
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
        }
    }
    Ok(tokens)
}

/// posのバイトが期待するものであれば1バイト消費してposを1進める
fn consume_byte(input: &[u8], pos:usize, b:u8)->Result<(u8,usize),LexError>{
    // posが入力サイズ以上なら入力が終わっている
    // 1バイト期待しているのに終わっているのでエラー
    if input.len()<=pos{
        return Err(LexError::eof(Loc(pos,pos)));
    }

    // 入力が期待するものでなければエラー
    if input[pos]!=b{
        return Err(LexError::invalid_char(
            input[pos] as char,
            Loc(pos, pos+1),
        ));
    }
    Ok((b, pos+1))
}

fn lex_plus(input:&[u8], start:usize)->Result<(Token, usize), LexError>{
    // Result::mapを使うことで結果が正常だった場合の処理を簡潔に書ける
    // これはこのコードと等価
    // ```
    // match consume_byte(input, start, b'+') {
    //     Ok((_, end)) => (Token::plus(Loc(start, end)), end),
    //     Err(err) => Err(err),
    // }
    consume_byte(input, start, b'+').map(|(_, end)| (Token::plus(Loc(start, end)), end))
}

fn lex_minus(input: &[u8], start: usize) ->Result<(Token,usize), LexError>{
    consume_byte(input, start, b'-').map(|(_,end)|(Token::minus(Loc(start, end)),end))
}

fn lex_asterisk(input: &[u8], start: usize) ->Result<(Token,usize), LexError>{
    consume_byte(input, start, b'*').map(|(_,end)|(Token::asterisk(Loc(start, end)),end))
}

fn lex_slash(input: &[u8], start: usize) ->Result<(Token,usize), LexError>{
    consume_byte(input, start, b'/').map(|(_,end)|(Token::slash(Loc(start, end)),end))
}

fn lex_lparen(input: &[u8], start: usize) ->Result<(Token,usize), LexError>{
    consume_byte(input, start, b'(').map(|(_,end)|(Token::lparen(Loc(start, end)),end))
}

fn lex_rparen(input: &[u8], start: usize) ->Result<(Token,usize), LexError>{
    consume_byte(input, start, b')').map(|(_,end)|(Token::rparen(Loc(start, end)),end))
}

fn lex_number(input: &[u8], mut pos: usize) -> (Token, usize) {
    use std::str::from_utf8;

    let start = pos;
    // recognize_many を使って数値を読み込む
    let end = recognize_many(input, start, |b| b"1234567890".contains(&b));
    // 数字の列を数値に変換する
    let n = from_utf8(&input[start..pos])
        // start..posの構成からfrom_utf8は常に成功するためunwrapしても安全
        .unwrap()
        .parse()
        // 同じく構成からparseは常に成功する
        .unwrap();
    (Token::number(n, Loc(start, pos)), pos)
}

fn skip_spaces(input: &[u8], mut pos: usize)-> Result<((), usize), LexError> {
    // recognize_many を使って空白を飛ばす
    let pos = recognize_many(input, pos, |b| b" \n\t".contains(&b));
    Ok(((),pos))
}

fn recognize_many(input: &[u8], mut pos: usize, mut f: impl FnMut(u8)-> bool) -> usize {
    while pos < input.len() && f(input[pos]) {
        pos += 1;
    }
    pos
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_lexer() {
    assert_eq!(
        lex("1 + 2 * 3 - -10"),
        Ok(vec![
            Token::number(1, Loc(0, 1)),
            Token::plus(Loc(2, 3)),
            Token::number(2, Loc(4, 5)),
            Token::asterisk(Loc(6, 7)),
            Token::number(3, Loc(8, 9)),
            Token::minus(Loc(10, 11)),
            Token::minus(Loc(12, 13)),
            Token::number(10, Loc(13, 15)),
        ])
    )
}

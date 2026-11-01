use std::io::BufRead;

use crate::BuildError;

fn error(line: u64, message: impl Into<String>) -> BuildError {
    BuildError::Dump {
        line,
        message: message.into(),
    }
}

#[derive(Debug)]
enum Token {
    Word(String),
    Ident(String),
    Text(String),
    Symbol(u8),
}

impl Token {
    fn keyword(&self, keyword: &str) -> bool {
        matches!(self, Self::Word(word) if word.eq_ignore_ascii_case(keyword))
    }

    fn name(self) -> Option<String> {
        match self {
            Self::Word(name) | Self::Ident(name) => Some(name),
            _ => None,
        }
    }
}

struct Lexer<R> {
    reader: R,
    line: u64,
    lookahead: Option<(Token, u64)>,
    utf8: [u8; 4],
    utf8_len: usize,
    utf8_expected: usize,
}

impl<R: BufRead> Lexer<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            line: 1,
            lookahead: None,
            utf8: [0; 4],
            utf8_len: 0,
            utf8_expected: 0,
        }
    }

    fn peek_byte(&mut self) -> Result<Option<u8>, BuildError> {
        let buf = self
            .reader
            .fill_buf()
            .map_err(|e| error(self.line, format!("reading dump: {e}")))?;
        Ok(buf.first().copied())
    }

    fn byte(&mut self) -> Result<Option<u8>, BuildError> {
        let byte = self.peek_byte()?;
        if let Some(b) = byte {
            self.reader.consume(1);
            if self.utf8_expected == 0 {
                self.utf8_expected = match b {
                    0..=0x7f => 0,
                    0xc2..=0xdf => 2,
                    0xe0..=0xef => 3,
                    0xf0..=0xf4 => 4,
                    _ => return Err(error(self.line, "invalid UTF-8")),
                };
                if self.utf8_expected != 0 {
                    self.utf8[0] = b;
                    self.utf8_len = 1;
                }
            } else {
                if b & 0xc0 != 0x80 {
                    return Err(error(self.line, "invalid UTF-8"));
                }
                self.utf8[self.utf8_len] = b;
                self.utf8_len += 1;
                if self.utf8_len == self.utf8_expected {
                    std::str::from_utf8(&self.utf8[..self.utf8_len])
                        .map_err(|e| error(self.line, format!("invalid UTF-8: {e}")))?;
                    self.utf8_expected = 0;
                }
            }
            if b == b'\n' {
                self.line += 1;
            }
        } else if self.utf8_expected != 0 {
            return Err(error(self.line, "incomplete UTF-8 sequence"));
        }
        Ok(byte)
    }

    fn quoted(&mut self, quote: u8, line: u64) -> Result<String, BuildError> {
        let mut bytes = Vec::new();
        loop {
            match self.byte()? {
                None => return Err(error(line, "unexpected end of file")),
                Some(b) if b == quote => {
                    if self.peek_byte()? == Some(quote) {
                        self.byte()?;
                        bytes.push(quote);
                    } else {
                        break;
                    }
                }
                Some(b'\\') if quote != b'`' => {
                    let Some(escaped) = self.byte()? else {
                        return Err(error(line, "unexpected end of file"));
                    };
                    match escaped {
                        b'0' => bytes.push(0),
                        b'b' => bytes.push(8),
                        b'n' => bytes.push(b'\n'),
                        b'r' => bytes.push(b'\r'),
                        b't' => bytes.push(b'\t'),
                        b'Z' => bytes.push(26),
                        b'%' | b'_' => {
                            bytes.push(b'\\');
                            bytes.push(escaped);
                        }
                        other => bytes.push(other),
                    }
                }
                Some(b) => bytes.push(b),
            }
        }
        String::from_utf8(bytes).map_err(|e| error(line, format!("invalid UTF-8: {e}")))
    }

    fn next_token(&mut self) -> Result<Option<(Token, u64)>, BuildError> {
        if self.lookahead.is_some() {
            return Ok(self.lookahead.take());
        }
        loop {
            let Some(b) = self.byte()? else {
                return Ok(None);
            };
            let line = self.line - u64::from(b == b'\n');
            match b {
                b if b.is_ascii_whitespace() => {}
                b'#' => {
                    while let Some(c) = self.byte()? {
                        if c == b'\n' {
                            break;
                        }
                    }
                }
                b'-' if self.peek_byte()? == Some(b'-') => {
                    self.byte()?;
                    while let Some(c) = self.byte()? {
                        if c == b'\n' {
                            break;
                        }
                    }
                }
                b'/' if self.peek_byte()? == Some(b'*') => {
                    self.byte()?;
                    let mut previous = 0;
                    loop {
                        let Some(c) = self.byte()? else {
                            return Err(error(line, "unterminated block comment"));
                        };
                        if previous == b'*' && c == b'/' {
                            break;
                        }
                        previous = c;
                    }
                }
                b'\'' | b'"' => return Ok(Some((Token::Text(self.quoted(b, line)?), line))),
                b'`' => return Ok(Some((Token::Ident(self.quoted(b, line)?), line))),
                b'(' | b')' | b',' | b';' => return Ok(Some((Token::Symbol(b), line))),
                b'.' if !self.peek_byte()?.is_some_and(|c| c.is_ascii_digit()) => {
                    return Ok(Some((Token::Symbol(b), line)));
                }
                _ => {
                    let mut bytes = vec![b];
                    while let Some(c) = self.peek_byte()? {
                        if c.is_ascii_whitespace()
                            || matches!(c, b'(' | b')' | b',' | b';' | b'\'' | b'"' | b'`' | b'#')
                        {
                            break;
                        }
                        // A decimal point belongs to a numeric literal; elsewhere it separates identifiers.
                        if c == b'.' && !bytes[0].is_ascii_digit() && bytes[0] != b'-' && bytes[0] != b'+' {
                            break;
                        }
                        bytes.push(c);
                        self.byte()?;
                    }
                    let word = String::from_utf8(bytes).map_err(|e| error(line, format!("invalid UTF-8: {e}")))?;
                    return Ok(Some((Token::Word(word), line)));
                }
            }
        }
    }

    fn next(&mut self) -> Result<(Token, u64), BuildError> {
        self.next_token()?
            .ok_or_else(|| error(self.line, "unexpected end of file"))
    }

    fn peek(&mut self) -> Result<Option<&(Token, u64)>, BuildError> {
        if self.lookahead.is_none() {
            self.lookahead = self.next_token()?;
        }
        Ok(self.lookahead.as_ref())
    }

    fn symbol(&mut self, symbol: u8) -> Result<(), BuildError> {
        let (token, line) = self.next()?;
        if matches!(token, Token::Symbol(b) if b == symbol) {
            Ok(())
        } else {
            Err(error(line, format!("expected '{}'", char::from(symbol))))
        }
    }

    fn name(&mut self) -> Result<String, BuildError> {
        let (token, line) = self.next()?;
        token.name().ok_or_else(|| error(line, "expected identifier"))
    }

    fn skip_statement(&mut self) -> Result<(), BuildError> {
        loop {
            match self.next_token()? {
                Some((Token::Symbol(b';'), _)) => return Ok(()),
                Some(_) => {}
                None => return Err(error(self.line, "unterminated statement")),
            }
        }
    }
}

struct Table {
    name: String,
    columns: Vec<String>,
}

#[derive(Debug)]
enum Value {
    Null,
    Text(String),
    Number(String),
}

pub(crate) struct Row<'a> {
    columns: &'a [String],
    values: &'a [Value],
    line: u64,
}

impl Row<'_> {
    pub(crate) fn line(&self) -> u64 {
        self.line
    }

    fn value(&self, name: &str) -> Result<&Value, BuildError> {
        let index = self
            .columns
            .iter()
            .position(|column| column.eq_ignore_ascii_case(name))
            .ok_or_else(|| error(self.line, format!("missing column {name}")))?;
        self.values
            .get(index)
            .ok_or_else(|| error(self.line, format!("missing value for {name}")))
    }

    pub(crate) fn int(&self, name: &str) -> Result<i64, BuildError> {
        match self.value(name)? {
            Value::Number(value) | Value::Text(value) => value
                .parse()
                .map_err(|_| error(self.line, format!("invalid integer in {name}: {value}"))),
            Value::Null => Ok(0),
        }
    }

    pub(crate) fn real(&self, name: &str) -> Result<f64, BuildError> {
        match self.value(name)? {
            Value::Number(value) | Value::Text(value) => value
                .parse()
                .map_err(|_| error(self.line, format!("invalid real in {name}: {value}"))),
            Value::Null => Ok(0.0),
        }
    }

    pub(crate) fn text(&self, name: &str) -> Result<Option<&str>, BuildError> {
        match self.value(name)? {
            Value::Null => Ok(None),
            Value::Text(value) | Value::Number(value) => Ok((!value.is_empty()).then_some(value.as_str())),
        }
    }
}

fn validate_columns(name: &str, columns: &[String], required: &[&str], line: u64) -> Result<(), BuildError> {
    for column in required {
        if !columns.iter().any(|candidate| candidate.eq_ignore_ascii_case(column)) {
            return Err(error(line, format!("table `{name}` lacks column `{column}`")));
        }
    }
    Ok(())
}

fn create<R: BufRead>(
    lexer: &mut Lexer<R>,
    wanted: &[(&str, &[&str])],
    tables: &mut Vec<Table>,
) -> Result<(), BuildError> {
    let (token, _) = lexer.next()?;
    if !token.keyword("TABLE") {
        return lexer.skip_statement();
    }
    let mut name = lexer.name()?;
    if name.eq_ignore_ascii_case("IF") {
        let (not, line) = lexer.next()?;
        let (exists, _) = lexer.next()?;
        if !not.keyword("NOT") || !exists.keyword("EXISTS") {
            return Err(error(line, "expected IF NOT EXISTS"));
        }
        name = lexer.name()?;
    }
    let Some((_, required)) = wanted.iter().find(|(table, _)| table.eq_ignore_ascii_case(&name)) else {
        return lexer.skip_statement();
    };
    let line = lexer.line;
    lexer.symbol(b'(')?;
    let mut columns = Vec::new();
    loop {
        let (first, item_line) = lexer.next()?;
        if matches!(first, Token::Symbol(b')')) {
            break;
        }
        let is_constraint = [
            "PRIMARY",
            "UNIQUE",
            "KEY",
            "INDEX",
            "CONSTRAINT",
            "FOREIGN",
            "CHECK",
            "FULLTEXT",
            "SPATIAL",
        ]
        .iter()
        .any(|keyword| first.keyword(keyword));
        if !is_constraint {
            let column = first
                .name()
                .ok_or_else(|| error(item_line, "expected column definition"))?;
            if columns
                .iter()
                .any(|existing: &String| existing.eq_ignore_ascii_case(&column))
            {
                return Err(error(item_line, format!("duplicate column {column}")));
            }
            columns.push(column);
        }
        let mut depth = 0usize;
        loop {
            let (token, _) = lexer.next()?;
            match token {
                Token::Symbol(b'(') => depth += 1,
                Token::Symbol(b')') if depth == 0 => {
                    lexer.lookahead = Some((Token::Symbol(b')'), lexer.line));
                    break;
                }
                Token::Symbol(b')') => depth -= 1,
                Token::Symbol(b',') if depth == 0 => break,
                Token::Symbol(b';') => return Err(error(item_line, "unterminated CREATE TABLE")),
                _ => {}
            }
        }
    }
    validate_columns(&name, &columns, required, line)?;
    if tables.iter().any(|table| table.name.eq_ignore_ascii_case(&name)) {
        return Err(error(line, format!("duplicate CREATE TABLE {name}")));
    }
    tables.push(Table { name, columns });
    lexer.skip_statement()
}

fn insert<R: BufRead, F: FnMut(&str, &Row<'_>) -> Result<(), BuildError>>(
    lexer: &mut Lexer<R>,
    wanted: &[(&str, &[&str])],
    tables: &[Table],
    callback: &mut F,
) -> Result<(), BuildError> {
    let (mut token, _) = lexer.next()?;
    while token.keyword("LOW_PRIORITY")
        || token.keyword("DELAYED")
        || token.keyword("HIGH_PRIORITY")
        || token.keyword("IGNORE")
    {
        token = lexer.next()?.0;
    }
    if !token.keyword("INTO") {
        return lexer.skip_statement();
    }
    let name = lexer.name()?;
    let Some((canonical, required)) = wanted.iter().find(|(table, _)| table.eq_ignore_ascii_case(&name)) else {
        return lexer.skip_statement();
    };
    let table = tables
        .iter()
        .find(|table| table.name.eq_ignore_ascii_case(&name))
        .ok_or_else(|| error(lexer.line, format!("INSERT before CREATE TABLE {name}")))?;
    let mut explicit = Vec::new();
    if matches!(lexer.peek()?, Some((Token::Symbol(b'('), _))) {
        lexer.next()?;
        loop {
            let column = lexer.name()?;
            if !table.columns.iter().any(|known| known.eq_ignore_ascii_case(&column)) {
                return Err(error(lexer.line, format!("unknown column {column} in {name}")));
            }
            if explicit
                .iter()
                .any(|known: &String| known.eq_ignore_ascii_case(&column))
            {
                return Err(error(lexer.line, format!("duplicate column {column}")));
            }
            explicit.push(column);
            let (separator, line) = lexer.next()?;
            match separator {
                Token::Symbol(b',') => {}
                Token::Symbol(b')') => break,
                _ => return Err(error(line, "expected comma or closing parenthesis")),
            }
        }
        validate_columns(&name, &explicit, required, lexer.line)?;
    }
    let columns = if explicit.is_empty() { &table.columns } else { &explicit };
    let (values, line) = lexer.next()?;
    if !values.keyword("VALUES") && !values.keyword("VALUE") {
        return Err(error(line, "expected VALUES"));
    }
    loop {
        let (open, row_line) = lexer.next()?;
        if !matches!(open, Token::Symbol(b'(')) {
            return Err(error(row_line, "expected row"));
        }
        let mut row = Vec::with_capacity(columns.len());
        loop {
            let (token, line) = lexer.next()?;
            let value = match token {
                Token::Text(value) => Value::Text(value),
                Token::Word(value) if value.eq_ignore_ascii_case("NULL") => Value::Null,
                Token::Word(value) if value.parse::<f64>().is_ok() => Value::Number(value),
                _ => return Err(error(line, "invalid row value")),
            };
            row.push(value);
            let (separator, line) = lexer.next()?;
            match separator {
                Token::Symbol(b',') => {}
                Token::Symbol(b')') => break,
                _ => return Err(error(line, "expected comma or closing parenthesis")),
            }
        }
        if row.len() != columns.len() {
            return Err(error(
                row_line,
                format!("table {name}: expected {} values, got {}", columns.len(), row.len()),
            ));
        }
        callback(
            canonical,
            &Row {
                columns,
                values: &row,
                line: row_line,
            },
        )?;
        let (separator, line) = lexer.next()?;
        match separator {
            Token::Symbol(b',') => {}
            Token::Symbol(b';') => break,
            _ => return Err(error(line, "expected comma or semicolon after row")),
        }
    }
    Ok(())
}

pub(crate) fn read_dump<R: BufRead, F: FnMut(&str, &Row<'_>) -> Result<(), BuildError>>(
    reader: R,
    wanted: &[(&str, &[&str])],
    mut callback: F,
) -> Result<(), BuildError> {
    let mut lexer = Lexer::new(reader);
    let mut tables = Vec::new();
    while let Some((token, _)) = lexer.next_token()? {
        if matches!(token, Token::Symbol(b';')) {
            continue;
        }
        if token.keyword("CREATE") {
            create(&mut lexer, wanted, &mut tables)?;
        } else if token.keyword("INSERT") {
            insert(&mut lexer, wanted, &tables, &mut callback)?;
        } else {
            lexer.skip_statement()?;
        }
    }
    for (name, _) in wanted {
        if !tables.iter().any(|table| table.name.eq_ignore_ascii_case(name)) {
            return Err(error(lexer.line, format!("TDB dump has no table `{name}`")));
        }
    }
    Ok(())
}

#[derive(Eq, PartialEq, Debug)]
pub enum Sexp<'a> {
    Nil,
    Int(i64),
    Symbol(&'a str),
    String(&'a str),
    List(Vec<Sexp<'a>>),
    Error(Error<'a>)
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error<'a> {
    ReadError(Vec<ReadError<'a>>)
}

#[derive(Eq, PartialEq, Debug)]
pub enum ReadError<'a> {
    InvalidToken(&'a str),
    IntegerLimit(&'a str),
    PartialString(&'a str),
    TrailingDelimiter(&'a str),
    UnmatchedDelimiter
}

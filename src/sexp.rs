#[derive(Eq, PartialEq, Debug)]
pub enum Sexp<'a> {
    Nil,
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
    TrailingDelimiter(&'a str),
    UnmatchedDelimiter
}

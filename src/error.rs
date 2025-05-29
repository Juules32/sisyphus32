use thiserror::Error;

#[derive(Error, Debug)]
pub enum FenParseError {
    #[error("Couldn't find fen pieces")]
    NoPieces,
    
    #[error("Couldn't find fen side")]
    NoSide,
    
    #[error("Couldn't find fen castling rights")]
    NoCastlingRights,
    
    #[error("Couldn't find fen en-passant square")]
    NoEnPassant,
    
    #[error("Couldn't parse fen pieces: {0}")]
    Pieces(String),
    
    #[error("Couldn't parse fen side: {0}")]
    Side(String),
    
    #[error("Couldn't parse fen castling rights: {0}")]
    CastlingRights(char),
    
    #[error("Couldn't parse fen en-passant square: {0}")]
    EnPassant(#[from] SquareParseError),
    
    #[error("Couldn't parse illegal piece: {0}")]
    IllegalPiece(char),
}

#[derive(Error, Debug)]
#[error("Couldn't parse file: {0}")]
pub struct FileParseError(pub char);

#[derive(Error, Debug)]
#[error("Couldn't parse rank: {0}")]
pub struct RankParseError(pub char);

#[derive(Error, Debug)]
pub enum UciParseError {
    #[error("Couldn't parse uci keyword")]
    Keyword,

    #[error("Couldn't parse parameter: {0}")]
    Param(&'static str),

    #[error("Couldn't parse parameter value: {0}")]
    ParamValue(&'static str),

    #[error("Parameter out of range for: {0}")]
    ParamRange(&'static str),

    #[error("Couldn't parse uci option")]
    Option,

    #[error("{0}")]
    MoveStringParseError(#[from] MoveStringParseError),

    #[error("{0}")]
    FenParseError(#[from] FenParseError),

    #[error("Disabled feature: {0}")]
    DisabledFeatureError(&'static str),
}

#[derive(Error, Debug)]
pub enum MoveStringParseError {
    #[error("Illegal move string length")]
    LengthParseError,

    #[error("Illegal promotion piece")]
    PromotionPieceParseError(String),
    
    #[error("Couldn't find pseudo-legal move: {0}")]
    IllegalMove(String),

    #[error("{0}")]
    SquareParseError(#[from] SquareParseError),
}

#[derive(Error, Debug)]
pub enum SquareParseError {
    #[error("Missing file character")]
    NoFile,

    #[error("Missing rank character")]
    NoRank,

    #[error("Illegal string length for square: {0}")]
    StringLength(String),

    #[error("{0}")]
    RankParseError(#[from] RankParseError),

    #[error("{0}")]
    FileParseError(#[from] FileParseError),
}

#[derive(Error, Debug)]
pub enum BotGameError {
    #[error("Illegal UCI move")]
    IllegalUciMoveError,

    #[error("Player performed illegal action")]
    IllegalActionError,
}

use rustler::{Encoder, Env, Term};
use std::io;
use thiserror::Error;

// Defines the atoms for each value of ExplorerError.
rustler::atoms! {
    io,
    utf8,
    delta_table,
    internal,
    other,
    try_from_int,
    parquet,
    unknown
}

#[derive(Error, Debug)]
pub enum DeltaLakexError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Utf8 Conversion Error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("DeltaTable Error: {0}")]
    DeltaTable(#[from] deltalake::DeltaTableError),
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl Encoder for DeltaLakexError {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        format!("{self}").encode(env)
    }
}

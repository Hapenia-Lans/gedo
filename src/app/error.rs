/**
 * Copyright (c) 2023 hapenia
 *
 * This software is released under the MIT License.
 * https://opensource.org/licenses/MIT
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot parse `{0}`: not a valid godot version")]
    CannotParseVersion(String),
    #[error("unknown error")]
    Unknown,
}

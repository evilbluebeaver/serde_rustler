//! Constants and utilities for conversion between Rust string-likes and Elixir atoms.

use crate::Error;
use rustler::{types::atom::Atom, Encoder, Env, Term};

lazy_static! {
    pub static ref OK: String = String::from("Ok");
    pub static ref ERROR: String = String::from("Err");
}

rustler_atoms! {
    /// The atom `nil`.
    atom nil;

    /// The atom `:ok`.
    atom ok;

    /// The atom `:error`.
    atom error;

    /// The atom/Boolean `true`.
    atom true_ = "true";

    /// The atom/Boolean `false`.
    atom false_ = "false";

    /// The atom `:__struct__`.
    atom __struct__;
}

/**
 * Attempts to create an atom term from the provided string (if the atom already exists in the atom table). If not, returns a string term.
 */
pub fn str_to_term<'a>(env: &Env<'a>, string: &str) -> Result<Term<'a>, Error> {
    if string == "Ok" {
        Ok(ok().encode(*env))
    } else if string == "Err" {
        Ok(error().encode(*env))
    } else {
        match Atom::from_bytes(*env, string.as_bytes()) {
            Ok(term) => Ok(term.encode(*env)),
            _ => Err(Error::InvalidStringable),
        }
    }
}

/**
 * Attempts to create a `String` from the term.
 */
pub fn term_to_string(term: &Term) -> Result<String, Error> {
    if ok().eq(term) {
        Ok(OK.to_string())
    } else if error().eq(term) {
        Ok(ERROR.to_string())
    } else if term.is_atom() {
        term.atom_to_string().or(Err(Error::InvalidAtom))
    } else {
        Err(Error::InvalidStringable)
    }
}

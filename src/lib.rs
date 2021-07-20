//! A set of functions to parse user-provided metadata strings.
//!
//! There are many sources of music metdata that are user-generated,
//! such as songs uploaded to SoundCloud or torrent websites.
//! The song metadata from these data sources contain all sorts of
//! noisy annotations. This crate parses or removes these annotations
//! to help normalize your dataset.
//!
use std::borrow::Cow;
use std::ops::Deref;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BITRATE_REGEX: Regex = Regex::new(r"[\(|[[:punct:]]|[[:space:]]]?(?i)\d+[[:space:]]]*kbps[\)|[[:punct:]]|[[:space:]]]?").unwrap();
    static ref MP3_REGEX: Regex = Regex::new(r"[\(|[[:punct:]]|[[:space:]]]?(?i)mp3[\)|[[:punct:]]|[[:space:]]]?").unwrap();
    // Require that year annotations are encased in punctuation
    // to avoid mangling artist names and album titles that happen to have
    // numbers in them.
    static ref YEAR_REGEX: Regex = Regex::new(r"[\(|[[:punct:]]]?(?:19|20)[0-9]{2}[\)|[[:punct:]]]").unwrap();
    static ref REDUNDANT_WHITESPACE_REGEX: Regex = Regex::new(r"[[:space:]]+").unwrap();
    static ref BEGINNING_WHITESPACE_REGEX: Regex =  Regex::new(r"^[[:space:]]+").unwrap();
    static ref ENDING_WHITESPACE_REGEX: Regex =  Regex::new(r"[[:space:]]+$").unwrap();
}

/// Remove "year annotations" from strings.
///
/// A year annotation is a year encased in punctuation, such as:
/// - `[2019]`
/// - `(1997)`
///
fn remove_year_annotation(dirty: &str) -> Cow<'_, str> {
    YEAR_REGEX.replace_all(dirty, " ")
}

/// Remove music bitrate annotations from strings.
///
/// A bitrate annotation looks like:
/// - `(128kbps)`
///
fn remove_bitrate_annotation(dirty: &str) -> Cow<'_, str> {
    BITRATE_REGEX.replace_all(dirty, " ")
}

/// Removes the case-insensitive string `mp3` from the input string.
fn remove_mp3_format_label(dirty: &str) -> Cow<'_, str> {
    MP3_REGEX.replace_all(dirty, " ")
}

/// Removes "unnecessary" whitespace from a string.
///
/// This function removes three types of whitespace:
/// 1. A sequence of two or more whitespace symbols in the middle of a string.
/// 2. All whitespace at the beginning of a string.
/// 3. All whitespace at the end of a string.
///
fn remove_redundant_whitespace(dirty: &str) -> String {
    let dirty_1 = REDUNDANT_WHITESPACE_REGEX.replace_all(dirty, " ");
    let dirty_2 = BEGINNING_WHITESPACE_REGEX.replace_all(&dirty_1, "");
    let dirty_3 = ENDING_WHITESPACE_REGEX.replace_all(&dirty_2, "");
    dirty_3.deref().to_string()
}

/// Applies a common set of input transformations to every string.
pub fn fix_common(dirty: &str) -> String {
    let dirty_1 = remove_year_annotation(dirty);
    let dirty_2 = remove_mp3_format_label(&dirty_1);
    let dirty_3 = remove_bitrate_annotation(&dirty_2);
    let dirty_4 = remove_redundant_whitespace(&dirty_3);
    dirty_4.deref().to_string()
}

/// Clean a raw string that represents a music album title.
pub fn fix_album_title(dirty: &str) -> String {
    fix_common(dirty).to_string()
}

/// Clean a raw string that represents the title of a single music song or track.
pub fn fix_track_title(dirty: &str) -> String {
    fix_common(dirty).to_string()
}

/// Clean a raw string that represents one or more artists. Returns a vector of artist names.
pub fn fix_artists_string(dirty: &str) -> Vec<String> {
    vec![fix_common(dirty).to_string()]
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn fix_album_title_1() {
        let actual = fix_album_title("Tyler, The Creator - IGOR (2019) Mp3 (320 kbps)");
        assert_eq!(actual, "Tyler, The Creator - IGOR");
    }
    #[test]
    fn fix_album_title_2() {
        let actual = fix_album_title("Tyler, The Creator - IGOR (2019) [Mp3] (320 kbps)");
        assert_eq!(actual, "Tyler, The Creator - IGOR");
    }
}

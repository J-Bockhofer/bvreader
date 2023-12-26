use std::str::FromStr;
use std::fmt::Debug;

use regex::Regex;

/// Parses value with type T from string.
/// ## Example
/// ```
///  use regex::Regex;
///  use std::sync::OnceLock;
///  use crate::bvreader::bv_reader::generic_parser::parse_generic_value;
/// 
///  static AMP_NUMCHAN_REGEX: OnceLock<Regex> = OnceLock::new();
/// 
///  fn main() {
///     let input = "
///     A m p l i f i e r  S e t u p
///     ============================
///     Number of channels: 71
///     Sampling Rate [Hz]: 500
///     Sampling Interval [µS]: 2000";
///
///     let re = AMP_NUMCHAN_REGEX.get_or_init(|| {
///         Regex::new(r"Number of channels: (\d*)").unwrap() 
///       });
///
///     let output = parse_generic_value::<usize>(input, re).unwrap();
///     let expected = 71;
///     assert_eq!(output, expected);
///  }
/// 
/// ```
pub fn parse_generic_value<T: std::str::FromStr>(textcontent: &str, regex: &Regex) -> Option<T> where <T as FromStr>::Err: Debug {
    
      let caps = regex.captures(textcontent);
      if caps.is_some() {
          let caps = caps.unwrap();
           caps.get(1).map_or(Option::None, |m|  {
              let numchan = m.as_str().parse::<T>();
              if numchan.is_ok() {
                  Some(numchan.unwrap())
              } else {
                  Option::None
              }
           })
      } else {
          Option::None
      }
}

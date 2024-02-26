use regex::Regex;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

static TIMECODE_REGEX: OnceLock<Regex> = OnceLock::new();

/// Type of BVTime after [specification](https://www.fieldtriptoolbox.org/assets/pdf/BrainVisionCoreFileFormat_1.0_2018-08-02.pdf)
/// 
/// Contains the timecode string literal and functions to convert to and from `std::time::SystemTime`
/// 
/// The timecode is part of the markers in the marker file.
/// 
/// The date is optional. It is only evaluated if the marker
/// type is "New Segment".
/// 
/// The date has the following format:
/// 
/// 4 digits = year
/// 
/// 2 digits = month
/// 
/// 2 digits = day
/// 
/// 2 digits = hour (24-hour system)
/// 
/// 2 digits = minute
/// 
/// 2 digits = second
/// 
/// 6 digits = microsecond
/// 
/// The result is a time resolution of a microsecond.
/// 
/// Example
/// 
/// 19990311140312003012
/// 
/// means 11 March 1999, 14:03:12.003012
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct BVTime {
    pub timecode: String
}

impl BVTime{
    /// Returns Option<None> when timecode has over 20 characters
    pub fn new(timecode: String) -> Option<Self> {
        if timecode.len() != 20 {
            return None; // Invalid timecode length
        }
        Some(BVTime{timecode})
    }

    pub fn from_str(timecode: &str) -> Option<Self> {
        BVTime::new(timecode.to_string())
    }

    pub fn to_system_time(&self) -> Option<SystemTime> {
        let year: u32 = self.timecode[0..4].parse().ok()?;
        let month: u32 = self.timecode[4..6].parse().ok()?;
        let day: u32 = self.timecode[6..8].parse().ok()?;
        let hour: u32 = self.timecode[8..10].parse().ok()?;
        let min: u32 = self.timecode[10..12].parse().ok()?;
        let sec: u32 = self.timecode[12..14].parse().ok()?;
        let subsec: u32 = self.timecode[14..20].parse().ok()?;
    
        // Calculate the total seconds since UNIX_EPOCH
        let total_seconds = (year - 1970) as u64 * 31556926 + // seconds in a year
                             ((month - 1) as u64 * 2629743) +        // seconds in a month
                             ((day - 1) as u64 * 86400) +            // seconds in a day
                             (hour as u64 * 3600) +            // seconds in an hour
                             (min as u64 * 60) +               // seconds in a minute
                             sec as u64;
    
        // Create a SystemTime instance
        let timestamp = UNIX_EPOCH + std::time::Duration::from_secs(total_seconds) +
                        std::time::Duration::from_nanos(subsec as u64);
    
        Some(timestamp)
    }

    pub fn from_system_time(system_time: SystemTime) -> Option<Self> {
        // Calculate the total seconds and nanoseconds since UNIX_EPOCH
        let duration_since_epoch = match system_time.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration,
            Err(_) => return None, // The provided time is before UNIX_EPOCH
        };
        let total_seconds = duration_since_epoch.as_secs();
        let subsec_nanos = duration_since_epoch.subsec_nanos();

        // Calculate years, months, days, hours, minutes, and seconds from total_seconds
        let year = 1970 + (total_seconds / 31556926) as u32;
        let mut remaining_seconds = total_seconds % 31556926;
        let month = remaining_seconds / 2629743 + 1;
        remaining_seconds %= 2629743;
        let day = remaining_seconds / 86400 + 1;
        remaining_seconds %= 86400;
        let hour = remaining_seconds / 3600;
        remaining_seconds %= 3600;
        let min = remaining_seconds / 60;
        let sec = remaining_seconds % 60;

        // Format the components into a timecode string
        let timecode = format!(
            "{:04}{:02}{:02}{:02}{:02}{:02}{:06}",
            year, month, day, hour, min, sec, subsec_nanos
        );

        BVTime::new(timecode)
    }
}

/// Returns Option<None> if no timecode was found
pub fn parse_timecode(textcontent: &str) -> Option<BVTime> {

    let re = TIMECODE_REGEX.get_or_init(|| {
        Regex::new(r"Mk1=New Segment,,\d*,\d*,\d*,(\d*)").unwrap() 
      });
    
    let caps = re.captures(textcontent);
    if caps.is_some() {
        let caps = caps.unwrap();
        caps.get(1).map_or(Option::None, |m| BVTime::from_str(m.as_str()))
    } else {
        Option::None
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timecode() {
        let input = "Mk1=New Segment,,1,1,0,20200316125805099157";
        let output = parse_timecode(input).unwrap();
        let expected = BVTime::from_str("20200316125805099157").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_timecode_empty() {
        let input = "Mk1=New Segment,,1,1,0";
        let output = parse_timecode(input);
        let expected = Option::None;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_timecode_to_unix_time() {
        let timecode = BVTime::from_str("19700101000000000000").unwrap();
        let timestamp = timecode.to_system_time().unwrap();
        let expected: SystemTime = SystemTime::UNIX_EPOCH + std::time::Duration::new(0, 0);
        let cmpd = timestamp.cmp(&expected);
        assert_eq!(std::cmp::Ordering::Equal, cmpd, "PARSED TIME: {:?}, EXPECTED: {:?}", timestamp, expected);
    }

    #[test]
    fn test_convert_timecode_between_unix_time() {
        let timecode = BVTime::from_str("19990311140312003012").unwrap();
        let timestamp = timecode.to_system_time().unwrap();
        let expected = BVTime::from_system_time(timestamp).unwrap();
        assert_eq!(timecode, expected);
    }

}
use regex::Regex;
use std::sync::OnceLock;

/* _unit_dict = {
    "V": 1.0,  # V stands for Volt
    "µV": 1e-6,
    "uV": 1e-6,
    "mV": 1e-3,
    "nV": 1e-9,
    "C": 1,  # C stands for Celsius
    "°C": 1,  # degrees Celsius
    "n/a": 1,  # unit "not available" (or applicable)
    "µS": 1e-6,  # S stands for Siemens
    "uS": 1e-6,
    "ARU": 1,  # ARU is the unity for the breathing data
    "S": 1,
    "N": 1,
} */

#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
/// uV is µV
pub enum DataUnit {
    V,
    mV,
    #[default]
    uV, 
    nV,
    C, // Celsius
    ARU, // Unit for breathing data
    S, // Siemens
    uS, 
    N,
    NA,
}


#[derive(Default, Clone, Debug, PartialEq)]
pub struct ChannelInfo {
    pub header_id: String,
    pub label: String,
    pub reference: String,
    pub resolution: Option<f32>,
    pub unit: DataUnit,
}



static CHANINFO_REGEX: OnceLock<Regex> = OnceLock::new();

/// Returns empty Vec if no channelinfo present
pub fn parse_chan_info(textcontent: &str) -> Vec<ChannelInfo> {

    let re = CHANINFO_REGEX.get_or_init(|| {
        Regex::new(r"([Cch].\d.*)=([\w\.,]*)").unwrap() 
      });
    
    let mut results = vec![];

    // Iterate over matches and extract components
    for caps in re.captures_iter(textcontent) {
        let header_id = caps.get(1).map(|m| m.as_str());
        if header_id.is_some() { // serves as validation that we can safely unwrap everything in this line
            let header_id = header_id.unwrap().to_string();
            let second_capture = caps.get(2).map(|m| m.as_str()).unwrap();
            let split_values: Vec<&str> = second_capture.split(',').collect();

            if split_values.len() == 4 {
                let label: String = split_values[0].to_string();
                let reference: String = split_values[1].to_string();
                let _res = split_values[2].parse::<f32>();
                let resolution: Option<f32> = if _res.is_ok() {Some(_res.unwrap())} else {Option::None};
                let unit = split_values[3];
                let unit = match unit {
                    "V" => DataUnit::V,
                    "mV" => DataUnit::mV,
                    "uV"|"µV" => DataUnit::uV,
                    "nV" => DataUnit::nV,
                    "C"|"°C" => DataUnit::C,
                    "S" => DataUnit::S,
                    "uS"|"µS" => DataUnit::uS,
                    "ARU" => DataUnit::ARU,
                    "N" => DataUnit::N,
                    "n/a" => DataUnit::NA,
                    _ => DataUnit::NA,
                };

    
                let chan = ChannelInfo{
                    header_id,
                    label,
                    reference,
                    resolution,
                    unit
                };
                results.push(chan);
            }
        }    
    }
    results

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chan_info() {
        let input = "; Commas in channel names are coded as.
        Ch1=Fp1,,0.0488281,µV";
        let output = parse_chan_info(input);
        let expected = vec![ChannelInfo{
            header_id: "Ch1".to_string(),
            label: "Fp1".to_string(),
            reference: "".to_string(),
            resolution: Some(0.0488281),
            unit: DataUnit::uV,
        }];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_chan_info_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFile=01_data.eeg";
        let output = parse_chan_info(input);
        let expected = vec![];
        assert_eq!(output, expected);
    }

}
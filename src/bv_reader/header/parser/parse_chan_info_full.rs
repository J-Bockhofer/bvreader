use regex::Regex;
use std::sync::OnceLock;


#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilterSetting {
    #[default]
    OFF,
    ON(usize),
    DC,
    Unknown,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ChannelInfoFull {
    pub id: usize,
    pub label: String,
    pub phys_chan: usize,
    pub resolution: f32,
    pub unit: String,
    pub low_cutoff: FilterSetting,
    pub high_cutoff: FilterSetting,
    pub notch: FilterSetting,
    pub gradient: Option<f32>,
    pub offset: Option<usize>,
}


static CHANINFOFULL_REGEX: OnceLock<Regex> = OnceLock::new();
/// Returns empty Vec if no channelinfo present
pub fn parse_chan_info_full(textcontent: &str) -> Vec<ChannelInfoFull> {

    let re = CHANINFOFULL_REGEX.get_or_init(|| {
        Regex::new(r"(\d*)\W*(\w*)\W*(\d*)\W*(\d\.\d*)\W(\w{1,2})\W*(\w*)\W*(\w*)\W*(\w*)").unwrap() // this does not include options for gradient and offset!
      });
    
    let mut results = vec![];

    // Iterate over matches and extract components
    for caps in re.captures_iter(textcontent) {
        let id = caps.get(1).map(|m| m.as_str());
        if id.is_some() { // serves as validation that we can safely unwrap everything in this line
            let id = id.unwrap().parse::<usize>().unwrap();
            let label = caps.get(2).map(|m| m.as_str().to_string()).unwrap();
            let phys_chan = caps.get(3).map(|m| m.as_str().parse::<usize>().unwrap()).unwrap();
            let resolution = caps.get(4).map(|m| m.as_str().parse::<f32>().unwrap()).unwrap();
            let unit = caps.get(5).map(|m| m.as_str().to_string()).unwrap();
            let low_cutoff = caps.get(6).map(|m| parse_filter_setting(m.as_str())).unwrap();
            let high_cutoff = caps.get(7).map(|m| parse_filter_setting(m.as_str())).unwrap();
            let notch = caps.get(8).map(|m| parse_filter_setting(m.as_str())).unwrap();
            let gradient: Option<f32> = Option::None;
            let offset: Option<usize> = Option::None;

            let chan_info = ChannelInfoFull {
                id,
                label,
                phys_chan,
                resolution,
                unit,
                low_cutoff,
                high_cutoff,
                notch,
                gradient,
                offset,
            };
            results.push(chan_info);
        }
    }    
    results

}


static DIGIT_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn parse_filter_setting(input: &str) -> FilterSetting {

    let digit_re = DIGIT_REGEX.get_or_init(|| {
        Regex::new(r"(\d*)").unwrap() 
      });

    match input {
        "DC" => {FilterSetting::DC},
        "OFF"|"Off" => {FilterSetting::OFF},
        x => { 
            if digit_re.is_match(x) {
                let freq = x.parse::<usize>().expect(&format!("Error parsing FilterSetting! Input was = \" {} \" try into usize", x));
                FilterSetting::ON(freq)
            } else {
                FilterSetting::Unknown
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filter_setting_dc() {
        // DC              140              Off
        let input = "DC";
        let output = parse_filter_setting(input);
        let expected = FilterSetting::DC;
        assert_eq!(output, expected);
    }    

    #[test]
    fn test_parse_filter_setting_on() {
        // DC              140              Off
        let input = "140";
        let output = parse_filter_setting(input);
        let expected = FilterSetting::ON(140);
        assert_eq!(output, expected);
    } 


    #[test]
    fn test_parse_chan_info_full() {
        let input = "
        Channels
        --------
        #     Name      Phys. Chn.    Resolution / Unit   Low Cutoff [s]   High Cutoff [Hz]   Notch [Hz]    Gradient         Offset
        1     Fp1         1          0.0488281 µV             DC              140              Off
        2     Fz          2          0.0488281 µV             DC              140              Off
        3     F3          3          0.0488281 µV             DC              140              Off";
        let output = parse_chan_info_full(input);
        let expected = ChannelInfoFull{
            id: 1,
            label: "Fp1".to_string(),
            phys_chan: 1,
            resolution: 0.0488281,
            unit: "µV".to_string(),
            low_cutoff: FilterSetting::DC,
            high_cutoff: FilterSetting::ON(140),
            notch: FilterSetting::OFF,
            gradient: Option::None,
            offset: Option::None,
        };
        assert_eq!(output[0], expected);
    }

    #[test]
    fn test_parse_chan_info_full_empty() {
        let input = "[Common Infos]
        Codepage=
        DataFile=01_data.eeg";
        let output = parse_chan_info_full(input);
        let expected = vec![];
        assert_eq!(output, expected);
    }

}
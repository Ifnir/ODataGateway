use std::collections::{HashMap};


pub fn convert_to_odata(base_url: &str, parameters: &HashMap<&str, &str>) -> String {
    let mut odata_url = String::from(base_url);
    odata_url.push_str("?");

    for (key, value) in parameters {
        let query_param = match key {
            &"filter" => {
                format!("$filter={}", value)
            },
            _ => format!("{}={}", key, value),
        };
        odata_url.push_str(&query_param);
    }

    return odata_url
}

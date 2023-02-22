use std::collections::{HashMap};



fn format_url(url: String) -> HashMap<String, String> {
    let mut query_params = HashMap::new();
    if let Some(query) = url.split('?').nth(1) {
        for pair in query.split('&') {
            let mut key_value = pair.splitn(2,'=');
            let key = key_value.next().unwrap_or("").to_string();
            let value = key_value.next().unwrap_or("").to_string();

            match key.as_str() {
                "filter" => {
                    query_params.insert("filter".to_string(), value);
                }
                "expand" => {
                    query_params.insert("expand".to_string(), value);
                }
                _ => {}
            }
        }
    }
    return query_params;
}

pub fn convert_to_odata(base_url: &str) -> HashMap<String, String> {
    let mut odata_url = String::from(base_url);
    odata_url.push_str("?");

    let formatted_url = format_url(odata_url);


    return formatted_url;
}

/* use std::collections::HashMap;

pub fn convert_to_odata(base_url: &str, parameters: &HashMap<&str, &str>) -> String {
    let mut odata_url = String::from(base_url);
    odata_url.push_str("?");
    for (key, value) in parameters {
        let query_param = match key {
            &"filter" => {
                let mapped_filter = map_comma_separated_param(value, Some("and"));
                format!("$filter={}", mapped_filter)
            },
            &"select" => {
                let mapped_select = map_comma_separated_param(value, Some(","));
                format!("$select={}", mapped_select)
            },
            &"expand" => {
                let mapped_expand = map_comma_separated_param(value, Some(","));
                format!("$expand={}", mapped_expand)
            },
            &"orderby" => format!("$orderby={}", value),
            &"skip" => format!("$skip={}", value),
            &"top" => format!("$top={}", value),
            _ => format!("{}={}", key, value),
        };
        odata_url.push_str(&query_param);
        odata_url.push('&');
    }
    // Remove the last '&' character
    odata_url.pop();
    odata_url
}

fn map_operator(operator: &str) -> Option<&str> {
    match operator {
        "=" => Some("eq"),
        "<" => Some("lt"),
        ">" => Some("gt"),
        "<=" => Some("le"),
        ">=" => Some("ge"),
        "!<" => Some("not lt"),
        "!>" => Some("not gt"),
        "<>" => Some("ne"),
        "!=" => Some("ne"),
        _ => None,
    }
}

fn map_comma_separated_param(key: &str, values: Option<&str>) -> String {
    if let Some(values) = values {
        let items: Vec<String> = values
            .split(',')
            .map(|s| s.to_string()) // convert each &str to a String
            .collect();
       
        format!("{}={}", key, items.join(","))
    } else {
        String::new()
    }
}
 */
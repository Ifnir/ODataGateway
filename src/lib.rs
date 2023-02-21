mod query_builder;

#[cfg(test)]
mod tests {
    // use std::collections::HashMap;

    use crate::query_builder;

   /*  #[test]
    fn test_convert_to_odata() {
        let base_url = "https://myapi.com";
        let mut parameters = HashMap::new();
        parameters.insert("filter", "name != 'John' and age >= 30");
        parameters.insert("select", "name,age,created_at");
        parameters.insert("expand", "products,orders");
        parameters.insert("orderby", "name desc");
        parameters.insert("skip", "10");
        parameters.insert("top", "50");
        let odata_url = converter::convert_to_odata(base_url, &parameters);
        assert_eq!(
            odata_url,
            "https://myapi.com?$filter=name%20ne%20'John'%20and%20age%20ge%2030&$select=name,age,created_at&$expand=products,orders&$orderby=name%20desc&$skip=10&$top=50"
        );
    } */

    #[test]
    fn test_query_builder() {
        let base_url = "https://myapi.com?filter=name=Johannes Slet&expand=*";
        let url = query_builder::convert_to_odata(base_url);
        assert_eq!(url, "https://myapi.com?");
    }
}

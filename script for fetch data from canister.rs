// fetch canister data
// input is canister id as string
// output is trait object array Vec<std::collections::HashMap<String, String>> and trait value array Vec<String>
pub async fn fetch_canister_data(input: String) -> (Vec<std::collections::HashMap<String, String>> , Vec<String>) {
    let mut trait_object_array: Vec<std::collections::HashMap<String, String>> = Vec::new();
    let mut trait_array: Vec<String> = Vec::new();
    
    let result: ic_cdk::api::call::CallResult<(String,())> = ic_cdk::api::call::call(ic_cdk::export::Principal::from_text(input).unwrap(), "getTokens", ()).await;
    
    if Ok(result) {
        let pretty_string = serde_json::to_string_pretty(&result).expect("can't convert string");

        let string_data = pretty_string.replace("\\\\22", r#"""#);
        let mut data = string_data.as_str();

        let mut index = data.find("record { 0 :").unwrap_or(0);
        (_, data) = data.split_at(index);
        let mut pointer = 1;
        while 1 == 1 {
            if pointer < 1000 {
                index = data
                    .find(format!("record {{ {} :", pointer).as_str())
                    .unwrap_or(0);
            } else {
                let mut str = pointer.to_string();
                str.insert_str(1, "_");
                index = data
                    .find(format!("record {{ {} :", str).as_str())
                    .unwrap_or(0);
            }
            let mut record;
            if index == 0 {
                record = data;
            } else {
                (record, data) = data.split_at(index);
            }
            let start = record.find("[").unwrap_or(0);
            (_, record) = record.split_at(start);
            let end = record.find("]").unwrap_or(0);
            if start == 0 || end == 1 {
                if index == 0 {
                    break;
                } else {
                    pointer += 1;
                    continue;
                }
            }
            let res;
            (res, _) = record.split_at(end + 1);
            let json_array: Vec<TraitType> = serde_json::from_str(res).unwrap();
            let mut my_object = std::collections::HashMap::new();
            for item in json_array {
                let key = item.trait_type;
                if !trait_array.contains(&key.clone()) {
                    trait_array.push(key.clone());
                }
                my_object.insert(key.clone(), item.value);
            }
            trait_object_array.push(my_object);
            if index == 0 {
                break;
            }
            pointer += 1;
        }
    } 
    (trait_object_array, trait_array)
}

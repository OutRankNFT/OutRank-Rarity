#[derive(Debug, Deserialize)]
struct TraitType {
    trait_type: String,
    value: String,
}

// convert canister data to traits array. input : canister_data and traits_list, output : traits_value
// canister_data type is HashMap<String, String>> (json object array)
// traits_list type is Vec<String> (string array)
// traits_value type is Vec<Vec<String>> (two dimentional array of string), row count is nft_count and column count is trait_count
fn canister_data_to_traits_value(canister_data: Vec<std::collections::HashMap<String, String>> ,trait_list: Vec<String>) -> std::vec::Vec<std::vec::Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    for json in canister_data {
        let mut sub_result:Vec<String> = Vec::new();
        for nft_trait in trait_list.clone() {
            let temp_trait = nft_trait.clone();
            if json.contains_key(&temp_trait) {
                let tmp_json = json.clone();
                let tmp_trait = temp_trait.clone();
                let trait_value = tmp_json.get(&tmp_trait);
                // .unwrap(); 
                let default_value = "NA".to_string();
                sub_result.push(trait_value.unwrap_or(&default_value).to_string());
            }
            else {
                sub_result.push("NA".to_string());
            }
        }
        result.push(sub_result);
    }
    result
}

// convert traits_value as column count is nft_count and row count is trait_count
fn reverse_mat(input: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();
    let row_len = input[0].len();
    let mut index = 0;
    while index < row_len {
        let col: Vec<String> = Vec::new();
        output.push(col);
        index += 1;
    }
    for row in input {
        for (index, value) in row.iter().enumerate() {
            let mut temp_col:Vec<String> = output[index].clone();
            temp_col.push(value.to_string());
            output[index] = temp_col;
        }
    }
    output
}

// get count of trait_value and frequency of trait_value
// Input is trait_value Vec<Vec<String>> (two dimentional array of string)
// outputs are trait_count and trait_frequency Vec<Vec<f64>> (two dimentional array of float) 
fn get_traits_count_freq_number(input: Vec<Vec<String>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let mut traits_count: Vec<Vec<f64>> = Vec::new();
    let mut traits_freq: Vec<Vec<f64>> = Vec::new();
    let no_tokens:i32 = input[0].len() as i32;
    for col in input {
        let mut count_col: Vec<f64> = Vec::new();
        let mut freq_col: Vec<f64> = Vec::new();
        let temp_col = col.clone();
        for value in temp_col {
            let tmp_col = col.clone();
            let count:f64 = tmp_col.iter().filter(|&v| *v == value).count() as f64;
            count_col.push(count.clone());
            freq_col.push(count.clone() as f64/no_tokens as f64);
        }
        traits_count.push(count_col);
        traits_freq.push(freq_col);
    }
    (traits_count, traits_freq)
}

// calculate rarity_matrix
// input is trait_freq Vec<Vec<f64>> (two dimentional array of float)
// output is rarity_matrix Vec<Vec<f64>> (two dimentional array of float)
fn rare_calc(input: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut output: Vec<Vec<f64>> = Vec::new();
    let mut min_col: Vec<f64> = Vec::new();
    let mut max_col: Vec<f64> = Vec::new();
    let mut arithmetic_col: Vec<f64> = Vec::new();
    let mut harmonic_col: Vec<f64> = Vec::new();
    let mut geometric_col: Vec<f64> = Vec::new();

    for (index_row, _) in input[0].iter().enumerate() {
        let mut param_array: Vec<f64> = Vec::new();
        for (index_col, _) in input.iter().enumerate() {
            param_array.push(input[index_col][index_row]);
        }
        min_col.push(*param_array.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
        max_col.push(*param_array.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
        arithmetic_col.push(wpmean(param_array.clone(), 1));
        harmonic_col.push(wpmean(param_array.clone(), -1));
        geometric_col.push(wpmean(param_array.clone(), 0));
    }
    output.push(min_col);
    output.push(max_col);
    output.push(geometric_col);
    output.push(harmonic_col);
    output.push(arithmetic_col);
    output
}

// calculate rarity_scroe
// input is rarity_matric Vec<Vec<f64>> (two dimentional array of float)
// output is rarity_score Vec<Vec<f64>> (two dimentional array of float)
fn score_calc(input: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut output: Vec<Vec<f64>> = Vec::new();
    for col in input {
        let min = col.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = col.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let mut new_col: Vec<f64> = Vec::new();
        let temp_col = col.clone();
        for value in temp_col {
            if min == max {
                new_col.push(0.0 as f64);
            }
            else {
                new_col.push(( value - min ) / ( max - min ));
            }
        }
        output.push(new_col);
    }
    output
}

// calculate p weighted mean of vector 
fn wpmean(input: Vec<f64>, p: i32) -> f64 {
    let output: f64;
    let lenth: f64 = input.clone().len() as f64;
    if p==0 {
        let mut multi: f64 = 1.0 as f64;
        for val in input {
            multi *= val;
        }
        output = f64::powf(multi, 1.0 as f64/lenth);
    }
    else {
        let mut mean: f64 = 0.0 as f64;
        for val in input {
            mean += f64::powf(val, p as f64);
        }
        mean /= lenth as f64;
        output = f64::powf(mean, 1.0 as f64 / p as f64);
    }
    output
}

// calculate rarity_rank
// input is rarity_score Vec<Vec<f64>> (two dimentional array of float)
// output is rarity_rank Vec<Vec<f64>> (two dimentional array of float)
fn rare_rank(input: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut output: Vec<Vec<f64>> = Vec::new();
    for col in input {
        let mut ranks: Vec<f64> = col.clone();
        ranks.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut rank_values: Vec<f64> = Vec::new();

        for value in col {
            let rank = ranks.iter().position(|&x| x == value).unwrap() as f64;
            rank_values.push(rank);
        }

        output.push(rank_values);
    }
    output
}

// add max-min field to rarity_score
// input is rarity_scrore Vec<Vec<f64>> (two dimentional array of float)
// output is updated rarity_score Vec<Vec<f64>> (two dimentional array of float)
fn add_max_min_minus_to_rarity_score(input: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut output: Vec<Vec<f64>> = Vec::new();
    for col in input.clone() {
        output.push(col.clone());
    }
    let mut new_col: Vec<f64> = Vec::new();
    let temp_input:Vec<Vec<f64>> = input.clone(); 
    for (index, _) in temp_input[0].iter().enumerate() {
        new_col.push(input[1][index] - input[0][index]);
    }
    output.push(new_col);
    output
}

// calculate trait_independence
// input is trait_freq Vec<Vec<f64>>  (two dimentional array of float)
// output is trait_independnece Vec<Vec<f64>>  (two dimentional array of float)
fn trait_independence(input: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut output: Vec<Vec<f64>> = vec![vec![0.0; input.len()]; input.len()];
    for out_val in 0 ..=input.len() -2 {
        for in_val in out_val+1 ..=input.len() -1 {
            let key_set= independent_test(input[out_val].clone(), input[in_val].clone());
            let (chi2, dof) = calculate_chi2_dof(key_set);
            let chi_squared = ChiSquared::new(dof as f64).unwrap();
            let critical_value = chi_squared.inverse_cdf(0.95);
            output[in_val][out_val] = format!("{:.1$}", chi2, 4).parse::<f64>().unwrap();
            output[out_val][in_val] = format!("{:.1$}", critical_value, 4).parse::<f64>().unwrap();
        }
    }
    output
}

// calculate trait_cramersV
// input is trait_freq Vec<Vec<f64>>  (two dimentional array of float)
// output is trait_cramersV Vec<Vec<f64>>  (two dimentional array of float)
fn trait_cramers_v(input: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut output: Vec<Vec<f64>> = vec![vec![0.0; input.len()]; input.len()];
    for out_val in 0 ..=input.len() -2 {
        for in_val in out_val+1 ..=input.len() -1 {
            let key_set= independent_test(input[out_val].clone(), input[in_val].clone());
            let (chi2, _) = calculate_chi2_dof(key_set.clone());
            let mut sum = 0.0;
            for col in key_set.clone() {
                for item in col {
                    sum += item;
                }
            }
            let dimension: f64; 
            if key_set.len() > key_set[0].len() {
                dimension = key_set[0].len() as f64 - 1.0;
            }
            else{
                dimension = key_set.len() as f64 - 1.0;
            }
            let sqrt = ((chi2/sum)/dimension).sqrt();
            output[in_val][out_val] = format!("{:.1$}", sqrt, 4).parse::<f64>().unwrap();
        }
    }
    output
}

// calculate correlation matrix two array
// inputs are two array Vec<Vec<f64>>
// output is correlation matrix Vec<Vec<f64>>
fn independent_test(first: Vec<f64>, second: Vec<f64>) -> Vec<Vec<f64>> {
    let first_unique_array: Vec<f64> = get_unique_array(first.clone());
    let second_unique_array: Vec<f64> = get_unique_array(second.clone());
    let mut key_set:Vec<Vec<f64>> = vec![vec![0.0; first_unique_array.len()]; second_unique_array.len()];
    for index in 0..first.len() {
        if let Some(col)= second_unique_array.iter().position(|&x| x == second[index]) {
            if let Some(row)= first_unique_array.iter().position(|&x| x == first[index]) {
                key_set[col][row] += 1.0;
            }
        }
    }
    key_set
}

// calculate chi2 statistic and degree of freedom
fn calculate_chi2_dof(input: Vec<Vec<f64>>) -> (f64, usize) {
    let row_totals: Vec<f64> = input.iter().map(|row| row.iter().sum()).collect();
    let column_totals: Vec<f64> = (0..input[0].len())
        .map(|col| input.iter().map(|row| row[col]).sum())
        .collect();
    let grand_total: f64 = row_totals.iter().sum();
    let expected: Vec<Vec<f64>> = input
        .iter()
        .enumerate()
        .map(|(col_index, col)| {
            col.iter()
                .enumerate()
                .map(|(row, _)| (row_totals[col_index] * column_totals[row]) / grand_total)
                .collect()
        })
        .collect();
    let chi2_statistic = chi2_contingency(&input, &expected);
    let dof: usize = (input.len() - 1) * (input[0].len() - 1);
    (chi2_statistic, dof)
}

// calculate cai2_statistic
fn chi2_contingency(input: &[Vec<f64>], expected: &[Vec<f64>]) -> f64 {
    let mut chi2_statistic = 0.0;
    for (i, row) in input.iter().enumerate() {
        for (j, &input_value) in row.iter().enumerate() {
            let expected_value = expected[i][j] as f64;
            chi2_statistic += (input_value as f64 - expected_value).powi(2) / expected_value;
        }
    }
    chi2_statistic
}

// calculate sorted unique array of input array
fn get_unique_array(input: Vec<f64>) -> Vec<f64> {
    let mut output: Vec<f64> = Vec::new();
    for val in input {
        if !output.contains(&val) {
            output.push(val);
        }
    }
    output.sort_by(|a, b| a.partial_cmp(b).unwrap());
    output
} 

// calculate trait_normalize
// inputs are traits_value Vec<Vec<f64>>, traits_count Vec<Vec<f64>>, traits_freq Vec<Vec<f64>> 
// output is trait_normalize Vec<Vec<f64>> 
fn trait_normalize(traits_value: Vec<Vec<String>>, traits_count: Vec<Vec<f64>>, traits_freq: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut w: Vec<i32> = Vec::new();
    for col in traits_value.clone() {
        let mut value_list: Vec<String> = Vec::new();
        for val in col {
            if !value_list.contains(&val) {
                value_list.push(val);
            }
        }
        w.push(value_list.len() as i32);
    } 
    let mut output: Vec<Vec<f64>> = Vec::new();
    let style_list: Vec<String> = vec!["geometric".to_string(), "harmonic".to_string(), "arithmetic".to_string()];
    let counts_control_list: Vec<bool> = vec![true, false];
    for style in style_list {
        for counts_control in counts_control_list.clone() {
            let counts;
            if counts_control == false {
                counts = traits_count.clone();
            }
            else {
                counts = traits_freq.clone();
            }
            let temp_nor:Vec<f64> = normalize_calc(w.clone(),counts,style.clone(),counts_control.clone());
            let max = temp_nor.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let min = temp_nor.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let mut result: Vec<f64> = Vec::new(); 
            for (_, item) in temp_nor.iter().enumerate() {
                result.push(( item - min ) / (max - min));
            }
            output.push(result);
        }
    }
    output
}

// calculate nomaize
// inputs are weights Vec<i32>, counts Vec<Vec<f64>>, style String, control bool
// output is normalize array Vec<f64>
fn normalize_calc(w: Vec<i32>, counts: Vec<Vec<f64>>, style: String, counts_control: bool) -> Vec<f64> {
    let mut weights: Vec<Vec<f64>> = Vec::new();
    let mut weight_sum: f64 = 0.0;
    if style == "geometric" && counts_control == true {
        for element in w {
            let temp = element as f64;
            weights.push(vec![temp.recip(); counts[0].len()]);
            weight_sum += temp.recip();
        }
    }
    else {
        for element in w {
            weights.push(vec![element as f64; counts[0].len()]);
            weight_sum += element as f64;
        }
    }

    let mut normalized_rarity: Vec<f64> = Vec::new();

    if style == "geometric" {
        if counts_control == false {
            for row_index in 0..counts[0].len() {
                let mut sum = 0.0;
                for col_index in 0..counts.len() {
                    sum += counts[col_index][row_index].ln()*weights[col_index][row_index];
                }
                sum = sum.powf(1.0/weight_sum);
                normalized_rarity.push(f64::exp(sum)/counts.len() as f64);
            }
        }
        else{
            for row_index in 0..counts[0].len() {
                let mut sum = 1.0;
                for col_index in 0..counts.len() {
                    sum *= counts[col_index][row_index].powf(weights[col_index][row_index]);
                }
                sum = sum.powf(1.0/weight_sum);
                normalized_rarity.push(sum);
            }
        }
    }
    else if style == "harmonic" {
        for row_index in 0..counts[0].len() {
            let mut sum = 0.0;
            for col_index in 0..counts.len() {
                sum += weights[col_index][row_index] / counts[col_index][row_index];
            }
            sum = sum/weight_sum;
            normalized_rarity.push(sum.powf(-1.0)*weight_sum);
        }
    }
    else {
        for row_index in 0..counts[0].len() {
            let mut sum = 0.0;
            for col_index in 0..counts.len() {
                sum += counts[col_index][row_index] * weights[col_index][row_index];
            }
            sum = sum/weight_sum;
            normalized_rarity.push(sum);
        }
    }

    normalized_rarity
}
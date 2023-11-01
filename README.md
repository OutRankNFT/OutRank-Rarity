# OutRank-Rarity
we've implemented mathematical code to calculate rarity of NFT collections in both of Python and Rust
- rarity_math_code_python.ipynb is for Python code and 
- rarity_math_code_rust.rs is for Rust code.
- script_for_fetch_data_from_canister.rs is for Rust code to fetch NFT collections trait data(we'll call this "canister data") by inter-canister call.
Here is a breif explanation for rarity_math_code_rust.rs.

## Basic Usage for Rust code
- fetch canister data(nft collections trait data) as an Object array.
  - (trait_object_array, trait_array) = fetch_canister_data(canister_id);
    - trait_object_array example : [{"skin (texture)": "Dark", "Gender": "Male", "Move": "Breakdance Uprock", "Background": "Blue", "Cloths": "Casual Shirt/Pants"}, ... ... ...]
  - trait_array  is array of collections trait properties.
    - trait_array example : ["Move", "skin (texture)", "Background", "Cloths", "Gender", "Asssecrioes"]
      
- calculate traits_value from canister_data
  - traits_value = canister_data_to_traits_value(trait_object_array,trait_array);
    
- calculate traits_count and traits_freq from reversed traits_value
  - (traits_count, traits_freq) = get_traits_count_freq_number(reverse_mat(traits_value));
    
- calculate rarity_mat from traits_freq
  - rarity_mat = rare_calc(traits_freq);
    
- calculate rarity_score from rarity_mat
  - mut rarity_score = score_calc(rarity_mat);
    
- calculate rarity_rank from rarity_score
  - rarity_rank = rare_rank(rarity_score);
    
- add max-min field to rarity_score
  - rarity_score = add_max_min_minus_to_rarity_score(rarity_score);
    
- calculate trait_independence from traits_freq
  - trait_independence = trait_independence(traits_freq);
    
- calculate trait_cramers_v from traits_freq
  - trait_cramers_v = trait_cramers_v(traits_freq);
    
- calculate trait_normalize from traits_value, traits_count, traits_freq
  - trait_normalize = trait_normalize(reverse_mat(traits_value), traits_count, traits_freq);
    

## Now you can use these data.

- rarity_rank: rarity_rank
- rarity_score: rarity_score
- trait_independence: trait_independence
- trait_cramers_v: trait_cramers_v
- trait_normalize: trait_normalize
- trait_array: trait_array

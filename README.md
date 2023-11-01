# OutRank-Rarity
We've implemented mathematical code to calculate rarity of NFT collections in both of Python and Rust
- Rarity_math_code_python.ipynb is for Python code and 
- Rarity_math_code_rust.rs is for Rust code.
- Script_for_fetch_data_from_canister.rs is for Rust code to fetch NFT collections trait data(we'll call this "canister data") by inter-canister call.
Here is a breif explanation for rarity_math_code_rust.rs.

## Basic Usage for Rust code
- Fetch canister data(nft collections trait data) as an Object array.
  - (trait_object_array, trait_array) = fetch_canister_data(canister_id);
    - trait_object_array example : [{"skin (texture)": "Dark", "Gender": "Male", "Move": "Breakdance Uprock", "Background": "Blue", "Cloths": "Casual Shirt/Pants"}, ... ... ...]
  - trait_array  is array of collections trait properties.
    - trait_array example : ["Move", "skin (texture)", "Background", "Cloths", "Gender", "Asssecrioes"]
      
- Calculate traits_value from canister_data. Canister_data is an Object Array and convert it as Two-Dimensional Array. Row Index is NFT id. Column Index is same as trait_array.
  - traits_value = canister_data_to_traits_value(trait_object_array,trait_array);
    - traits_value example : [ [ "Breakdance Uprock", "Dark", "Blue", "Casual Shirt/ Pants", "Male", "NA" ], [ "Salsa (long)", "Light", "Yellow", "Jump Suit", "Male", "NA" ], ... ... ...]
    
- Calculate traits_count and traits_freq from reversed traits_value. Reversed traits_value is also Two-Dimensional array. But Row Index is same as trait_array and Column Index is same as NFT id. traits_count represent count of same value in row(Each row is trait property) and traits_freq is same as NFTs count devided traits_count.
  - (traits_count, traits_freq) = get_traits_count_freq_number(reverse_mat(traits_value));
    - traits_count example : [ [2 ,2 ,1, 8, 8, 8, 2, 6, ...], [8, 12, 12, 12, 12, 12, 8, ...], ... ... ]
    - traits_freq example : [ [0.0391, 0.03921, 0.01961, 0.15686, 0.15686, 0.15686, ... ], ... ]
- calculate rarity_mat from traits_freq. rarity_mat has 5 rows.
  - First row is array of min value of column.
  - Second row is array of max value of column.
  - Third row is array of arithmetic value of column.
  - Fourth row is array of harmonic value of column.
  - Fifth row is array of geometric value of column.
  - rarity_mat = rare_calc(traits_freq);
    
- Calculate rarity_score from rarity_mat. rarity_score is Two-Dementional array that contains normalized value between 0 and 1 of rarity_mat.
  - rarity_score = score_calc(rarity_mat);
    
- Calculate rarity_rank from rarity_score. rarity_rank is Two-Dementional array contains rows sorted by value from rarity_score.
  - rarity_rank = rare_rank(rarity_score);
    
- These two methods calculate trait_independence and trait_cramers_v from traits_freq. By calcuating Chi-Two-squared distribution.
  - trait_independence = trait_independence(traits_freq);
  - trait_cramers_v = trait_cramers_v(traits_freq);
    
- Calculate trait_normalize from traits_value, traits_count, traits_freq.Trait_normalize means trait normalised rarity score
  - trait_normalize = trait_normalize(reverse_mat(traits_value), traits_count, traits_freq);

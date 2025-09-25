use crate::{
    client::{KeyIndexArray, VaultConf, generate_standard_vault_sets},
    same_items,
};

pub fn get_vault_conf(sets: Vec<KeyIndexArray>) -> VaultConf {
    let sets: Vec<Vec<u64>> = sets.iter().map(|keys| keys.clone()).collect();

    let paths_length: Vec<usize> = sets.iter().map(|keys| keys.len()).collect();

    let min_required_keys = paths_length.iter().min().unwrap_or(&0);
    let max_required_keys = paths_length.iter().max().unwrap_or(&0);

    if min_required_keys != max_required_keys {
        return VaultConf::Custom;
    }

    let required_keys = *min_required_keys as u64;

    let total_keys = sets
        .iter()
        .flat_map(|keys| keys.iter())
        .max()
        .map(|max_index| max_index + 1)
        .unwrap_or(0);

    let standard_sets: Vec<Vec<u64>> = generate_standard_vault_sets(total_keys, required_keys)
        .iter()
        .map(|set| set.clone())
        .collect();

    let is_standard = same_items(&standard_sets, &sets, |a, b| {
        same_items(a, b, |a, b| a == b)
    });

    if !is_standard {
        return VaultConf::Custom;
    }

    VaultConf::Standard {
        total: total_keys,
        required: required_keys,
    }
}

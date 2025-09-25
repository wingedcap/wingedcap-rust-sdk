use crate::client::KeyIndexArray;

pub fn generate_standard_vault_sets(total: u64, required: u64) -> Vec<KeyIndexArray> {
    if required == 0 || required == total {
        let set = (0..required).collect();
        return vec![set];
    }

    let without_last = generate_standard_vault_sets(total - 1, required);

    let with_last: Vec<KeyIndexArray> = generate_standard_vault_sets(total - 1, required - 1)
        .into_iter()
        .map(|mut set| {
            set.push(total - 1);
            set
        })
        .collect();

    let mut result = without_last;

    result.extend(with_last);

    result
}

#[derive(PartialEq, Clone)]
pub enum VaultConf {
    Standard { total: u64, required: u64 },
    Custom,
}

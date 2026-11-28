use super::contract_role::ContractRole;

pub struct FsharpClassType {
    pub role: ContractRole,
    pub marker: String,
    pub name: String,
    pub header: usize,
    pub body: Vec<usize>,
}

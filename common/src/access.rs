/// The roles for interacting the contract
pub enum Role {
    /// Name setter role
    NameSetter,
    /// Unlimited reader role
    UnlimitedReaderRole,
}

/// The access control registry interface in the solidity contract
pub trait AccessControlRegistry {
    /// Checks if user has a particular role
    /// `role` The role to check
    /// `who` The address to check
    fn has_role(&self, role: Role, who: &[u8]) -> bool;
}

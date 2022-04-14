use crate::Bytes32;
use crate::Empty;

/// Trait that implements temporary and permanent whitelists for
/// multiple services identified with a hash
///
/// This trait implements two kinds of whitelisting:
///   (1) Temporary, ends when the expiration timestamp is in the past
///   (2) Indefinite, ends when the indefinite whitelist count is zero
/// Multiple senders can indefinitely whitelist/unwhitelist independently. The
/// user will be considered whitelisted as long as there is at least one active
/// indefinite whitelisting.
///
/// The interface of this contract is not implemented. It should be
/// inherited and its functions should be exposed with a sort of an
/// authorization scheme.
pub trait Whitelist {
    /// The address type for the chain
    type Address: AsRef<[u8]> + Empty;
    type U256;

    /// Returns if the user is whitelised to use the service
    /// `service_id` Service ID
    /// `user` User address
    fn user_is_whitelisted(&self, service_id: &Bytes32, user: &Self::Address) -> bool;

    /// @notice Extends the expiration of the temporary whitelist of the user
    /// for the service
    /// @param serviceId Service ID
    /// @param user User address
    /// @param expirationTimestamp Timestamp at which the temporary whitelist
    /// will expire
    fn extend_whitelist_expiration(
        &mut self,
        service_id: &Bytes32,
        user: &Self::Address,
        expiration_timestamp: u64,
    );

    /// @notice Sets the expiration of the temporary whitelist of `user` to be
    /// able to use the service with `serviceId` if the sender has the
    /// whitelist expiration setter role
    /// @param serviceId Service ID
    /// @param user User address
    /// @param expirationTimestamp Timestamp at which the temporary whitelist
    /// will expire
    fn set_whitelist_expiration(
        &mut self,
        service_id: &Bytes32,
        user: &Self::Address,
        expiration_timestamp: u64,
    );

    /// @notice Sets the indefinite whitelist status of `user` to be able to
    /// use the service with `serviceId` if the sender has the indefinite
    /// whitelister role
    /// @param serviceId Service ID
    /// @param user User address
    /// @param status Indefinite whitelist status
    fn set_indefinite_whitelist_status(
        &mut self,
        service_id: &Bytes32,
        user: &Self::Address,
        status: bool,
    ) -> Self::U256;

    /// @notice Revokes the indefinite whitelist status granted to the user for
    /// the service by a specific account
    /// @param serviceId Service ID
    /// @param user User address
    /// @param setter Setter of the indefinite whitelist status
    fn revoke_indefinite_whitelist_status(
        &mut self,
        service_id: &Bytes32,
        user: &Self::Address,
        setter: &Self::Address,
    ) -> (bool, Self::U256);
}
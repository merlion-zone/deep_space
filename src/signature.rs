use crate::CosmosPublicKey;
#[cfg(feature = "ethermint")]
use crate::EthermintPublicKey;

/// Signed data that contains both the signature, and the public key
/// used to sign it.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Signature {
    pub signature: Vec<u8>,
    pub pub_key: PubKey,
}

/// The types of public key which deep_space supports, used in signature
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PubKey {
    Cosmos(CosmosPublicKey),
    #[cfg(feature = "ethermint")]
    Ethermint(EthermintPublicKey),
}

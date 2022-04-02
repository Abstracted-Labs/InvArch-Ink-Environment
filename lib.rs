#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::vec::Vec;
use invarch_runtime_primitives::{AnyId, CommonId};

#[ink::chain_extension]
pub trait InvarchExtension {
    type ErrorCode = ExtensionError;

    #[ink(extension = 5000, returns_result = false)]
    fn ipf_mint(metadata: Vec<u8>, data: ink_env::Hash) -> CommonId;

    #[ink(extension = 5001, returns_result = false)]
    fn ipf_burn(ipf_id: CommonId);

    #[ink(extension = 5100, returns_result = false)]
    fn ips_create(metadata: Vec<u8>, data: Vec<CommonId>, allow_replica: bool) -> CommonId;

    #[ink(extension = 5102, returns_result = false)]
    fn ips_append(
        ips_id: CommonId,
        assets: Vec<AnyId<CommonId, CommonId>>,
        new_metadata: Option<Vec<u8>>,
    );
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ExtensionError {
    FailGetRandomSource,
    MaxMetadataExceeded,
}

impl ink_env::chain_extension::FromStatusCode for ExtensionError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetRandomSource),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum InvarchEnvironment {}

impl Environment for InvarchEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
    type ChainExtension = InvarchExtension;
}

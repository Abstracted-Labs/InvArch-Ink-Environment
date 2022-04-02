#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(env = InvarchEnvironment)]
mod example {
    use ink_prelude::vec;
    use ink_prelude::vec::Vec;
    use invarch_ink_environment::{ExtensionError, InvarchEnvironment};
    use invarch_runtime_primitives::{AnyId, CommonId};

    #[ink(storage)]
    pub struct Extension {
        ipf: Vec<CommonId>,
        ips: Vec<CommonId>,
    }

    #[ink(event)]
    pub struct MintedNewIpf {
        #[ink(topic)]
        new: CommonId,
    }

    #[ink(event)]
    pub struct CreatedNewIps {
        #[ink(topic)]
        new: CommonId,
    }

    #[ink(event)]
    pub struct BurnedIpf {
        #[ink(topic)]
        ipf_id: CommonId,
    }

    #[ink(event)]
    pub struct AppendedToIps {
        #[ink(topic)]
        ips_id: CommonId,
    }

    impl Extension {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                ipf: vec![],
                ips: vec![],
            }
        }

        #[ink(message)]
        pub fn mint_ipf(
            &mut self,
            metadata: Vec<u8>,
            data: ink_env::Hash,
        ) -> Result<(), ExtensionError> {
            let id = self.env().extension().ipf_mint(metadata, data)?;

            self.ipf.push(id);
            self.env().emit_event(MintedNewIpf { new: id });
            Ok(())
        }

        #[ink(message)]
        pub fn burn_ipf(&mut self, ipf_id: CommonId) -> Result<(), ExtensionError> {
            self.env().extension().ipf_burn(ipf_id)?;

            self.ipf = self
                .ipf
                .clone()
                .into_iter()
                .filter(|id| *id != ipf_id)
                .collect();

            self.env().emit_event(BurnedIpf { ipf_id });
            Ok(())
        }

        #[ink(message)]
        pub fn create_ips(
            &mut self,
            metadata: Vec<u8>,
            data: Vec<CommonId>,
            allow_replica: bool,
        ) -> Result<(), ExtensionError> {
            let id = self
                .env()
                .extension()
                .ips_create(metadata, data, allow_replica)?;

            self.ips.push(id);
            self.env().emit_event(CreatedNewIps { new: id });
            Ok(())
        }

        #[ink(message)]
        pub fn append_to_ips(
            &mut self,
            ips_id: CommonId,
            assets: Vec<AnyId<CommonId, CommonId>>,
            new_metadata: Option<Vec<u8>>,
        ) -> Result<(), ExtensionError> {
            self.env()
                .extension()
                .ips_append(ips_id, assets, new_metadata)?;

            self.env().emit_event(AppendedToIps { ips_id });
            Ok(())
        }

        /// Simply returns the current value.
        #[ink(message)]
        pub fn get_ipf(&self) -> Vec<CommonId> {
            self.ipf.clone()
        }

        #[ink(message)]
        pub fn get_ips(&self) -> Vec<CommonId> {
            self.ips.clone()
        }
    }
}

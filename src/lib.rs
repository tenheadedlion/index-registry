#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;

#[ink::contract]
mod registry {
    use alloc::{string::String, vec::Vec};
    use ink_storage::traits::{PackedLayout, SpreadLayout, StorageLayout};

    #[derive(Debug, Clone, Default, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Chain {
        id: i32,
        name: String,
        chain_type: i32,
    }

    #[derive(Debug, Clone, Default, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Asset {
        id: i32,
        symbol: String,
        chain_id: i32,
    }

    #[derive(Debug, Clone, Default, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Dex {
        id: i32,
        name: String,
        chain_id: i32,
    }

    #[derive(Debug, Clone, Default, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct DexIndexer {
        id: i32,
        url: String,
        dex_id: i32,
    }

    #[derive(Debug, Clone, Default, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct DexPair {
        id: i32,
        asset0_id: i32,
        asset1_id: i32,
        dex_id: i32,
        pair_id: String,
    }

    #[derive(Debug, Clone, Default, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Bridge {
        id: i32,
        name: String,
        location: String,
    }

    #[derive(Debug, Clone, Default, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct BridgePair {
        id: i32,
        asset0_id: i32,
        asset1_id: i32,
        bridge_id: i32,
    }

    #[derive(Debug, Clone, Default, scale::Encode, scale::Decode, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Graph {
        chains: Vec<Chain>,
        assets: Vec<Asset>,
        dexs: Vec<Dex>,
        dex_pairs: Vec<DexPair>,
        dex_indexers: Vec<DexIndexer>,
        bridges: Vec<Bridge>,
        bridge_pairs: Vec<BridgePair>,
    }

    impl Graph {}

    #[ink(storage)]
    pub struct Registry {
        value: Graph,
    }

    impl Registry {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: Graph::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn set_graph(&mut self, graph: Graph) {
            self.value = graph;
        }

        #[ink(message)]
        pub fn get_graph(&self) -> Graph {
            return self.value.clone();
        }
    }
}

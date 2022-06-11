#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod truhuis_address_registry {
    // Imports from ink!
    use ink_storage::traits::SpreadAllocate;

    // Imports from openbrush
    use openbrush::{contracts::ownable::*, modifiers};

    /// The TruhuisAddressRegistry error types.
    #[derive(Debug, PartialEq, Eq, Copy, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if update action failed.
        UpdateAuctionFailed,
        /// Returned if update cadastre failed.
        UpdateCurrencyRegistryFailed,
        /// Returned if update cadastre failed.
        UpdateCadastreFailed,
        /// Returned if update marketplace failed.
        UpdateMarketplaceFailed,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, OwnableStorage)]
    pub struct TruhuisAddressRegistry {
        pub auction: AccountId,
        pub currency_registry: AccountId,
        pub cadastre: AccountId,
        pub marketplace: AccountId,
        #[OwnableStorageField]
        pub ownable: OwnableData,
    }

    /// Event emitted when an update auction occurs.
    #[ink(event)]
    pub struct AuctionUpdated {
        #[ink(topic)]
        new: AccountId,
    }

    /// Event emitted when an update currency registry occurs.
    #[ink(event)]
    pub struct CurrencyRegistryUpdated {
        #[ink(topic)]
        new: AccountId,
    }

    /// Event emitted when an update cadastre occurs.
    #[ink(event)]
    pub struct CadastreUpdated {
        #[ink(topic)]
        new: AccountId,
    }

    /// Event emitted when an update marketplace occurs.
    #[ink(event)]
    pub struct MarketplaceUpdated {
        #[ink(topic)]
        new: AccountId,
    }

    // Section contains default implementation without any modifications.
    impl Ownable for TruhuisAddressRegistry {}

    // Section contains modified methods with additional functionality.
    impl TruhuisAddressRegistry {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {
                _instance._init_with_owner(_instance.env().caller());
            })
        }

        #[ink(message)]
        //#[modifiers(only_owner)]
        pub fn update_auction(&mut self, _new: AccountId) -> Result<(), Error> {
            self.auction = _new.clone();

            if self.auction != _new.clone() {
                return Err(Error::UpdateAuctionFailed);
            };

            self.env().emit_event(AuctionUpdated { new: _new });

            Ok(())
        }

        #[ink(message)]
        //#[modifiers(only_owner)]
        pub fn update_currency_registry(&mut self, _new: AccountId) -> Result<(), Error> {
            self.currency_registry = _new.clone();

            if self.currency_registry != _new.clone() {
                return Err(Error::UpdateCurrencyRegistryFailed);
            };

            self.env().emit_event(CurrencyRegistryUpdated { new: _new });

            Ok(())
        }

        #[ink(message)]
        //#[modifiers(only_owner)]
        pub fn update_cadastre(&mut self, _new: AccountId) -> Result<(), Error> {
            self.cadastre = _new.clone();

            if self.auction != _new.clone() {
                return Err(Error::UpdateCadastreFailed);
            };

            self.env().emit_event(CadastreUpdated { new: _new });

            Ok(())
        }

        #[ink(message)]
        //#[modifiers(only_owner)]
        pub fn update_marketplace(&mut self, _new: AccountId) -> Result<(), Error> {
            self.marketplace = _new.clone();

            if self.marketplace != _new.clone() {
                return Err(Error::UpdateMarketplaceFailed);
            };

            self.env().emit_event(MarketplaceUpdated { new: _new });

            Ok(())
        }
    }
}

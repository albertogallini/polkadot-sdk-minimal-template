//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
    use frame::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
        fn ed() ->  Balance;
        
    }

	#[pallet::pallet]
	pub struct Pallet<T>(_);

    pub type HashT = [u8; 4];
    pub type Timestamp = u64;
    pub type Balance = u128;

    #[pallet::storage]
    type TotalIssuance<T: Config>  = StorageValue<Value=Balance> ;
   
    #[pallet::storage]
    type Balances<T: Config> = StorageMap<Key = T::AccountId, Value = Balance>;

    #[pallet::call]
    impl <T:Config>  Pallet<T> {
        pub fn mint_safe(origin: T::RuntimeOrigin, amount: Balance) -> DispatchResult {
                let who = ensure_signed(origin)?;
                if amount < T::ed(){
                    return Err("BelowId".into());
                }

                if Balances::<T>::contains_key(&who) {
                    return Err("Error message".into())
                }
                Balances::<T>::insert(&who, amount);
                let mut issuance  = TotalIssuance::<T>::get().unwrap();
                issuance += amount; // unsafe sum  (check add to be used)
                TotalIssuance::<T>::put(issuance);            
                Ok(())
        }
    }

}


#[cfg(test)]
mod test {

    mod runtime {
    
        use super::super::pallet as pallet_currency;
        use super::super::Balance;
        use frame::testing_prelude::*;
        
        construct_runtime!(
            pub enum Runtime {
                System: frame_system,
                Currency: pallet_currency,
            }
        );

        #[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
        impl frame_system::Config for Runtime {
            type Block = MockBlock<Runtime>;
            // within pallet we just said `<T as frame_system::Config>::AccountId`, now we
            // finally specified it.
            type AccountId = u64;
        }

        // our simple pallet has nothing to be configured.
        impl pallet_currency::Config for Runtime {
            fn ed() -> Balance{
                5
            }
        }
    }
    


}

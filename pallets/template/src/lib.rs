#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use frame_support::{PalletId};

use frame_support::traits::{Currency, Imbalance, OnUnbalanced};

/// Hardcoded pallet ID; used to create the special Pot Account
/// Must be exactly 8 characters long
const PALLET_ID: PalletId = PalletId(*b"Charity!");

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use crate::BalanceOf;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement::AllowDeath},
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The currency type that the charity deals in
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Donor has made a charitable donation to the charity
		DonationReceived(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// An imbalance from elsewhere in the runtime has been absorbed by the Charity
		ImbalanceAbsorbed(BalanceOf<T>, BalanceOf<T>),
		/// Charity has allocated funds to a cause
		FundsAllocated(T::AccountId, BalanceOf<T>, BalanceOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::weight(10_000)]
		pub fn donate(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let donor = ensure_signed(origin)?;

			T::Currency::transfer(&donor, &Self::account_id(), amount, AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make donation"))?;

			Self::deposit_event(Event::DonationReceived(donor, amount, Self::pot()));
			Ok(().into())
		}

		// /// An example dispatchable that may throw a custom error.
		// #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		// pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	// Read a value from storage.
		// 	match <Something<T>>::get() {
		// 		// Return an error if the value has not been set.
		// 		None => return Err(Error::<T>::NoneValue.into()),
		// 		Some(old) => {
		// 			// Increment the value read from storage; will error in the event of overflow.
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			// Update the value in storage with the incremented result.
		// 			<Something<T>>::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }
	}

	
}
impl<T: Config> Pallet<T> {
	/// The account ID that holds the Charity's funds
	pub fn account_id() -> T::AccountId {
		sp_runtime::traits::AccountIdConversion::into_account_truncating(&PALLET_ID)
	}

	/// The Charity's balance
	fn pot() -> BalanceOf<T> {
		T::Currency::free_balance(&Self::account_id())
	}
}

// This implementation allows the charity to be the recipient of funds that are burned elsewhere in
// the runtime. For eample, it could be transaction fees, consensus-related slashing, or burns that
// align incentives in other pallets.
impl<T: Config> OnUnbalanced<NegativeImbalanceOf<T>> for Module<T> {
	fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<T>) {
		let numeric_amount = amount.peek();

		// Must resolve into existing but better to be safe.
		let _ = T::Currency::resolve_creating(&Self::account_id(), amount);

		Self::deposit_event(Event::ImbalanceAbsorbed(numeric_amount, Self::pot()));
	}
}

#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

use frame_support::traits::Currency;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {

	use crate::BalanceOf;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::Currency,
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

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn donate(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
		
	}
}

#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_module, decl_storage, decl_event, dispatch:: { Result, fmt::Debug }, Parameter};
use system::ensure_signed;
use sr_primitives::{ traits::{ Member } };

pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// ID which identifies a content
	type ContentIdentifier: Parameter + Member + Debug + Default + Copy;

	// Key of a property
	type PropertyKey: Parameter + Member + Debug + Default + Copy;

	// Value of a property
	type PropertyValue: Parameter + Member + Debug + Default + Copy;
}

decl_storage! {
	trait Store for Module<T: Trait> as Content {
		// Key/Value storage for each content
		ContentProperties get(content_properties): map (T::ContentIdentifier, T::PropertyKey) => T::PropertyValue;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		fn deposit_event() = default;

		fn set_property(origin, cid: T::ContentIdentifier, key: T::PropertyKey, value: T::PropertyValue) {
			let sender = ensure_signed(origin)?;
			<ContentProperties<T>>::insert((cid, key), value);
		}

	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		SomethingStored(u32, AccountId),
	}
);
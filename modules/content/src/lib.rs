#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_module, decl_storage, decl_event, dispatch:: { Result, fmt::Debug }, Parameter};
use system::ensure_signed;
use sr_primitives::{ traits::{ Member } };

pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// ID which identifies a content
	type ContentIdentifier: Parameter + Member + Debug + Copy;

	// Key of a property
	type PropertyKey: Parameter + Member + Debug + Copy;

	// Value of a property
	type PropertyValue: Parameter + Member + Debug + Copy;
}

decl_storage! {
	trait Store for Module<T: Trait> as Content {
		// Key/Value storage for each content
		ContentProperties get(content_properties): map (T::ContentIdentifier, T::PropertyKey) => Option<T::PropertyValue>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		fn deposit_event() = default;

		fn set_property(origin, cid: T::ContentIdentifier, key: T::PropertyKey, value: T::PropertyValue) {
			let sender = ensure_signed(origin)?;

			<ContentProperties<T>>::insert((cid, key), value);

			Self::deposit_event(RawEvent::ContentPropertySet(sender, cid, key, value));
		}

	}
}

decl_event!(
	pub enum Event<T> 
	where 
		AccountId = <T as system::Trait>::AccountId,
		ContentIdentifier = <T as Trait>::ContentIdentifier,
		PropertyKey = <T as Trait>::PropertyKey,
		PropertyValue = <T as Trait>::PropertyValue
	{
		SomethingStored(u32, AccountId),
		// A property of a content is set.
		ContentPropertySet(AccountId, ContentIdentifier, PropertyKey, PropertyValue),
	}
);
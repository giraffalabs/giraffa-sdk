#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_module, decl_storage, decl_event, dispatch:: { Result, fmt::Debug }, Parameter};
use system::ensure_signed;
use sr_primitives::RuntimeDebug;
use sr_primitives::{ traits::{ Member, SimpleArithmetic, Bounded, CheckedAdd } };
use codec::{Encode, Decode};
use graph_primitives:: { property:: { PropertyKey, PropertyKeyValue, PropertyValue } };

pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// ID which identifies a content
	type ContentIdentifier: Parameter + Member + Debug + Copy;

}

decl_storage! {
	trait Store for Module<T: Trait> as Content {
		// Key/Value storage for each content
		ContentProperties get(content_properties): map (T::ContentIdentifier, PropertyKey) => Option<PropertyValue<T::Hash, T::AccountId>>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		fn deposit_event() = default;

		fn create(origin, cid: T::ContentIdentifier) {
			let sender = ensure_signed(origin)?;

			let key = PropertyKey::from(PropertyKeyValue::Owner as u8); 
			let value = <PropertyValue<T::Hash, T::AccountId>>::AccountId(sender.clone());

			Self::do_set_property(cid, key, value.clone());

			Self::deposit_event(RawEvent::ContentPropertySet(sender, cid, key, value));
		}

		fn set_property(origin, cid: T::ContentIdentifier, key: PropertyKey, value: PropertyValue<T::Hash, T::AccountId>) {
			let sender = ensure_signed(origin)?;

			Self::do_set_property(cid, key, value.clone());

			Self::deposit_event(RawEvent::ContentPropertySet(sender, cid, key, value));
		}

	}
}

decl_event!(
	pub enum Event<T> 
	where 
		AccountId = <T as system::Trait>::AccountId,
		ContentIdentifier = <T as Trait>::ContentIdentifier,
		Hash = <T as system::Trait>::Hash
	{
		SomethingStored(u32, AccountId),
		// A property of a content is set.
		ContentPropertySet(AccountId, ContentIdentifier, PropertyKey, PropertyValue<Hash, AccountId>),
	}
);

impl<T: Trait> Module<T> {

	fn do_set_property(cid: T::ContentIdentifier, key: PropertyKey, value: PropertyValue<T::Hash, T::AccountId>) {
		<ContentProperties<T>>::insert((cid, key), value);
	}

}
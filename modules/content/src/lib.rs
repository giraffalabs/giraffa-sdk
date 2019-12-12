#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_module, decl_storage, decl_event, dispatch:: { Result, fmt::Debug }, Parameter, ensure };
use system::ensure_signed;
use sp_runtime::RuntimeDebug;
use sp_runtime::{ traits::{ Member, SimpleArithmetic, Bounded, CheckedAdd } };
use codec::{Codec, Encode, Decode};
use graph_primitives:: { property:: { PropertyKey, PropertyKeyValue, PropertyValue as PropertyValueT} };

pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// ID which identifies a content
	type ContentIdentifier: Parameter + Member + Debug + Copy + Codec;

}

type PropertyValue<T> = PropertyValueT<<T as system::Trait>::Hash, <T as system::Trait>::AccountId>;

decl_storage! {
	trait Store for Module<T: Trait> as Content {
		/// Key/Value storage for each content
		pub ContentProperties get(fn content_properties):
			double_map T::ContentIdentifier, blake2_256(PropertyKey) => Option<PropertyValue<T>>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		fn deposit_event() = default;


		fn create(origin, cid: T::ContentIdentifier) -> Result {
			// Check if signed
			let sender = ensure_signed(origin)?;

			let key = PropertyKey::from(PropertyKeyValue::Owner as u8); 
			let value = <PropertyValue<T>>::AccountId(sender.clone());

			Self::do_set_property(cid, key, value.clone());

			Self::deposit_event(RawEvent::ContentCreated(sender, cid));

			Ok(())
		}

		fn set_property(origin, cid: T::ContentIdentifier, key: PropertyKey, value: PropertyValue<T>) -> Result {
			// Check if signed
			let sender = ensure_signed(origin)?;
			// Check if owner of cid
			let owner_key = PropertyKey::from(PropertyKeyValue::Owner as u8);
			let wrap_owner = Self::content_properties(cid, owner_key).ok_or("Content does not exist")?;
			let owner = match wrap_owner {
				PropertyValueT::AccountId(owner) => owner,
				_ => return Err("Wrong type")
			};
			ensure!(owner == sender.clone(), "You are not the owner of the content");
			
			Self::do_set_property(cid, key, value.clone());

			Self::deposit_event(RawEvent::ContentPropertySet(sender, cid, key, value));

			Ok(())
		}

	}
}

decl_event!(
	pub enum Event<T> 
	where 
		AccountId = <T as system::Trait>::AccountId,
		ContentIdentifier = <T as Trait>::ContentIdentifier,
		Key = PropertyKey,
		Value = PropertyValue<T>,
	{
		SomethingStored(u32, AccountId),
		// A property of a content is set.
		ContentCreated(AccountId, ContentIdentifier),
		ContentPropertySet(AccountId, ContentIdentifier, Key, Value),
	}
);

impl<T: Trait> Module<T> {

	fn do_set_property(cid: T::ContentIdentifier, key: PropertyKey, value: PropertyValue<T>) {
		<ContentProperties<T>>::insert(cid, key, value);
	}

}
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

	/// LinkId
	type LinkIdentifier: Parameter + Member + Debug + Copy + Default + Bounded + SimpleArithmetic;

	/// Link Type
	type LinkType: Parameter + Member + Debug + Copy;

}

decl_storage! {
	trait Store for Module<T: Trait> as Links {
		// Link Count
		LinkCount get(link_count): T::LinkIdentifier;

		// return links id
		Links get(links): map (T::ContentIdentifier, T::ContentIdentifier, T::LinkType) => T::LinkIdentifier;

		// Key/Value storage for each link
		LinkProperties get(link_properties): map (T::LinkIdentifier, PropertyKey) => Option<PropertyValue<T::Hash, T::AccountId>>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		fn deposit_event() = default;

		fn link(origin, from: T::ContentIdentifier, to: T::ContentIdentifier, link_type: T::LinkType) {
			let sender = ensure_signed(origin)?;

			let count = Self::link_count();
			let one = T::LinkIdentifier::from(1);
			let new_count = count.checked_add(&one).ok_or("exceed maximum amount of links")?;

			<Links<T>>::insert((from, to, link_type), new_count);
			<LinkCount<T>>::put(new_count);

			Self::deposit_event(RawEvent::ContentLinked(sender, from, to, link_type));
		}

		fn set_property(origin, lid: T::LinkIdentifier, key: PropertyKey, value: PropertyValue<T::Hash, T::AccountId>) {
			let sender = ensure_signed(origin)?;

			<LinkProperties<T>>::insert((lid, key), value.clone());

			Self::deposit_event(RawEvent::LinkPropertySet(sender, lid, key, value));
		}

	}
}

decl_event!(
	pub enum Event<T> 
	where 
		AccountId = <T as system::Trait>::AccountId,
		ContentIdentifier = <T as Trait>::ContentIdentifier,
		LinkIdentifier = <T as Trait>::LinkIdentifier,
		LinkType = <T as Trait>::LinkType,
		Hash = <T as system::Trait>::Hash
	{
		SomethingStored(u32, AccountId),
		// A content was linked.
		ContentLinked(AccountId, ContentIdentifier, ContentIdentifier, LinkType),
		// A property of a link was set.
		LinkPropertySet(AccountId, LinkIdentifier, PropertyKey, PropertyValue<Hash, AccountId>),
	}
);
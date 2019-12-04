use sp_runtime::RuntimeDebug;
use codec::{Encode, Decode};

pub type PropertyKey = u64;

#[repr(u8)]
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub enum PropertyKeyValue {
	Owner = 0,
	Hello = 1,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub enum PropertyValue<Hash, AccountId> {
	Char32([u8; 32]),	
	Hash(Hash),
	Uint64(u64),
	Bool(bool),
	AccountId(AccountId)
}
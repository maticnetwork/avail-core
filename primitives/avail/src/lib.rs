#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use derive_more::{Add, Constructor, Display, From, Into, Mul};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::Perbill;

/// Customized headers.
pub mod header;
pub use header::*;

/// Kate Commitment on Headers.
pub mod kate_commitment;
pub use kate_commitment::*;

/// Application Specific Data Retrieval
pub mod asdr;

pub mod sha2;
pub mod traits;
pub use sha2::ShaTwo256;

pub mod data_proof;
pub use data_proof::DataProof;

pub mod well_known_keys {
	/// Public params used to generate Kate commitment
	pub const KATE_PUBLIC_PARAMS: &[u8] = b":kate_public_params:";
}

/// We allow `Normal` extrinsics to fill up the block up to 90%, the rest can be used
/// by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(90);

pub const BLOCK_CHUNK_SIZE: u32 = 32;

/// Money matters.
pub mod currency {

	pub type Balance = u128;

	/// AVL has 18 decimal positions.
	pub const AVL: Balance = 1_000_000_000_000_000_000;

	/// Cents of AVL has 16 decimal positions (100 Cents = $1)
	/// 1 DOLLARS = 10_000_000_000_000_000
	pub const CENTS: Balance = AVL / 100;

	/// Millicent of AVL has 13 decimal positions( 100 mCents = 1 cent).
	pub const MILLICENTS: Balance = CENTS / 1_000;
}

#[repr(u8)]
pub enum InvalidTransactionCustomId {
	/// The AppId is not registered.
	InvalidAppId = 137,
	/// Extrinsic is not allowed for the given `AppId`.
	ForbiddenAppId,
	/// Max padded length was exceeded.
	MaxPaddedLenExceeded,
}

/// Provides an implementation of [`frame_support::traits::Randomness`] that should only be used in
/// on Benchmarks!
pub struct BenchRandomness<T>(sp_std::marker::PhantomData<T>);

impl<Output, T> frame_support::traits::Randomness<Output, T> for BenchRandomness<T>
where
	Output: codec::Decode + Default,
	T: Default,
{
	fn random(subject: &[u8]) -> (Output, T) {
		use sp_runtime::traits::TrailingZeroInput;

		(
			Output::decode(&mut TrailingZeroInput::new(subject)).unwrap_or_default(),
			T::default(),
		)
	}
}

/// Strong type for `BlockLength::cols`
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
	Clone,
	Copy,
	Debug,
	From,
	Into,
	Add,
	Mul,
	Display,
	PartialEq,
	Eq,
	Encode,
	Decode,
	TypeInfo,
	PartialOrd,
	Ord,
	Constructor,
	MaxEncodedLen,
)]
#[mul(forward)]
pub struct BlockLengthColumns(#[codec(compact)] pub u32);

impl BlockLengthColumns {
	#[inline]
	pub fn as_usize(&self) -> usize {
		self.0 as usize
	}
}

/// Strong type for `BlockLength::rows`
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
	Clone,
	Copy,
	Debug,
	From,
	Into,
	Add,
	Mul,
	Display,
	PartialEq,
	Eq,
	Encode,
	Decode,
	TypeInfo,
	PartialOrd,
	Ord,
	Constructor,
	MaxEncodedLen,
)]
#[mul(forward)]
pub struct BlockLengthRows(#[codec(compact)] pub u32);

impl BlockLengthRows {
	#[inline]
	pub fn as_usize(&self) -> usize {
		self.0 as usize
	}
}

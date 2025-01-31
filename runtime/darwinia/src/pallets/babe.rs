// --- paritytech ---
use frame_support::traits::KeyOwnerProofSystem;
use pallet_babe::{AuthorityId, Config, EquivocationHandler, ExternalTrigger};
use sp_core::crypto::KeyTypeId;
// --- darwinia-network ---
use crate::*;

frame_support::parameter_types! {
	pub const EpochDuration: u64 = DARWINIA_BLOCKS_PER_SESSION as _;
	pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
	pub const ReportLongevity: u64 =
		BondingDurationInEra::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}

impl Config for Runtime {
	type DisabledValidators = Session;
	// session module is the trigger
	type EpochChangeTrigger = ExternalTrigger;
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type HandleEquivocation =
		EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		AuthorityId,
	)>>::IdentificationTuple;
	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, AuthorityId)>>::Proof;
	type KeyOwnerProofSystem = Historical;
	type MaxAuthorities = MaxAuthorities;
	type WeightInfo = ();
}

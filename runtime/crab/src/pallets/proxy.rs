// --- crates.io ---
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
// --- paritytech ---
use frame_support::traits::InstanceFilter;
use pallet_proxy::Config;
use sp_runtime::RuntimeDebug;
// --- darwinia-network ---
use crate::*;

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	TypeInfo,
)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	Staking,
	IdentityJudgement,
	EthereumBridge,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl InstanceFilter<Call> for ProxyType {
	fn filter(&self, c: &Call) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => matches!(
				c,
				Call::System{ .. } |
				Call::Babe{ .. } |
				Call::Timestamp{ .. } |
				Call::Indices(pallet_indices::Call::claim{ .. }) |
				Call::Indices(pallet_indices::Call::free{ .. }) |
				Call::Indices(pallet_indices::Call::freeze{ .. }) |
				// Specifically omitting Indices `transfer`, `force_transfer`
				// Specifically omitting the entire Balances pallet
				Call::Authorship{ .. } |
				Call::Democracy{ .. } |
				Call::Staking{ .. } |
				Call::Session{ .. } |
				Call::Grandpa{ .. } |
				Call::ImOnline{ .. } |
				Call::Council{ .. } |
				Call::TechnicalCommittee{ .. } |
				Call::PhragmenElection{ .. } |
				Call::TechnicalMembership{ .. } |
				Call::Treasury{ .. } |
				Call::Tips{ .. } |
				Call::Bounties{ .. } |
				Call::Utility{ .. } |
				Call::Identity{ .. } |
				Call::Society{ .. } |
				Call::Recovery(pallet_recovery::Call::as_recovered{ .. }) |
				Call::Recovery(pallet_recovery::Call::vouch_recovery{ .. }) |
				Call::Recovery(pallet_recovery::Call::claim_recovery{ .. }) |
				Call::Recovery(pallet_recovery::Call::close_recovery{ .. }) |
				Call::Recovery(pallet_recovery::Call::remove_recovery{ .. }) |
				Call::Recovery(pallet_recovery::Call::cancel_recovered{ .. }) |
				Call::Scheduler{ .. } |
				Call::Proxy{ .. } |
				Call::Multisig{ .. } // Specifically omitting the entire CrabIssuing pallet
			),
			ProxyType::Governance => {
				matches!(
					c,
					Call::Democracy { .. }
						| Call::Council { .. } | Call::TechnicalCommittee { .. }
						| Call::PhragmenElection { .. }
						| Call::Treasury { .. } | Call::Tips { .. }
						| Call::Bounties { .. } | Call::Utility { .. }
				)
			},
			ProxyType::Staking => matches!(c, Call::Staking { .. } | Call::Utility { .. }),
			ProxyType::IdentityJudgement => matches!(
				c,
				Call::Identity(pallet_identity::Call::provide_judgement { .. })
					| Call::Utility(pallet_utility::Call::batch { .. })
			),
			ProxyType::EthereumBridge => false,
		}
	}

	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

frame_support::parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub const ProxyDepositBase: Balance = old_crab_deposit(1, 8);
	// Additional storage item size of 33 bytes.
	pub const ProxyDepositFactor: Balance = old_crab_deposit(0, 33);
	pub const MaxProxies: u16 = 32;
	pub const AnnouncementDepositBase: Balance = old_crab_deposit(1, 8);
	pub const AnnouncementDepositFactor: Balance = old_crab_deposit(0, 66);
	pub const MaxPending: u16 = 32;
}

impl Config for Runtime {
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
	type Call = Call;
	type CallHasher = Hashing;
	type Currency = Ring;
	type Event = Event;
	type MaxPending = MaxPending;
	type MaxProxies = MaxProxies;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type ProxyType = ProxyType;
	type WeightInfo = ();
}

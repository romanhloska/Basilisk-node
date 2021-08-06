// This file is part of Basilisk-node.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate as price_oracle;
use crate::Config;
use frame_support::parameter_types;
use frame_support::traits::OnInitialize;
use frame_system;
use orml_traits::parameter_type_with_key;
use price_oracle::{PriceEntry, PriceOracleHandler};
use primitives::asset::AssetPair;
use primitives::{fee, traits::AssetPairAccountIdFor, AssetId, Balance, Price};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, Zero},
};

pub type Amount = i128;
pub type AccountId = u64;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const ASSET_PAIR_A: AssetPair = AssetPair {
	asset_in: 1_000,
	asset_out: 2_000,
};
pub const ASSET_PAIR_B: AssetPair = AssetPair {
	asset_in: 1_000,
	asset_out: 3_000,
};

pub const PRICE_ENTRY_1: PriceEntry = PriceEntry {
	price: Price::from_inner(2000000000000000000),
	amount: 1_000,
	liq_amount: 2_000,
};
pub const PRICE_ENTRY_2: PriceEntry = PriceEntry {
	price: Price::from_inner(5000000000000000000),
	amount: 3_000,
	liq_amount: 4_000,
};

frame_support::construct_runtime!(
	pub enum Test where
	 Block = Block,
	 NodeBlock = Block,
	 UncheckedExtrinsic = UncheckedExtrinsic,
	 {
		 System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		 PriceOracle: price_oracle::{Pallet, Call, Storage, Event<T>},
		 XYK: pallet_xyk::{Pallet, Call, Storage, Event<T>},
		 Currency: orml_tokens::{Pallet, Event<T>},
		 AssetRegistry: pallet_asset_registry::{Pallet, Storage},
	 }

);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 63;
}

impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

pub struct AssetPairAccountIdTest();

impl AssetPairAccountIdFor<AssetId, u64> for AssetPairAccountIdTest {
	fn from_assets(asset_a: AssetId, asset_b: AssetId) -> u64 {
		let mut a = asset_a as u128;
		let mut b = asset_b as u128;
		if a > b {
			let tmp = a;
			a = b;
			b = tmp;
		}
		return (a * 1000 + b) as u64;
	}
}

parameter_types! {
	pub const HdxAssetId: u32 = 0;
	pub ExchangeFeeRate: fee::Fee = fee::Fee::default();
}

impl pallet_xyk::Config for Test {
	type Event = Event;
	type AssetPairAccountId = AssetPairAccountIdTest;
	type Currency = Currency;
	type NativeAssetId = HdxAssetId;
	type WeightInfo = ();
	type GetExchangeFee = ExchangeFeeRate;
	type AMMHandler = PriceOracleHandler<Test>;
}

impl pallet_asset_registry::Config for Test {
	type AssetId = AssetId;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
		Zero::zero()
	};
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
}

impl Config for Test {
	type Event = Event;
}

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		sp_io::TestExternalities::from(storage)
	}
}

fn next_block() {
	System::set_block_number(System::block_number() + 1);
	PriceOracle::on_initialize(System::block_number());
}

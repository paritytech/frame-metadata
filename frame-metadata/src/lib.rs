// This file is part of Substrate.

// Copyright (C) 2018-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Decodable variant of the RuntimeMetadata.

#![cfg_attr(not(feature = "std"), no_std)]

cfg_if::cfg_if! {
	if #[cfg(feature = "std")] {
		use codec::{Decode, Error, Input};
		use serde::{
			Deserialize,
			Serialize,
		};
	} else {
		extern crate alloc;
		use alloc::vec::Vec;
	}
}

use codec::{Encode, Output};

#[cfg(feature = "v12")]
pub mod v12;

#[cfg(feature = "v13")]
pub mod v13;

cfg_if::cfg_if! {
	if #[cfg(not(feature = "v13"))] {
		/// Dummy trait in place of `scale_info::form::FormString`.
		/// Since the `scale-info` crate is only imported for the `v13` feature.
		pub trait FormString {}

		impl FormString for &'static str {}
		#[cfg(feature = "std")]
		impl FormString for String {}
	} else {
		pub(crate) use scale_info::form::FormString;
	}
}

/// Metadata prefixed by a u32 for reserved usage
#[derive(Eq, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(feature = "std", serde(bound(serialize = "S: Serialize")))]
pub struct RuntimeMetadataPrefixed<S: FormString = &'static str>(pub u32, pub RuntimeMetadata<S>);

impl Into<Vec<u8>> for RuntimeMetadataPrefixed {
	fn into(self) -> Vec<u8> {
		self.encode()
	}
}

/// The metadata of a runtime.
/// The version ID encoded/decoded through
/// the enum nature of `RuntimeMetadata`.
#[derive(Eq, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(feature = "std", serde(bound(serialize = "S: Serialize")))]
pub enum RuntimeMetadata<S: FormString = &'static str> {
	/// Unused; enum filler.
	V0(core::marker::PhantomData<S>),
	/// Version 1 for runtime metadata. No longer used.
	V1(RuntimeMetadataDeprecated),
	/// Version 2 for runtime metadata. No longer used.
	V2(RuntimeMetadataDeprecated),
	/// Version 3 for runtime metadata. No longer used.
	V3(RuntimeMetadataDeprecated),
	/// Version 4 for runtime metadata. No longer used.
	V4(RuntimeMetadataDeprecated),
	/// Version 5 for runtime metadata. No longer used.
	V5(RuntimeMetadataDeprecated),
	/// Version 6 for runtime metadata. No longer used.
	V6(RuntimeMetadataDeprecated),
	/// Version 7 for runtime metadata. No longer used.
	V7(RuntimeMetadataDeprecated),
	/// Version 8 for runtime metadata. No longer used.
	V8(RuntimeMetadataDeprecated),
	/// Version 9 for runtime metadata. No longer used.
	V9(RuntimeMetadataDeprecated),
	/// Version 10 for runtime metadata. No longer used.
	V10(RuntimeMetadataDeprecated),
	/// Version 11 for runtime metadata. No longer used.
	V11(RuntimeMetadataDeprecated),
	/// Version 12 for runtime metadata
	#[cfg(feature = "v12")]
	V12(v12::RuntimeMetadataV12<S>),
	/// Version 12 for runtime metadata, as raw encoded bytes.
	#[cfg(not(feature = "v12"))]
	V12(OpaqueMetadata),
	/// Version 13 for runtime metadata.
	#[cfg(feature = "v13")]
	V13(v13::RuntimeMetadataV13<S>),
	/// Version 13 for runtime metadata, as raw encoded bytes.
	#[cfg(not(feature = "v13"))]
	V13(OpaqueMetadata),
}

/// Stores the encoded `RuntimeMetadata` as raw bytes.
#[derive(Encode, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct OpaqueMetadata(pub Vec<u8>);

/// Enum that should fail.
#[derive(Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum RuntimeMetadataDeprecated {}

impl Encode for RuntimeMetadataDeprecated {
	fn encode_to<W: Output>(&self, _dest: &mut W) {}
}

impl codec::EncodeLike for RuntimeMetadataDeprecated {}

#[cfg(feature = "std")]
impl Decode for RuntimeMetadataDeprecated {
	fn decode<I: Input>(_input: &mut I) -> Result<Self, Error> {
		Err("Decoding is not supported".into())
	}
}

// Copyright (C) Parity Technologies (UK) Ltd.
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

#[cfg(feature = "decode")]
use codec::Decode;
#[cfg(feature = "serde_full")]
use serde::Serialize;

use super::RuntimeMetadataPrefixed;
use codec::Encode;
use scale_info::prelude::vec::Vec;
use scale_info::{
	form::{Form, MetaForm, PortableForm},
	IntoPortable, MetaType, PortableRegistry, Registry,
};

pub use crate::common::{
	PalletCallMetadata, PalletConstantMetadata, PalletErrorMetadata, PalletEventMetadata,
	PalletStorageMetadata, SignedExtensionMetadata, StorageEntryMetadata, StorageEntryModifier,
	StorageEntryType, StorageHasher,
};

/// Current prefix of metadata
pub const META_RESERVED: u32 = 0x6174656d; // 'meta' warn endianness

/// Latest runtime metadata
pub type RuntimeMetadataLastVersion = RuntimeMetadataV14;

impl From<RuntimeMetadataLastVersion> for super::RuntimeMetadataPrefixed {
	fn from(metadata: RuntimeMetadataLastVersion) -> RuntimeMetadataPrefixed {
		RuntimeMetadataPrefixed(META_RESERVED, super::RuntimeMetadata::V14(metadata))
	}
}

/// The metadata of a runtime.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
pub struct RuntimeMetadataV14 {
	/// Type registry containing all types used in the metadata.
	pub types: PortableRegistry,
	/// Metadata of all the pallets.
	pub pallets: Vec<PalletMetadata<PortableForm>>,
	/// Metadata of the extrinsic.
	pub extrinsic: ExtrinsicMetadata<PortableForm>,
	/// The type of the `Runtime`.
	pub ty: <PortableForm as Form>::Type,
}

impl RuntimeMetadataV14 {
	/// Create a new instance of [`RuntimeMetadataV14`].
	pub fn new(
		pallets: Vec<PalletMetadata>,
		extrinsic: ExtrinsicMetadata,
		runtime_type: MetaType,
	) -> Self {
		let mut registry = Registry::new();
		let pallets = registry.map_into_portable(pallets);
		let extrinsic = extrinsic.into_portable(&mut registry);
		let ty = registry.register_type(&runtime_type);
		Self {
			types: registry.into(),
			pallets,
			extrinsic,
			ty,
		}
	}
}

/// Metadata of the extrinsic used by the runtime.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct ExtrinsicMetadata<T: Form = MetaForm> {
	/// The type of the extrinsic.
	pub ty: T::Type,
	/// Extrinsic version.
	pub version: u8,
	/// The signed extensions in the order they appear in the extrinsic.
	pub signed_extensions: Vec<SignedExtensionMetadata<T>>,
}

impl IntoPortable for ExtrinsicMetadata {
	type Output = ExtrinsicMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		ExtrinsicMetadata {
			ty: registry.register_type(&self.ty),
			version: self.version,
			signed_extensions: registry.map_into_portable(self.signed_extensions),
		}
	}
}

/// All metadata about an runtime pallet.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletMetadata<T: Form = MetaForm> {
	/// Pallet name.
	pub name: T::String,
	/// Pallet storage metadata.
	pub storage: Option<PalletStorageMetadata<T>>,
	/// Pallet calls metadata.
	pub calls: Option<PalletCallMetadata<T>>,
	/// Pallet event metadata.
	pub event: Option<PalletEventMetadata<T>>,
	/// Pallet constants metadata.
	pub constants: Vec<PalletConstantMetadata<T>>,
	/// Pallet error metadata.
	pub error: Option<PalletErrorMetadata<T>>,
	/// Define the index of the pallet, this index will be used for the encoding of pallet event,
	/// call and origin variants.
	pub index: u8,
}

impl IntoPortable for PalletMetadata {
	type Output = PalletMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletMetadata {
			name: self.name.into_portable(registry),
			storage: self.storage.map(|storage| storage.into_portable(registry)),
			calls: self.calls.map(|calls| calls.into_portable(registry)),
			event: self.event.map(|event| event.into_portable(registry)),
			constants: registry.map_into_portable(self.constants),
			error: self.error.map(|error| error.into_portable(registry)),
			index: self.index,
		}
	}
}

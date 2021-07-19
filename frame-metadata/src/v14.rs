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

cfg_if::cfg_if! {
	if #[cfg(feature = "std")] {
		use codec::Decode;
		use serde::Serialize;
	}
}

use super::RuntimeMetadataPrefixed;
use codec::Encode;
use scale_info::prelude::{
	string::String,
	vec::Vec,
};
use scale_info::{
	form::{Form, MetaForm, PortableForm},
	IntoPortable, MetaType, PortableRegistry, Registry,
};

/// Current prefix of metadata
pub const META_RESERVED: u32 = 0x6174656d; // 'meta' warn endianness

pub type RuntimeMetadataLastVersion = RuntimeMetadataV14;

impl From<RuntimeMetadataLastVersion> for super::RuntimeMetadataPrefixed {
	fn from(metadata: RuntimeMetadataLastVersion) -> RuntimeMetadataPrefixed {
		RuntimeMetadataPrefixed(META_RESERVED, super::RuntimeMetadata::V14(metadata))
	}
}

/// The metadata of a runtime.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
pub struct RuntimeMetadataV14 {
	pub types: PortableRegistry,
	/// Metadata of all the pallets.
	pub pallets: Vec<PalletMetadata<PortableForm>>,
	/// Metadata of the extrinsic.
	pub extrinsic: ExtrinsicMetadata<PortableForm>,
}

impl RuntimeMetadataV14 {
	pub fn new(pallets: Vec<PalletMetadata>, extrinsic: ExtrinsicMetadata) -> Self {
		let mut registry = Registry::new();
		let pallets = registry.map_into_portable(pallets);
		let extrinsic = extrinsic.into_portable(&mut registry);
		Self {
			types: registry.into(),
			pallets,
			extrinsic,
		}
	}
}

/// Metadata of the extrinsic used by the runtime.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
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

/// Metadata of an extrinsic's signed extension.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct SignedExtensionMetadata<T: Form = MetaForm> {
	/// The unique signed extension identifier, which may be different from the type name.
	pub identifier: T::String,
	/// The type of the signed extension, with the data to be included in the extrinsic.
	pub ty: T::Type,
	/// The type of the additional signed data, with the data to be included in the signed payload
	pub additional_signed: T::Type,
}

impl IntoPortable for SignedExtensionMetadata {
	type Output = SignedExtensionMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		SignedExtensionMetadata {
			identifier: self.identifier.into_portable(registry),
			ty: registry.register_type(&self.ty),
			additional_signed: registry.register_type(&self.additional_signed),
		}
	}
}

/// All metadata about an runtime pallet.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub storage: Option<PalletStorageMetadata<T>>,
	pub calls: Option<PalletCallMetadata<T>>,
	pub event: Option<PalletEventMetadata<T>>,
	pub constants: Vec<PalletConstantMetadata<T>>,
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

/// All metadata of the pallet's storage.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletStorageMetadata<T: Form = MetaForm> {
	/// The common prefix used by all storage entries.
	pub prefix: T::String,
	pub entries: Vec<StorageEntryMetadata<T>>,
}

impl IntoPortable for PalletStorageMetadata {
	type Output = PalletStorageMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletStorageMetadata {
			prefix: self.prefix.into_portable(registry),
			entries: registry.map_into_portable(self.entries),
		}
	}
}

/// Metadata about one storage entry.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct StorageEntryMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub modifier: StorageEntryModifier,
	pub ty: StorageEntryType<T>,
	pub default: Vec<u8>,
	docs: Vec<T::String>,
}

impl IntoPortable for StorageEntryMetadata {
	type Output = StorageEntryMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		StorageEntryMetadata {
			name: self.name.into_portable(registry),
			modifier: self.modifier,
			ty: self.ty.into_portable(registry),
			default: self.default,
			docs: registry.map_into_portable(self.docs),
		}
	}
}

impl StorageEntryMetadata<MetaForm> {
	/// Create a new [`StorageEntryMetadata`].
	pub fn new(
		name: &'static str,
		modifier: StorageEntryModifier,
		ty: StorageEntryType<MetaForm>,
		default: Vec<u8>,
	) -> Self {
		StorageEntryMetadata {
			name,
			modifier,
			ty,
			default,
			docs: Vec::new(),
		}
	}

	#[cfg(feature = "docs")]
	/// Set the documentation.
	pub fn with_docs(mut self, docs: &[&'static str]) -> Self {
		self.docs = docs.to_vec();
		self
	}

	#[cfg(not(feature = "docs"))]
	/// Docs feature is not enabled so this is a no-op.
	#[inline]
	pub fn with_docs(mut self, _docs: &[&'static str]) -> Self {
		self
	}
}

impl StorageEntryMetadata<PortableForm> {
	/// Get the documentation.
	pub fn docs(&self) -> &[String] {
		&self.docs
	}
}

/// A storage entry modifier.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
pub enum StorageEntryModifier {
	Optional,
	Default,
}

/// Hasher used by storage maps
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
pub enum StorageHasher {
	Blake2_128,
	Blake2_256,
	Blake2_128Concat,
	Twox128,
	Twox256,
	Twox64Concat,
	Identity,
}

/// A storage entry type.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub enum StorageEntryType<T: Form = MetaForm> {
	Plain(T::Type),
	Map {
		hasher: StorageHasher,
		key: T::Type,
		value: T::Type,
	},
	DoubleMap {
		hasher: StorageHasher,
		key1: T::Type,
		key2: T::Type,
		value: T::Type,
		key2_hasher: StorageHasher,
	},
	NMap {
		keys: T::Type,
		hashers: Vec<StorageHasher>,
		value: T::Type,
	},
}

impl IntoPortable for StorageEntryType {
	type Output = StorageEntryType<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		match self {
			Self::Plain(plain) => StorageEntryType::Plain(registry.register_type(&plain)),
			Self::Map { hasher, key, value } => StorageEntryType::Map {
				hasher,
				key: registry.register_type(&key),
				value: registry.register_type(&value),
			},
			Self::DoubleMap {
				hasher,
				key1,
				key2,
				value,
				key2_hasher,
			} => StorageEntryType::DoubleMap {
				hasher,
				key1: registry.register_type(&key1),
				key2: registry.register_type(&key2),
				value: registry.register_type(&value),
				key2_hasher,
			},
			StorageEntryType::NMap {
				keys,
				hashers,
				value,
			} => StorageEntryType::NMap {
				keys: registry.register_type(&keys),
				hashers,
				value: registry.register_type(&value),
			},
		}
	}
}

/// Metadata for all calls in a pallet
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletCallMetadata<T: Form = MetaForm> {
	/// The corresponding enum type for the pallet call.
	pub ty: T::Type,
}

impl IntoPortable for PalletCallMetadata {
	type Output = PalletCallMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletCallMetadata {
			ty: registry.register_type(&self.ty),
		}
	}
}

impl From<MetaType> for PalletCallMetadata {
	fn from(ty: MetaType) -> Self {
		Self { ty }
	}
}

/// Metadata about the pallet event type.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
pub struct PalletEventMetadata<T: Form = MetaForm> {
	pub ty: T::Type,
}

impl IntoPortable for PalletEventMetadata {
	type Output = PalletEventMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletEventMetadata {
			ty: registry.register_type(&self.ty),
		}
	}
}

impl From<MetaType> for PalletEventMetadata {
	fn from(ty: MetaType) -> Self {
		Self { ty }
	}
}

/// Metadata about one pallet constant.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletConstantMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub ty: T::Type,
	pub value: Vec<u8>,
	docs: Vec<T::String>,
}

impl IntoPortable for PalletConstantMetadata {
	type Output = PalletConstantMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletConstantMetadata {
			name: self.name.into_portable(registry),
			ty: registry.register_type(&self.ty),
			value: self.value,
			docs: registry.map_into_portable(self.docs),
		}
	}
}

impl PalletConstantMetadata {
	pub fn new(name: &'static str, ty: MetaType, value: Vec<u8>) -> Self {
		Self {
			name,
			ty,
			value,
			docs: Vec::new(),
		}
	}

	#[cfg(feature = "docs")]
	/// Set the documentation.
	pub fn with_docs(mut self, docs: &[&'static str]) -> Self {
		self.docs = docs.to_vec();
		self
	}

	#[cfg(not(feature = "docs"))]
	/// Docs feature is not enabled so this is a no-op.
	#[inline]
	pub fn with_docs(mut self, _docs: &[&'static str]) -> Self {
		self
	}
}

impl PalletConstantMetadata<PortableForm> {
	/// Get the documentation.
	pub fn docs(&self) -> &[String] {
		&self.docs
	}
}

/// Metadata about a pallet error.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(feature = "std", serde(bound(serialize = "T::Type: Serialize")))]
pub struct PalletErrorMetadata<T: Form = MetaForm> {
	/// The error type information.
	pub ty: T::Type,
}

impl IntoPortable for PalletErrorMetadata {
	type Output = PalletErrorMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletErrorMetadata {
			ty: registry.register_type(&self.ty),
		}
	}
}

impl From<MetaType> for PalletErrorMetadata {
	fn from(ty: MetaType) -> Self {
		Self { ty }
	}
}

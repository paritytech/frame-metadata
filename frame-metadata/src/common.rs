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

//! Shared metadata types between different versions.

#[cfg(feature = "decode")]
use codec::Decode;
#[cfg(feature = "serde_full")]
use serde::Serialize;

use codec::Encode;
use scale_info::{
	form::{Form, MetaForm, PortableForm},
	prelude::vec::Vec,
	IntoPortable, MetaType, Registry,
};

/// Metadata of an extrinsic's signed extension.
#[cfg(any(feature = "v14", feature = "v15-unstable"))]
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
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

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
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

/// All metadata of the pallet's storage.
#[cfg(any(feature = "v14", feature = "v15-unstable"))]
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletStorageMetadata<T: Form = MetaForm> {
	/// The common prefix used by all storage entries.
	pub prefix: T::String,
	/// Metadata for all storage entries.
	pub entries: Vec<StorageEntryMetadata<T>>,
}

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
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
#[cfg(any(feature = "v14", feature = "v15-unstable"))]
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct StorageEntryMetadata<T: Form = MetaForm> {
	/// Variable name of the storage entry.
	pub name: T::String,
	/// An `Option` modifier of that storage entry.
	pub modifier: StorageEntryModifier,
	/// Type of the value stored in the entry.
	pub ty: StorageEntryType<T>,
	/// Default value (SCALE encoded).
	pub default: Vec<u8>,
	/// Storage entry documentation.
	pub docs: Vec<T::String>,
}

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
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

/// A storage entry modifier indicates how a storage entry is returned when fetched and what the value will be if the key is not present.
/// Specifically this refers to the "return type" when fetching a storage entry, and what the value will be if the key is not present.
///
/// `Optional` means you should expect an `Option<T>`, with `None` returned if the key is not present.
/// `Default` means you should expect a `T` with the default value of default if the key is not present.
#[cfg(any(feature = "v14", feature = "v15-unstable"))]
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
pub enum StorageEntryModifier {
	/// The storage entry returns an `Option<T>`, with `None` if the key is not present.
	Optional,
	/// The storage entry returns `T::Default` if the key is not present.
	Default,
}

/// Hasher used by storage maps
#[cfg(any(feature = "v14", feature = "v15-unstable"))]
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
pub enum StorageHasher {
	/// 128-bit Blake2 hash.
	Blake2_128,
	/// 256-bit Blake2 hash.
	Blake2_256,
	/// Multiple 128-bit Blake2 hashes concatenated.
	Blake2_128Concat,
	/// 128-bit XX hash.
	Twox128,
	/// 256-bit XX hash.
	Twox256,
	/// Multiple 64-bit XX hashes concatenated.
	Twox64Concat,
	/// Identity hashing (no hashing).
	Identity,
}

/// A type of storage value.
#[cfg(any(feature = "v14", feature = "v15-unstable"))]
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub enum StorageEntryType<T: Form = MetaForm> {
	/// Plain storage entry (just the value).
	Plain(T::Type),
	/// A storage map.
	Map {
		/// One or more hashers, should be one hasher per key element.
		hashers: Vec<StorageHasher>,
		/// The type of the key, can be a tuple with elements for each of the hashers.
		key: T::Type,
		/// The type of the value.
		value: T::Type,
	},
}

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
impl IntoPortable for StorageEntryType {
	type Output = StorageEntryType<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		match self {
			Self::Plain(plain) => StorageEntryType::Plain(registry.register_type(&plain)),
			Self::Map {
				hashers,
				key,
				value,
			} => StorageEntryType::Map {
				hashers,
				key: registry.register_type(&key),
				value: registry.register_type(&value),
			},
		}
	}
}

/// Metadata for all calls in a pallet
#[cfg(any(feature = "v14", feature = "v15-unstable"))]
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletCallMetadata<T: Form = MetaForm> {
	/// The corresponding enum type for the pallet call.
	pub ty: T::Type,
}

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
impl IntoPortable for PalletCallMetadata {
	type Output = PalletCallMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletCallMetadata {
			ty: registry.register_type(&self.ty),
		}
	}
}

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
impl From<MetaType> for PalletCallMetadata {
	fn from(ty: MetaType) -> Self {
		Self { ty }
	}
}

/// Metadata about the pallet Event type.
#[cfg(any(feature = "v14", feature = "v15-unstable"))]
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
pub struct PalletEventMetadata<T: Form = MetaForm> {
	/// The Event type.
	pub ty: T::Type,
}

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
impl IntoPortable for PalletEventMetadata {
	type Output = PalletEventMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletEventMetadata {
			ty: registry.register_type(&self.ty),
		}
	}
}

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
impl From<MetaType> for PalletEventMetadata {
	fn from(ty: MetaType) -> Self {
		Self { ty }
	}
}

/// Metadata about one pallet constant.
#[cfg(any(feature = "v14", feature = "v15-unstable"))]
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletConstantMetadata<T: Form = MetaForm> {
	/// Name of the pallet constant.
	pub name: T::String,
	/// Type of the pallet constant.
	pub ty: T::Type,
	/// Value stored in the constant (SCALE encoded).
	pub value: Vec<u8>,
	/// Documentation of the constant.
	pub docs: Vec<T::String>,
}

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
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

/// Metadata about a pallet error.
#[cfg(any(feature = "v14", feature = "v15-unstable"))]
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(feature = "serde_full", serde(bound(serialize = "T::Type: Serialize")))]
pub struct PalletErrorMetadata<T: Form = MetaForm> {
	/// The error type information.
	pub ty: T::Type,
}

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
impl IntoPortable for PalletErrorMetadata {
	type Output = PalletErrorMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletErrorMetadata {
			ty: registry.register_type(&self.ty),
		}
	}
}

#[cfg(any(feature = "v14", feature = "v15-unstable"))]
impl From<MetaType> for PalletErrorMetadata {
	fn from(ty: MetaType) -> Self {
		Self { ty }
	}
}

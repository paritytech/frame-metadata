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
use scale_info::prelude::vec::Vec;
use scale_info::{
	form::{Form, MetaForm, PortableForm},
	meta_type, IntoPortable, PortableRegistry, Registry, TypeInfo,
};

/// Current prefix of metadata
pub const META_RESERVED: u32 = 0x6174656d; // 'meta' warn endianness

/// Type alias placeholder for `ByteGetter` equivalent. todo: [AJ] figure out what to do here
pub type ByteGetter = Vec<u8>;

pub type RuntimeMetadataLastVersion = RuntimeMetadataV13;

impl From<RuntimeMetadataLastVersion> for super::RuntimeMetadataPrefixed {
	fn from(metadata: RuntimeMetadataLastVersion) -> RuntimeMetadataPrefixed {
		RuntimeMetadataPrefixed(META_RESERVED, super::RuntimeMetadata::V13(metadata))
	}
}

/// The metadata of a runtime.
// todo: [AJ] add back clone derive if required (requires PortableRegistry to implement clone)
#[derive(PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
pub struct RuntimeMetadataV13 {
	pub types: PortableRegistry,
	/// Metadata of all the pallets.
	pub pallets: Vec<PalletMetadata<PortableForm>>,
	/// Metadata of the extrinsic.
	pub extrinsic: ExtrinsicMetadata<PortableForm>,
}

impl RuntimeMetadataV13 {
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
	/// The signed extensions in the order they appear in the extrinsic.
	pub ty: T::Type,
}

impl IntoPortable for SignedExtensionMetadata {
	type Output = SignedExtensionMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		SignedExtensionMetadata {
			identifier: self.identifier.into_portable(registry),
			ty: registry.register_type(&self.ty),
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
	pub storage: Option<StorageMetadata<T>>,
	pub calls: Option<PalletCallMetadata<T>>,
	pub event: Option<PalletEventMetadata<T>>,
	pub constants: Vec<PalletConstantMetadata<T>>,
	pub errors: ErrorMetadata<T>,
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
			errors: self.errors.into_portable(registry),
			index: self.index,
		}
	}
}

/// All metadata of the storage.module
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct StorageMetadata<T: Form = MetaForm> {
	/// The common prefix used by all storage entries.
	pub prefix: T::String,
	pub entries: Vec<StorageEntryMetadata<T>>,
}

impl IntoPortable for StorageMetadata {
	type Output = StorageMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		StorageMetadata {
			prefix: self.prefix.into_portable(registry),
			entries: registry.map_into_portable(self.entries),
		}
	}
}

/// All the metadata about one storage entry.
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
	pub default: ByteGetter,
	pub documentation: Vec<T::String>,
}

impl IntoPortable for StorageEntryMetadata {
	type Output = StorageEntryMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		StorageEntryMetadata {
			name: self.name.into_portable(registry),
			modifier: self.modifier,
			ty: self.ty.into_portable(registry),
			default: self.default,
			documentation: registry.map_into_portable(self.documentation),
		}
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
		// is_linked flag previously, unused now to keep backwards compat
		unused: bool,
	},
	DoubleMap {
		hasher: StorageHasher,
		key1: T::Type,
		key2: T::Type,
		value: T::Type,
		key2_hasher: StorageHasher,
	},
}

impl IntoPortable for StorageEntryType {
	type Output = StorageEntryType<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		match self {
			Self::Plain(plain) => StorageEntryType::Plain(registry.register_type(&plain)),
			Self::Map {
				hasher,
				key,
				value,
				unused,
			} => StorageEntryType::Map {
				hasher,
				key: registry.register_type(&key),
				value: registry.register_type(&value),
				unused,
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
	pub calls: Vec<FunctionMetadata<T>>,
}

impl IntoPortable for PalletCallMetadata {
	type Output = PalletCallMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletCallMetadata {
			ty: registry.register_type(&self.ty),
			calls: registry.map_into_portable(self.calls),
		}
	}
}

/// All the metadata about a function.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct FunctionMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub arguments: Vec<FunctionArgumentMetadata<T>>,
	pub documentation: Vec<T::String>,
}

impl IntoPortable for FunctionMetadata {
	type Output = FunctionMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		FunctionMetadata {
			name: self.name.into_portable(registry),
			arguments: registry.map_into_portable(self.arguments),
			documentation: registry.map_into_portable(self.documentation),
		}
	}
}

/// All the metadata about a function argument.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
pub struct FunctionArgumentMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub ty: T::Type,
}

impl IntoPortable for FunctionArgumentMetadata {
	type Output = FunctionArgumentMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		FunctionArgumentMetadata {
			name: self.name.into_portable(registry),
			ty: registry.register_type(&self.ty),
		}
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

/// All the metadata about one pallet constant.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletConstantMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub ty: T::Type,
	pub value: ByteGetter,
	pub documentation: Vec<T::String>,
}

impl IntoPortable for PalletConstantMetadata {
	type Output = PalletConstantMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletConstantMetadata {
			name: self.name.into_portable(registry),
			ty: registry.register_type(&self.ty),
			value: self.value,
			documentation: registry.map_into_portable(self.documentation),
		}
	}
}

/// Metadata about a pallet error.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(feature = "std", serde(bound(serialize = "T::Type: Serialize")))]
pub struct ErrorMetadata<T: Form = MetaForm> {
	/// The error type information.
	pub ty: T::Type,
}

impl IntoPortable for ErrorMetadata {
	type Output = ErrorMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		ErrorMetadata {
			ty: registry.register_type(&self.ty),
		}
	}
}

/// A type specification.
///
/// This contains the actual type as well as an optional compile-time
/// known displayed representation of the type. This is useful for cases
/// where the type is used through a type alias in order to provide
/// information about the alias name.
///
/// # Examples
///
/// Consider the following Rust function:
/// ```no_compile
/// fn is_sorted(input: &[i32], pred: Predicate) -> bool;
/// ```
/// In this above example `input` would have no displayable name,
/// `pred`'s display name is `Predicate` and the display name of
/// the return type is simply `bool`. Note that `Predicate` could
/// simply be a type alias to `fn(i32, i32) -> Ordering`.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct TypeSpec<T: Form = MetaForm> {
	/// The actual type.
	pub ty: T::Type,
	/// The compile-time known displayed representation of the type.
	pub name: T::String,
}

impl IntoPortable for TypeSpec {
	type Output = TypeSpec<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		TypeSpec {
			ty: registry.register_type(&self.ty),
			name: self.name.into_portable(registry),
		}
	}
}

impl TypeSpec {
	/// Creates a new type specification without a display name.
	pub fn new<T>(name: &'static str) -> Self
	where
		T: TypeInfo + 'static,
	{
		Self {
			ty: meta_type::<T>(),
			name,
		}
	}
}

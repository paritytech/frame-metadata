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
	/// Metadata of all the modules.
	pub modules: Vec<ModuleMetadata<PortableForm>>,
	/// Metadata of the extrinsic.
	pub extrinsic: ExtrinsicMetadata<PortableForm>,
}

impl RuntimeMetadataV13 {
	pub fn new(modules: Vec<ModuleMetadata>, extrinsic: ExtrinsicMetadata) -> Self {
		let mut registry = Registry::new();
		let modules = registry.map_into_portable(modules);
		let extrinsic = extrinsic.into_portable(&mut registry);
		Self {
			types: registry.into(),
			modules,
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

/// All metadata about an runtime module.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct ModuleMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub storage: Option<Vec<StorageMetadata<T>>>,
	pub calls: Option<Vec<FunctionMetadata<T>>>,
	pub event: Option<Vec<EventMetadata<T>>>,
	pub constants: Option<Vec<ModuleConstantMetadata<T>>>,
	pub errors: Vec<ErrorMetadata<T>>,
	/// Define the index of the module, this index will be used for the encoding of module event,
	/// call and origin variants.
	pub index: u8,
}

impl IntoPortable for ModuleMetadata {
	type Output = ModuleMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		ModuleMetadata {
			name: self.name.into_portable(registry),
			storage: self
				.storage
				.map(|storage| registry.map_into_portable(storage)),
			calls: self.calls.map(|calls| registry.map_into_portable(calls)),
			event: self.event.map(|event| registry.map_into_portable(event)),
			constants: self
				.constants
				.map(|constant| registry.map_into_portable(constant)),
			errors: registry.map_into_portable(self.errors),
			index: self.index,
		}
	}
}

/// All metadata of the storage.
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
	Plain(T::String),
	Map {
		hasher: StorageHasher,
		key: T::String,
		value: T::String,
		// is_linked flag previously, unused now to keep backwards compat
		unused: bool,
	},
	DoubleMap {
		hasher: StorageHasher,
		key1: T::String,
		key2: T::String,
		value: T::String,
		key2_hasher: StorageHasher,
	},
}

impl IntoPortable for StorageEntryType {
	type Output = StorageEntryType<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		match self {
			Self::Plain(plain) => StorageEntryType::Plain(plain.into_portable(registry)),
			Self::Map {
				hasher,
				key,
				value,
				unused,
			} => StorageEntryType::Map {
				hasher,
				key: key.into_portable(registry),
				value: value.into_portable(registry),
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
				key1: key1.into_portable(registry),
				key2: key2.into_portable(registry),
				value: value.into_portable(registry),
				key2_hasher,
			},
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
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct FunctionArgumentMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub ty: T::Type,
	pub is_compact: bool,
}

impl IntoPortable for FunctionArgumentMetadata {
	type Output = FunctionArgumentMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		FunctionArgumentMetadata {
			name: self.name.into_portable(registry),
			ty: registry.register_type(&self.ty),
			is_compact: self.is_compact,
		}
	}
}

/// All the metadata about an outer event.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct OuterEventMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub events: Vec<ModuleEventMetadata<T>>,
}

impl IntoPortable for OuterEventMetadata {
	type Output = OuterEventMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		OuterEventMetadata {
			name: self.name.into_portable(registry),
			events: registry.map_into_portable(self.events),
		}
	}
}

/// Metadata about a module event.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct ModuleEventMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub events: Vec<EventMetadata<T>>,
}

impl IntoPortable for ModuleEventMetadata {
	type Output = ModuleEventMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		ModuleEventMetadata {
			name: self.name.into_portable(registry),
			events: registry.map_into_portable(self.events),
		}
	}
}

/// All the metadata about an event.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct EventMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub arguments: Vec<TypeSpec<T>>,
	pub documentation: Vec<T::String>,
}

impl IntoPortable for EventMetadata {
	type Output = EventMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		EventMetadata {
			name: self.name.into_portable(registry),
			arguments: registry.map_into_portable(self.arguments),
			documentation: registry.map_into_portable(self.documentation),
		}
	}
}

/// All the metadata about one module constant.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct ModuleConstantMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub ty: T::Type,
	pub value: ByteGetter,
	pub documentation: Vec<T::String>,
}

impl IntoPortable for ModuleConstantMetadata {
	type Output = ModuleConstantMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		ModuleConstantMetadata {
			name: self.name.into_portable(registry),
			ty: registry.register_type(&self.ty),
			value: self.value,
			documentation: registry.map_into_portable(self.documentation),
		}
	}
}

/// All the metadata about a module error.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Debug))]
#[cfg_attr(
	feature = "std",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct ErrorMetadata<T: Form = MetaForm> {
	pub name: T::String,
	pub documentation: Vec<T::String>,
}

impl IntoPortable for ErrorMetadata {
	type Output = ErrorMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		ErrorMetadata {
			name: self.name.into_portable(registry),
			documentation: registry.map_into_portable(self.documentation),
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

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
#![warn(missing_docs)]

#[cfg(feature = "std")]
use codec::{Decode, Error, Input};
use codec::{Encode, Output};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
type StringBuf = String;

/// Current prefix of metadata
pub const META_RESERVED: u32 = 0x6174656d; // 'meta' warn endianness

/// On `no_std` we do not support `Decode` and thus `StringBuf` is just `&'static str`.
/// So, if someone tries to decode this stuff on `no_std`, they will get a compilation error.
#[cfg(not(feature = "std"))]
type StringBuf = &'static str;

/// A type that decodes to a different type than it encodes.
/// The user needs to make sure that both types use the same encoding.
///
/// For example a `&'static [ &'static str ]` can be decoded to a `Vec<String>`.
#[derive(Clone)]
pub enum DecodeDifferent<B, O>
where
	B: 'static,
	O: 'static,
{
	/// Encodable variant of the value (doesn't need to be decodeable).
	Encode(B),
	/// Encodable & decodeable variant of the value.
	Decoded(O),
}

impl<B, O> Encode for DecodeDifferent<B, O>
where
	B: Encode + 'static,
	O: Encode + 'static,
{
	fn encode_to<W: Output>(&self, dest: &mut W) {
		match self {
			DecodeDifferent::Encode(b) => b.encode_to(dest),
			DecodeDifferent::Decoded(o) => o.encode_to(dest),
		}
	}
}

impl<B, O> codec::EncodeLike for DecodeDifferent<B, O>
where
	B: Encode + 'static,
	O: Encode + 'static,
{
}

#[cfg(feature = "std")]
impl<B, O> Decode for DecodeDifferent<B, O>
where
	B: 'static,
	O: Decode + 'static,
{
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		<O>::decode(input).map(|val| DecodeDifferent::Decoded(val))
	}
}

impl<B, O> PartialEq for DecodeDifferent<B, O>
where
	B: Encode + Eq + PartialEq + 'static,
	O: Encode + Eq + PartialEq + 'static,
{
	fn eq(&self, other: &Self) -> bool {
		self.encode() == other.encode()
	}
}

impl<B, O> Eq for DecodeDifferent<B, O>
where
	B: Encode + Eq + PartialEq + 'static,
	O: Encode + Eq + PartialEq + 'static,
{
}

impl<B, O> core::fmt::Debug for DecodeDifferent<B, O>
where
	B: core::fmt::Debug + Eq + 'static,
	O: core::fmt::Debug + Eq + 'static,
{
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			DecodeDifferent::Encode(b) => b.fmt(f),
			DecodeDifferent::Decoded(o) => o.fmt(f),
		}
	}
}

#[cfg(feature = "std")]
impl<B, O> serde::Serialize for DecodeDifferent<B, O>
where
	B: serde::Serialize + 'static,
	O: serde::Serialize + 'static,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			DecodeDifferent::Encode(b) => b.serialize(serializer),
			DecodeDifferent::Decoded(o) => o.serialize(serializer),
		}
	}
}

#[cfg(feature = "std")]
impl<'de, B, O> serde::Deserialize<'de> for DecodeDifferent<B, O>
where
	O: serde::Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let d = O::deserialize(deserializer)?;
		Ok(DecodeDifferent::Decoded(d))
	}
}

impl<B, O, T> core::ops::Deref for DecodeDifferent<B, O>
where
	B: core::ops::Deref<Target = T>,
	O: core::ops::Deref<Target = T>,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match *self {
			Self::Encode(ref e) => &**e,
			Self::Decoded(ref d) => &**d,
		}
	}
}

/// A encodeable/decodable type abstracting static array slices and a vector.
pub type DecodeDifferentArray<B, O = B> = DecodeDifferent<&'static [B], Vec<O>>;

impl<B> DecodeDifferentArray<B> {
	/// Return a slice of contained data.
	pub fn as_slice(&self) -> &[B] {
		match *self {
			Self::Encode(ref e) => e,
			Self::Decoded(ref d) => &**d,
		}
	}
}

type DecodeDifferentStr = DecodeDifferent<&'static str, StringBuf>;

#[cfg(feature = "std")]
impl DecodeDifferentStr {
	/// Allocate an owned version of the string.
	pub fn to_string(&self) -> String {
		match *self {
			Self::Encode(ref e) => e.to_string(),
			Self::Decoded(ref d) => d.clone(),
		}
	}
}

/// All the metadata about a function.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct FunctionMetadata {
	/// Function name.
	pub name: DecodeDifferentStr,
	/// A list of arguments this function takes.
	pub arguments: DecodeDifferentArray<FunctionArgumentMetadata>,
	/// Function documentation.
	pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}

/// All the metadata about a function argument.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct FunctionArgumentMetadata {
	/// Name of the variable for the argument.
	pub name: DecodeDifferentStr,
	/// Type of the parameter.
	pub ty: DecodeDifferentStr,
}

/// Newtype wrapper for support encoding functions (actual the result of the function).
#[derive(Clone, Eq)]
pub struct FnEncode<E>(pub fn() -> E)
where
	E: Encode + 'static;

impl<E: Encode> Encode for FnEncode<E> {
	fn encode_to<W: Output>(&self, dest: &mut W) {
		self.0().encode_to(dest);
	}
}

impl<E: Encode> codec::EncodeLike for FnEncode<E> {}

impl<E: Encode + PartialEq> PartialEq for FnEncode<E> {
	fn eq(&self, other: &Self) -> bool {
		self.0().eq(&other.0())
	}
}

impl<E: Encode + core::fmt::Debug> core::fmt::Debug for FnEncode<E> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		self.0().fmt(f)
	}
}

#[cfg(feature = "std")]
impl<E: Encode + serde::Serialize> serde::Serialize for FnEncode<E> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0().serialize(serializer)
	}
}

/// All the metadata about an outer event.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct OuterEventMetadata {
	/// Name of the event.
	pub name: DecodeDifferentStr,
	/// A list of event details.
	pub events: DecodeDifferentArray<
		(&'static str, FnEncode<&'static [EventMetadata]>),
		(StringBuf, Vec<EventMetadata>),
	>,
}

/// All the metadata about an event.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct EventMetadata {
	/// Name of the event.
	pub name: DecodeDifferentStr,
	/// Arguments of the event.
	pub arguments: DecodeDifferentArray<&'static str, StringBuf>,
	/// Documentation of the event.
	pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}

/// All the metadata about one storage entry.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct StorageEntryMetadata {
	/// Variable name of the storage entry.
	pub name: DecodeDifferentStr,
	/// An `Option` modifier of that storage entry.
	pub modifier: StorageEntryModifier,
	/// Type of the value stored in the entry.
	pub ty: StorageEntryType,
	/// Default value (SCALE encoded).
	pub default: ByteGetter,
	/// Storage entry documentation.
	pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}

/// All the metadata about one module constant.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct ModuleConstantMetadata {
	/// Name of the module constant.
	pub name: DecodeDifferentStr,
	/// Type of the module constant.
	pub ty: DecodeDifferentStr,
	/// Value stored in the constant (SCALE encoded).
	pub value: ByteGetter,
	/// Documentation of the constant.
	pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}

/// All the metadata about a module error.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct ErrorMetadata {
	/// Name of the error.
	pub name: DecodeDifferentStr,
	/// Error variant documentation.
	pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}

/// All the metadata about errors in a module.
pub trait ModuleErrorMetadata {
	/// Returns error metadata.
	fn metadata() -> &'static [ErrorMetadata];
}

impl ModuleErrorMetadata for &'static str {
	fn metadata() -> &'static [ErrorMetadata] {
		&[]
	}
}

/// A technical trait to store lazy initiated vec value as static dyn pointer.
pub trait DefaultByte: Send + Sync {
	/// A default value (SCALE encoded).
	fn default_byte(&self) -> Vec<u8>;
}

/// Wrapper over dyn pointer for accessing a cached once byte value.
#[derive(Clone)]
pub struct DefaultByteGetter(pub &'static dyn DefaultByte);

/// Decode different for static lazy initiated byte value.
pub type ByteGetter = DecodeDifferent<DefaultByteGetter, Vec<u8>>;

impl Encode for DefaultByteGetter {
	fn encode_to<W: Output>(&self, dest: &mut W) {
		self.0.default_byte().encode_to(dest)
	}
}

impl codec::EncodeLike for DefaultByteGetter {}

impl PartialEq<DefaultByteGetter> for DefaultByteGetter {
	fn eq(&self, other: &DefaultByteGetter) -> bool {
		let left = self.0.default_byte();
		let right = other.0.default_byte();
		left.eq(&right)
	}
}

impl Eq for DefaultByteGetter {}

#[cfg(feature = "std")]
impl serde::Serialize for DefaultByteGetter {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.default_byte().serialize(serializer)
	}
}

impl core::fmt::Debug for DefaultByteGetter {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		self.0.default_byte().fmt(f)
	}
}

/// Hasher used by storage maps
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub enum StorageHasher {
	/// 128-bit Blake2 hash.
	Blake2_128,
	/// 256-bit Blake2 hash.
	Blake2_256,
	/// 128-bit Blake2 concatenating multiple hashes.
	Blake2_128Concat,
	/// 128-bit XX hash.
	Twox128,
	/// 256-bit XX hash.
	Twox256,
	/// 64-bit XX hashes concatentation.
	Twox64Concat,
	/// Identity hashing (no hashing).
	Identity,
}

/// A storage entry type.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub enum StorageEntryType {
	/// Plain storage entry (just the value).
	Plain(DecodeDifferentStr),
	/// A storage map.
	Map {
		/// Hasher type for the keys.
		hasher: StorageHasher,
		/// Key type.
		key: DecodeDifferentStr,
		/// Value type.
		value: DecodeDifferentStr,
		///
		/// NOTE is_linked flag previously, unused now to keep backwards compat
		/// with SCALE encoding.
		unused: bool,
	},
	/// Storage Double Map.
	DoubleMap {
		/// Hasher type for the keys.
		hasher: StorageHasher,
		/// First key type.
		key1: DecodeDifferentStr,
		/// Second key type.
		key2: DecodeDifferentStr,
		/// Value type.
		value: DecodeDifferentStr,
		/// Hasher for the second key.
		key2_hasher: StorageHasher,
	},
}

/// A storage entry modifier.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub enum StorageEntryModifier {
	/// The value may not be set.
	Optional,
	/// If the value is not set it will resolve to default value.
	Default,
}

/// All metadata of the storage.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct StorageMetadata {
	/// The common prefix used by all storage entries.
	pub prefix: DecodeDifferent<&'static str, StringBuf>,
	/// A list of all storage entries.
	pub entries: DecodeDifferent<&'static [StorageEntryMetadata], Vec<StorageEntryMetadata>>,
}

/// Metadata prefixed by a u32 for reserved usage
#[derive(Eq, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct RuntimeMetadataPrefixed(
	/// Magic prefix number.
	pub u32,
	/// Runtime metadata.
	pub RuntimeMetadata,
);

/// Metadata of the extrinsic used by the runtime.
#[derive(Eq, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct ExtrinsicMetadata {
	/// Extrinsic version.
	pub version: u8,
	/// The signed extensions in the order they appear in the extrinsic.
	pub signed_extensions: Vec<DecodeDifferentStr>,
}

/// The metadata of a runtime.
/// The version ID encoded/decoded through
/// the enum nature of `RuntimeMetadata`.
#[derive(Eq, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub enum RuntimeMetadata {
	/// Unused; enum filler.
	V0(RuntimeMetadataDeprecated),
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
	/// Version 12 for runtime metadata.
	#[cfg(feature = "v12")]
	V12(RuntimeMetadataV12),
	#[cfg(not(feature = "v12"))]
	V12(RuntimeMetadataDeprecated),
}

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

/// The metadata of a runtime.
#[derive(Eq, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
#[cfg(feature = "v12")]
pub struct RuntimeMetadataV12 {
	/// Metadata of all the modules.
	pub modules: DecodeDifferentArray<ModuleMetadata>,
	/// Metadata of the extrinsic.
	pub extrinsic: ExtrinsicMetadata,
}

/// The latest version of the metadata.
#[cfg(feature = "v12")]
pub type RuntimeMetadataLastVersion = RuntimeMetadataV12;

#[cfg(feature = "v12")]
impl Into<RuntimeMetadataPrefixed> for RuntimeMetadataLastVersion {
	fn into(self) -> RuntimeMetadataPrefixed {
		RuntimeMetadataPrefixed(META_RESERVED, RuntimeMetadata::V12(self))
	}
}

/// All metadata about an runtime module.
#[derive(Clone, PartialEq, Eq, Encode)]
#[cfg_attr(feature = "std", derive(Decode, Serialize, Deserialize, Debug))]
pub struct ModuleMetadata {
	/// Name of the pallet.
	pub name: DecodeDifferentStr,
	/// Optional storage items.
	pub storage: Option<DecodeDifferent<FnEncode<StorageMetadata>, StorageMetadata>>,
	/// [`Option`] of public calls (dispatchables) to the pallet. See [`ODFnA`].
	pub calls: ODFnA<FunctionMetadata>,
	/// Events the pallet may generate.
	pub event: ODFnA<EventMetadata>,
	/// Constant values defined on the module (public parameters).
	pub constants: DFnA<ModuleConstantMetadata>,
	/// Errors the calls may generate.
	pub errors: DFnA<ErrorMetadata>,
	/// Define the index of the module, this index will be used for the encoding of module event,
	/// call and origin variants.
	pub index: u8,
}

type ODFnA<T> = Option<DFnA<T>>;
type DFnA<T> = DecodeDifferent<FnEncode<&'static [T]>, Vec<T>>;

impl Into<Vec<u8>> for RuntimeMetadataPrefixed {
	fn into(self) -> Vec<u8> {
		self.encode()
	}
}

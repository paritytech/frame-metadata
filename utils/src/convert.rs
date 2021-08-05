// This file is part of frame-metadata.

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

use frame_metadata::{
	decode_different::{DecodeDifferent, DecodeDifferentArray, DecodeDifferentStr},
	v13, v14, RuntimeMetadata, RuntimeMetadataPrefixed,
};
use scale_info::{
	form::{Form, PortableForm},
	Type, TypeDef, TypeDefPrimitive,
};

pub type MetadataConversionError = String;
pub type Result<T> = core::result::Result<T, MetadataConversionError>;

/// Convert the current version to the previous version.
pub fn backwards(metadata: RuntimeMetadataPrefixed) -> Result<RuntimeMetadataPrefixed> {
	match metadata.1 {
		RuntimeMetadata::V14(v14) => Ok(RuntimeMetadataPrefixed(
			metadata.0,
			RuntimeMetadata::V13(v14_to_v13(v14)?),
		)),
		_ => Err(format!(
			"Unsupported metadata version V{}, currently only V14 to v13 conversion supported",
			metadata.1.version()
		)),
	}
}

/// Convert V14 metadata to V13.
pub fn v14_to_v13(metadata: v14::RuntimeMetadataV14) -> Result<v13::RuntimeMetadataV13> {
	let converter = Converter { metadata };
	converter.convert()
}

struct Converter {
	metadata: v14::RuntimeMetadataV14,
}

impl Converter {
	fn convert(&self) -> Result<v13::RuntimeMetadataV13> {
		let modules = self
			.metadata
			.pallets
			.iter()
			.map(|p| self.convert_pallet(p))
			.collect::<Result<Vec<_>>>()?;
		let extrinsic = self.convert_extrinsic()?;
		Ok(v13::RuntimeMetadataV13 {
			modules: DecodeDifferentArray::Decoded(modules),
			extrinsic,
		})
	}

	fn convert_extrinsic(&self) -> Result<v13::ExtrinsicMetadata> {
		let extrinsic = &self.metadata.extrinsic;
		let signed_extensions = extrinsic
			.signed_extensions
			.iter()
			.map(|se| DecodeDifferent::Decoded(se.identifier.clone()))
			.collect::<Vec<_>>();
		Ok(v13::ExtrinsicMetadata {
			version: extrinsic.version,
			signed_extensions,
		})
	}

	fn convert_pallet(
		&self,
		pallet: &v14::PalletMetadata<PortableForm>,
	) -> Result<v13::ModuleMetadata> {
		let name = pallet.name.clone();
		let storage = pallet
			.storage
			.as_ref()
			.map(|storage| {
				self.convert_pallet_storage(storage)
					.map(DecodeDifferent::Decoded)
			})
			.transpose()?;
		let calls = pallet
			.calls
			.as_ref()
			.map(|call| self.convert_call(call).map(DecodeDifferent::Decoded))
			.transpose()?;
		let event = pallet
			.event
			.as_ref()
			.map(|event| self.convert_event(event).map(DecodeDifferent::Decoded))
			.transpose()?;
		let constants = pallet
			.constants
			.iter()
			.map(|constant| self.convert_constant(constant))
			.collect::<Result<Vec<_>>>()?;
		let errors = pallet
			.error
			.as_ref()
			.map(|err| self.convert_error(err))
			.unwrap_or_else(|| Ok(Vec::new()))?;
		Ok(v13::ModuleMetadata {
			name: DecodeDifferent::Decoded(name),
			storage,
			calls,
			event,
			constants: DecodeDifferent::Decoded(constants),
			errors: DecodeDifferent::Decoded(errors),
			index: pallet.index,
		})
	}

	fn convert_pallet_storage(
		&self,
		storage: &v14::PalletStorageMetadata<PortableForm>,
	) -> Result<v13::StorageMetadata> {
		let entries = storage
			.entries
			.iter()
			.map(|entry| {
				let modifier = match entry.modifier {
					v14::StorageEntryModifier::Optional => v13::StorageEntryModifier::Optional,
					v14::StorageEntryModifier::Default => v13::StorageEntryModifier::Default,
				};

				let ty = self.convert_storage_entry_type(&entry.ty)?;
				Ok(v13::StorageEntryMetadata {
					name: DecodeDifferent::Decoded(entry.name.clone()),
					modifier,
					ty,
					default: DecodeDifferent::Decoded(entry.default.clone()),
					documentation: DecodeDifferent::Decoded(entry.docs.clone()),
				})
			})
			.collect::<Result<Vec<_>>>()?;
		Ok(v13::StorageMetadata {
			prefix: DecodeDifferent::Decoded(storage.prefix.clone()),
			entries: DecodeDifferent::Decoded(entries),
		})
	}

	fn convert_storage_entry_type(
		&self,
		storage_entry_type: &v14::StorageEntryType<PortableForm>,
	) -> Result<v13::StorageEntryType> {
		let convert_hasher = |hasher: &v14::StorageHasher| match hasher {
			v14::StorageHasher::Blake2_128 => v13::StorageHasher::Blake2_128,
			v14::StorageHasher::Blake2_256 => v13::StorageHasher::Blake2_256,
			v14::StorageHasher::Blake2_128Concat => v13::StorageHasher::Blake2_128Concat,
			v14::StorageHasher::Twox128 => v13::StorageHasher::Twox128,
			v14::StorageHasher::Twox256 => v13::StorageHasher::Twox256,
			v14::StorageHasher::Twox64Concat => v13::StorageHasher::Twox64Concat,
			v14::StorageHasher::Identity => v13::StorageHasher::Identity,
		};

		match storage_entry_type {
			v14::StorageEntryType::<PortableForm>::Plain(key) => {
				let type_ident = self.get_type_ident(key)?;
				Ok(v13::StorageEntryType::Plain(DecodeDifferent::Decoded(
					type_ident,
				)))
			}
			v14::StorageEntryType::<PortableForm>::Map {
				hashers,
				key,
				value,
			} => {
				let keys_ty = self.resolve_type(key)?;
				let key_idents = match keys_ty.type_def() {
					TypeDef::Tuple(tuple) => tuple
						.fields()
						.iter()
						.map(|f| self.get_type_ident(f))
						.collect::<Result<Vec<_>>>(),
					_ => Ok(vec![self.get_type_ident(key)?]),
				}?;

				match &hashers[..] {
					[] => panic!("Expected at least one hasher"),
					[hasher] => Ok(v13::StorageEntryType::Map {
						hasher: convert_hasher(hasher),
						key: DecodeDifferentStr::Decoded(self.get_type_ident(key)?),
						value: DecodeDifferentStr::Decoded(self.get_type_ident(value)?),
						unused: false,
					}),
					[hasher, key2_hasher] => match &key_idents[..] {
						[key1, key2] => Ok(v13::StorageEntryType::DoubleMap {
							hasher: convert_hasher(hasher),
							key1: DecodeDifferentStr::Decoded(key1.clone()),
							key2: DecodeDifferentStr::Decoded(key2.clone()),
							value: DecodeDifferentStr::Decoded(self.get_type_ident(value)?),
							key2_hasher: convert_hasher(key2_hasher),
						}),
						_ => Err(format!(
							"Expected two keys for a DoubleMap, found {:?}",
							key_idents
						)),
					},
					hashers => {
						let hashers = hashers.iter().map(convert_hasher).collect::<Vec<_>>();
						Ok(v13::StorageEntryType::NMap {
							keys: DecodeDifferent::Decoded(key_idents),
							hashers: DecodeDifferent::Decoded(hashers),
							value: DecodeDifferentStr::Decoded(self.get_type_ident(value)?),
						})
					}
				}
			}
		}
	}

	fn convert_call(
		&self,
		call: &v14::PalletCallMetadata<PortableForm>,
	) -> Result<Vec<v13::FunctionMetadata>> {
		let ty = self.resolve_type(&call.ty)?;

		if let TypeDef::Variant(call) = ty.type_def() {
			call.variants()
				.iter()
				.map(|variant| {
					let arguments = variant
						.fields()
						.iter()
						.map(|f| {
							let name = f
								.name()
								.ok_or_else(|| format!("Expected named variant fields"))?;
							Ok(v13::FunctionArgumentMetadata {
								name: DecodeDifferent::Decoded(name.clone()),
								ty: DecodeDifferentStr::Decoded(self.get_type_ident(f.ty())?),
							})
						})
						.collect::<Result<Vec<_>>>()?;
					Ok(v13::FunctionMetadata {
						name: DecodeDifferent::Decoded(variant.name().clone()),
						arguments: DecodeDifferentArray::Decoded(arguments),
						documentation: DecodeDifferentArray::Decoded(variant.docs().to_vec()),
					})
				})
				.collect()
		} else {
			Err("Call type should be an enum/variant type".into())
		}
	}

	fn convert_event(
		&self,
		event: &v14::PalletEventMetadata<PortableForm>,
	) -> Result<Vec<v13::EventMetadata>> {
		let ty = self.resolve_type(&event.ty)?;

		if let TypeDef::Variant(event) = ty.type_def() {
			event
				.variants()
				.iter()
				.map(|variant| {
					let arguments = variant
						.fields()
						.iter()
						.map(|f| self.get_type_ident(f.ty()))
						.collect::<Result<Vec<_>>>()?;
					Ok(v13::EventMetadata {
						name: DecodeDifferentStr::Decoded(variant.name().clone()),
						arguments: DecodeDifferentArray::Decoded(arguments),
						documentation: DecodeDifferentArray::Decoded(variant.docs().to_vec()),
					})
				})
				.collect()
		} else {
			Err("Event type should be an enum/variant type".into())
		}
	}

	fn convert_constant(
		&self,
		constant: &v14::PalletConstantMetadata<PortableForm>,
	) -> Result<v13::ModuleConstantMetadata> {
		Ok(v13::ModuleConstantMetadata {
			name: DecodeDifferentStr::Decoded(constant.name.clone()),
			ty: DecodeDifferentStr::Decoded(self.get_type_ident(&constant.ty)?),
			value: DecodeDifferent::Decoded(constant.value.clone()),
			documentation: DecodeDifferentArray::Decoded(constant.docs.to_vec()),
		})
	}

	fn convert_error(
		&self,
		error: &v14::PalletErrorMetadata<PortableForm>,
	) -> Result<Vec<v13::ErrorMetadata>> {
		let ty = self.resolve_type(&error.ty)?;
		if let TypeDef::Variant(error) = ty.type_def() {
			error
				.variants()
				.iter()
				.map(|variant| {
					Ok(v13::ErrorMetadata {
						name: DecodeDifferentStr::Decoded(variant.name().clone()),
						documentation: DecodeDifferentArray::Decoded(variant.docs().to_vec()),
					})
				})
				.collect()
		} else {
			Err("Call type should be an enum/variant type".into())
		}
	}

	fn resolve_type(&self, ty: &<PortableForm as Form>::Type) -> Result<&Type<PortableForm>> {
		self.metadata
			.types
			.resolve(ty.id())
			.ok_or_else(|| format!("Type {} not found", ty.id()))
	}

	fn get_type_ident(&self, ty: &<PortableForm as Form>::Type) -> Result<String> {
		let ty = self.resolve_type(ty)?;
		match ty.type_def() {
			TypeDef::Composite(_) | TypeDef::Variant(_) => ty
				.path()
				.ident()
				.ok_or_else(|| format!("Type should have an indent")),
			TypeDef::Sequence(seq) => {
				Ok(format!("Vec<{}>", self.get_type_ident(seq.type_param())?))
			}
			TypeDef::Array(arr) => Ok(format!(
				"[{}; {}]",
				self.get_type_ident(arr.type_param())?,
				arr.len()
			)),
			TypeDef::Tuple(tuple) => {
				let elements = tuple
					.fields()
					.iter()
					.map(|f| self.get_type_ident(f))
					.collect::<Result<Vec<_>>>()?;
				Ok(format!("({})", elements.join(", ")))
			}
			TypeDef::Primitive(primitive) => {
				let type_str = match primitive {
					TypeDefPrimitive::Bool => "bool",
					TypeDefPrimitive::Char => "char",
					TypeDefPrimitive::Str => "str",
					TypeDefPrimitive::U8 => "u8",
					TypeDefPrimitive::U16 => "u16",
					TypeDefPrimitive::U32 => "u32",
					TypeDefPrimitive::U64 => "u64",
					TypeDefPrimitive::U128 => "u128",
					TypeDefPrimitive::U256 => "U256",
					TypeDefPrimitive::I8 => "i8",
					TypeDefPrimitive::I16 => "i16",
					TypeDefPrimitive::I32 => "i32",
					TypeDefPrimitive::I64 => "i64",
					TypeDefPrimitive::I128 => "i128",
					TypeDefPrimitive::I256 => "I256",
				};
				Ok(type_str.to_string())
			}
			TypeDef::Compact(compact) => Ok(format!(
				"Compact<{}>",
				self.get_type_ident(compact.type_param())?
			)),
			TypeDef::BitSequence(bitvec) => Ok(format!(
				"BitVec<{}, {}>",
				self.get_type_ident(bitvec.bit_order_type())?,
				self.get_type_ident(bitvec.bit_store_type())?
			)),
		}
	}
}

#[cfg(test)]
mod tests {
	use codec::Decode;
	use frame_metadata::RuntimeMetadataPrefixed;
	use std::{env, fs, io::Read, path};

	#[test]
	fn substrate_node_runtime_v14_to_v13() {
		let root = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
		let root_path = path::Path::new(&root);

		// generate with:
		// curl -sX POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"state_getMetadata", "id": 1}' localhost:9933 \
		// | jq .result \
		// | cut -d '"' -f 2 \
		// | xxd -r -p > ./utils/node-runtime-v13.scale
		//
		let path = root_path.join("node-runtime-v13.scale");

		let mut file = fs::File::open(path).expect("Error opening metadata file");
		let mut bytes = Vec::new();
		file.read_to_end(&mut bytes).expect("Error reading metadata file");

		let metadata = RuntimeMetadataPrefixed::decode(&mut &bytes[..]).expect("Error decoding metadata file");
	}
}

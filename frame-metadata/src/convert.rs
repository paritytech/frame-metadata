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

#[cfg(not(feature = "std"))]
use alloc::{
    string::String,
    vec::Vec,
    format
};

use core::{
    convert::TryFrom,
};
use crate::decode_different::{DecodeDifferent, DecodeDifferentArray, DecodeDifferentStr};
use crate::{v13, v14, StorageHasher};
use scale_info::form::{Form, PortableForm};
use scale_info::{TypeDef, TypeDefPrimitive, Type};

pub type MetadataConversionError = String;
pub type Result<T> = core::result::Result<T, MetadataConversionError>;

impl TryFrom<v14::RuntimeMetadataV14> for v13::RuntimeMetadataV13 {
    type Error = MetadataConversionError;

    fn try_from(metadata: v14::RuntimeMetadataV14) -> Result<Self> {
        let converter = Converter { metadata };
        converter.convert()
    }
}

struct Converter {
    metadata: v14::RuntimeMetadataV14,
}

impl Converter {
    fn convert(&self) -> Result<v13::RuntimeMetadataV13> {
        let modules = self.metadata.pallets
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
        let signed_extensions = extrinsic.signed_extensions
            .iter()
            .map(|se| DecodeDifferent::Decoded(se.identifier))
            .collect::<Vec<_>>();
        Ok(v13::ExtrinsicMetadata {
            version: extrinsic.version,
            signed_extensions
        })
    }

    fn convert_pallet(&self, pallet: &v14::PalletMetadata<PortableForm>) -> Result<v13::ModuleMetadata> {
        let name = pallet.name.clone();
        let storage = pallet
            .storage
            .as_ref()
            .map(|storage|
                self.convert_pallet_storage(storage).map(DecodeDifferent::Decoded)
            )
            .transpose()?;
        let calls = pallet
            .calls
            .as_ref()
            .map(|call|
                self.convert_call(call).map(DecodeDifferent::Decoded)
            )
            .transpose()?;
        let event = pallet
            .event
            .as_ref()
            .map(|event| {
                self.convert_event(event).map(DecodeDifferent::Decoded)
            })
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

    fn convert_pallet_storage(&self, storage: &v14::PalletStorageMetadata<PortableForm>) -> Result<v13::StorageMetadata> {
        let entries = storage
            .entries
            .iter()
            .map(|entry| {
                let modifier = match entry.modifier {
                    v14::StorageEntryModifier::Optional => v13::StorageEntryModifier::Optional,
                    v14::StorageEntryModifier::Default => v13::StorageEntryModifier::Default,
                };
                let convert_hasher = |hasher| {
                    match hasher {
                        v14::StorageHasher::Blake2_128 => v13::StorageHasher::Blake2_128,
                        v14::StorageHasher::Blake2_256 => v13::StorageHasher::Blake2_256,
                        v14::StorageHasher::Blake2_128Concat => v13::StorageHasher::Blake2_128Concat,
                        v14::StorageHasher::Twox128 => v13::StorageHasher::Twox128,
                        v14::StorageHasher::Twox256 => v13::StorageHasher::Twox256,
                        v14::StorageHasher::Twox64Concat => v13::StorageHasher::Twox64Concat,
                        v14::StorageHasher::Identity => v13::StorageHasher::Identity,
                    }
                };
                let ty = match entry.ty {
                    v14::StorageEntryType::Plain(key) =>
                        v13::StorageEntryType::Plain(DecodeDifferent::Decoded(self.get_type_ident(key)?)),
                    v14::StorageEntryType::Map { hasher, key, value } => {
                        v13::StorageEntryType::Map {
                            hasher: convert_hasher(hasher),
                            key: self.get_type_ident(key)?,
                            value: self.get_type_ident(value)?,
                            unused: false
                        }
                    }
                    v14::StorageEntryType::DoubleMap { hasher, key1, key2, value, key2_hasher } => {
                        v13::StorageEntryType::DoubleMap {
                            hasher: convert_hasher(hasher),
                            key1: self.get_type_ident(key1)?,
                            key2: self.get_type_ident(key2)?,
                            value: self.get_type_ident(value)?,
                            key2_hasher: convert_hasher(key2_hasher),
                        }
                    }
                    v14::StorageEntryType::NMap { keys, hashers, value } => {
                        let keys_ty = self.resolve_type(keys)?;
                        let keys =
                            match keys_ty.type_def() {
                                TypeDef::Tuple(tuple) => {
                                    tuple.fields().iter().map(|f| self.get_type_ident(*f)).collect::<Result<Vec<_>>>()
                                }
                                td => Err(format!("Expected a tuple type for NMap keys, got {:?}", td))
                            }?;
                        v13::StorageEntryType::NMap {
                            keys: DecodeDifferent::Decoded(keys),
                            hashers: hashers.iter().map(convert_hasher).collect(),
                            value: self.get_type_ident(value)?,
                        }
                    }
                };
                let default = todo!();
                Ok(v13::StorageEntryMetadata {
                    name: DecodeDifferent::Decoded(entry.name),
                    modifier,
                    ty,
                    default,
                    documentation: DecodeDifferent::Decoded(entry.docs)
                })
            })
            .collect();
        Ok(v13::StorageMetadata {
            prefix: DecodeDifferent::Decoded(storage.prefix),
            entries: DecodeDifferent::Decoded(entries)
        })
    }

    fn convert_call(&self, call: &v14::PalletCallMetadata<PortableForm>) -> Result<Vec<v13::FunctionMetadata>> {
        todo!()
    }

    fn convert_event(&self, event: &v14::PalletEventMetadata<PortableForm>) -> Result<Vec<v13::EventMetadata>> {
        todo!()
    }

    fn convert_constant(&self, constant: &v14::PalletConstantMetadata<PortableForm>) -> Result<v13::ModuleConstantMetadata> {
        todo!()
    }

    fn convert_error(&self, error: &v14::PalletErrorMetadata<PortableForm>) -> Result<Vec<v13::ErrorMetadata>> {
        todo!()
    }

    fn resolve_type(&self, ty: <PortableForm as Form>::Type) -> Result<&Type<PortableForm>> {
        self.metadata.types.resolve(ty.id()).ok_or_else(|| format!("Type {} not found", ty.id()))
    }

    fn get_type_ident(&self, ty: <PortableForm as Form>::Type) -> Result<String> {
        let ty = self.resolve_type(ty)?;
        Ok(match ty.type_def() {
            TypeDef::Composite(_) | TypeDef::Variant(_) => ty.ident().unwrap_or_else(|| format!("Type should have an indent")),
            TypeDef::Sequence(_) => todo!(),
            TypeDef::Array(_) => todo!(),
            TypeDef::Tuple(_) => todo!(),
            TypeDef::Primitive(primitive) => {
                let type_str =
                    match primitive {
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
                type_str.to_string()
            }
            TypeDef::Compact(_) => todo!(),
            TypeDef::BitSequence(_) => todo!(),
        })
    }
}


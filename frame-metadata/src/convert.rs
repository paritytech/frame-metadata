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

use core::convert::{TryFrom, TryInto};
use crate::decode_different::{DecodeDifferent, DecodeDifferentArray};
use crate::{v13, PalletMetadata, PalletStorageMetadata, ExtrinsicMetadata};
use crate::v14;

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
        let modules = value.pallets
            .iter()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<_>>>()?;
        let extrinsic = value.extrinsic.try_into()?;
        Ok(v13::RuntimeMetadataV13 {
            modules: DecodeDifferentArray::Decoded(modules),
            extrinsic,
        })
    }

    fn convert_extrinsic(&self, extrinsic: &v14::ExtrinsicMetadata) -> Result<v13::ExtrinsicMetadata> {
        todo!()
    }

    fn convert_pallet(&self, pallet: v14::PalletMetadata) -> Result<v13::ModuleMetadata> {
        let name = pallet.name;
        let storage = pallet.storage
            .map(|s| self.convert_pallet_storage(s)).transpose()?;
        let calls = pallet.calls.map(TryFrom::try_from).transpose()?;
        let event = pallet.event.map(TryFrom::try_from).transpose()?;
        let constants = pallet.constants.map(TryFrom::try_from).transpose()?;
        let errors = pallet.error.map(TryFrom::try_from).transpose()?;
        Ok(v13::ModuleMetadata {
            name: DecodeDifferent::Decoded(name),
            storage,
            calls,
            event,
            constants,
            errors,
            index: value.index,
        })
    }

    fn convert_pallet_storage(&self, storage: v14::PalletStorageMetadata) -> Result<v13::StorageMetadata> {
        todo!()
    }
}



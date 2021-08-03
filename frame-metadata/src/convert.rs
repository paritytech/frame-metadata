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
};

use core::{
    convert::TryFrom,
};
use crate::decode_different::{DecodeDifferent, DecodeDifferentArray};
use crate::{v13, v14};
use scale_info::form::PortableForm;

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
        todo!()
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
}



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

//! Convert the IR to V14 metdata.

use super::{
	ExtrinsicMetadataIR, MetadataIR, PalletCallMetadataIR, PalletConstantMetadataIR,
	PalletErrorMetadataIR, PalletEventMetadataIR, PalletMetadataIR, PalletStorageMetadataIR,
	SignedExtensionMetadataIR, StorageEntryMetadataIR,
};
use crate::{
	v14::{PalletMetadata, PalletStorageMetadata, RuntimeMetadataV14, StorageEntryMetadata},
	ExtrinsicMetadata, PalletCallMetadata, PalletConstantMetadata, PalletErrorMetadata,
	PalletEventMetadata, SignedExtensionMetadata,
};

impl From<MetadataIR> for RuntimeMetadataV14 {
	fn from(ir: MetadataIR) -> Self {
		RuntimeMetadataV14::new(
			ir.pallets.into_iter().map(Into::into).collect(),
			ir.extrinsic.into(),
			ir.ty,
		)
	}
}

impl From<PalletMetadataIR> for PalletMetadata {
	fn from(ir: PalletMetadataIR) -> Self {
		PalletMetadata {
			name: ir.name,
			storage: ir.storage.map(Into::into),
			calls: ir.calls.map(Into::into),
			event: ir.event.map(Into::into),
			constants: ir.constants.into_iter().map(Into::into).collect(),
			error: ir.error.map(Into::into),
			index: ir.index,
			// Note: ir.docs are not part of v14.
		}
	}
}

impl From<StorageEntryMetadataIR> for StorageEntryMetadata {
	fn from(ir: StorageEntryMetadataIR) -> Self {
		StorageEntryMetadata {
			name: ir.name,
			modifier: match ir.modifier {
				super::StorageEntryModifier::Optional => crate::v14::StorageEntryModifier::Optional,
				super::StorageEntryModifier::Default => crate::v14::StorageEntryModifier::Default,
			},
			ty: match ir.ty {
				super::StorageEntryType::Plain(ty) => crate::v14::StorageEntryType::Plain(ty),
				super::StorageEntryType::Map {
					hashers,
					key,
					value,
				} => crate::v14::StorageEntryType::Map {
					hashers: hashers
						.into_iter()
						.map(|hasher| match hasher {
							super::StorageHasher::Blake2_128 => crate::StorageHasher::Blake2_128,
							super::StorageHasher::Blake2_256 => crate::StorageHasher::Blake2_256,
							super::StorageHasher::Blake2_128Concat => {
								crate::StorageHasher::Blake2_128Concat
							}
							super::StorageHasher::Twox128 => crate::StorageHasher::Twox128,
							super::StorageHasher::Twox256 => crate::StorageHasher::Twox256,
							super::StorageHasher::Twox64Concat => {
								crate::StorageHasher::Twox64Concat
							}
							super::StorageHasher::Identity => crate::StorageHasher::Identity,
						})
						.collect(),
					key,
					value,
				},
			},
			default: ir.default,
			docs: ir.docs,
		}
	}
}

impl From<PalletStorageMetadataIR> for PalletStorageMetadata {
	fn from(ir: PalletStorageMetadataIR) -> Self {
		PalletStorageMetadata {
			prefix: ir.prefix,
			entries: ir.entries.into_iter().map(Into::into).collect(),
		}
	}
}

impl From<PalletCallMetadataIR> for PalletCallMetadata {
	fn from(ir: PalletCallMetadataIR) -> Self {
		PalletCallMetadata { ty: ir.ty }
	}
}

impl From<PalletEventMetadataIR> for PalletEventMetadata {
	fn from(ir: PalletEventMetadataIR) -> Self {
		PalletEventMetadata { ty: ir.ty }
	}
}

impl From<PalletConstantMetadataIR> for PalletConstantMetadata {
	fn from(ir: PalletConstantMetadataIR) -> Self {
		PalletConstantMetadata {
			name: ir.name,
			ty: ir.ty,
			value: ir.value,
			docs: ir.docs,
		}
	}
}

impl From<PalletErrorMetadataIR> for PalletErrorMetadata {
	fn from(ir: PalletErrorMetadataIR) -> Self {
		PalletErrorMetadata { ty: ir.ty }
	}
}

impl From<SignedExtensionMetadataIR> for SignedExtensionMetadata {
	fn from(ir: SignedExtensionMetadataIR) -> Self {
		SignedExtensionMetadata {
			identifier: ir.identifier,
			ty: ir.ty,
			additional_signed: ir.additional_signed,
		}
	}
}

impl From<ExtrinsicMetadataIR> for ExtrinsicMetadata {
	fn from(ir: ExtrinsicMetadataIR) -> Self {
		ExtrinsicMetadata {
			ty: ir.ty,
			version: ir.version,
			signed_extensions: ir.signed_extensions.into_iter().map(Into::into).collect(),
		}
	}
}

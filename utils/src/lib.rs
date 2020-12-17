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

#![warn(missing_docs)]

//! Metadata utilities.
//!
//! This crate allows you to parse the latest metadata and use that knowledge
//! to interact with the chain.

use frame_metadata::RuntimeMetadata;

pub mod error;

/// Latest metadata utilities.
pub struct Metadata {
    metadata: frame_metadata::RuntimeMetadataLastVersion,
}

impl Metadata {
    /// Create new metadata utilities from decoded/deserialized metadata.
    pub fn new(metadata: RuntimeMetadata) -> error::Result<Self> {
        let metadata = match metadata {
            RuntimeMetadata::V12(latest) => Ok(latest),
            e => Err(error::Error::InvalidMetadataVersion {
                expected: 12,
                got: Box::new(e)
            }),
        }?;
        Ok(Self {
            metadata,
        })
    }

    /// Read metadata from a JSON-encoded string.
    pub fn from_json_str(json: &str) -> error::Result<Self> {
        let metadata = serde_json::from_str(json)?;
        Self::new(metadata)
    }

    /// Read metadata from SCALE-encoded bytes.
    pub fn from_encoded_bytes(mut bytes: &[u8]) -> error::Result<Self> {
        let metadata = scale::Decode::decode(&mut bytes)?;
        Self::new(metadata)
    }
}

impl Metadata {
    /// List all pallet names in order.
    pub fn pallets(&self) -> Vec<String> {
        self.metadata
            .modules
            .as_slice()
            .iter()
            .map(|pallet| pallet.name.to_string())
            .collect()
    }

    /// Return an index of a pallet if any (case insensitive).
    pub fn pallet_index(&self, name: &str) -> Option<usize> {
        self.pallets()
            .iter()
            .position(|pallet| pallet.eq_ignore_ascii_case(name))
    }
}


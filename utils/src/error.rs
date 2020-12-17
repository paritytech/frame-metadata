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

//! Error for Metadata Utilities.

/// A helper Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Metadata errors.
#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum Error {
    /// Error while decoding metadata from it's SCALE-encoded form.
    #[display(fmt = "Scale: {}", _0)]
    Scale(scale::Error),
    /// Error deserializing metadata from Serde format.
    #[display(fmt = "Serde: {}", _0)]
    Serde(serde_json::Error),
    /// Unexpected version of metadata.
    #[display(fmt = "Invalid version. Expected: {}, Got: {:?}", expected, got)]
    InvalidMetadataVersion {
        /// Numeric value of expected version.
        expected: u32,
        /// Full metadata object that was received.
        got: Box<dyn std::fmt::Debug>,
    },
}

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

//! Convert the IR to specific versions.

#[cfg(feature = "v14")]
use crate::v14::RuntimeMetadataV14;
use crate::{ir::types::MetadataIR, RuntimeMetadataPrefixed};

/// Transform the IR to the specified version.
///
/// Use [`Self::metadata_versions`] to find supported versions.
pub fn to_version(metadata: MetadataIR, version: u32) -> Option<RuntimeMetadataPrefixed> {
	match version {
		#[cfg(feature = "v14")]
		14 => {
			let v14: RuntimeMetadataV14 = metadata.into();
			Some(v14.into())
		}
		_ => None,
	}
}

/// Returns the supported versions of metadata.
pub fn supported_versions() -> Vec<u32> {
	vec![
		#[cfg(feature = "v14")]
		14,
	]
}

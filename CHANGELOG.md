
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [23.0.1] - 2025-12-09

- fix: ensure extrinsic types are collected before pallet types [#110](https://github.com/paritytech/frame-metadata/pull/110)

## [23.0.0] - 2025-05-06

This version stabilizes metadata V16, moving it from being marked as "unstable" and hidden behind that feature flag, to being "current" and having version 16 rather than `u32::MAX`.

## [22.0.0] - 2025-04-24

### Changed

- Alter the deprecation information metadata to remove duplicate/confusing states [#101](https://github.com/paritytech/frame-metadata/pull/101)

## [21.0.0] - 2025-04-01

### Changed

- Address some small niggles in the unstable V16 metadata [#98](https://github.com/paritytech/frame-metadata/pull/98)

## [20.0.0] - 2025-02-20

Metadata version 16 is currently unstable and can be enabled using the unstable feature flag.

### Added

- Add Runtime Api version to v16 [#92](https://github.com/paritytech/frame-metadata/pull/92)

## [19.0.0] - 2025-02-11

Metadata version 16 is currently unstable and can be enabled using the unstable feature flag.

### Added

- v16: Add view functions to the pallets metadata [#89](https://github.com/paritytech/frame-metadata/pull/89)

## [18.0.0] - 2024-11-13

### Changed

- v16: ExtrinsicMetadata extensions [#86](https://github.com/paritytech/frame-metadata/pull/86)

## [17.0.0] - 2024-10-16

### Added

- v16: Add unstable metadata v16 [#82](https://github.com/paritytech/frame-metadata/pull/82)

## [16.0.0] - 2023-06-29

### Changed

- Stabilize V15 metadata [#66](https://github.com/paritytech/frame-metadata/pull/66)

## [15.2.0] - 2023-06-27

### Added

- V15: Add custom values to the metadata  [#61](https://github.com/paritytech/frame-metadata/pull/61)
- v15/metadata: Add outer enum types for calls, events, errors [#57](https://github.com/paritytech/frame-metadata/pull/57)
- Metadata V15: Enrich extrinsic type info for decoding [#56](https://github.com/paritytech/frame-metadata/pull/56)

### Changed

- Simplify feature-flag and use common types [#62](https://github.com/paritytech/frame-metadata/pull/62)
- v15: Rename `error_enum_ty` to `module_error_enum_ty` [#60](https://github.com/paritytech/frame-metadata/pull/60)

## [15.1.0] - 2023-03-30

### Added

- Add metadata V15 with Runtime API support [#48](https://github.com/paritytech/frame-metadata/pull/48)

## [15.0.0] - 2022-02-08

### Changed

- Update edition to 2021
- Update `scale-info` to v2.0.0 and `parity-scale-codec` to v3.0.0 [#35](https://github.com/paritytech/frame-metadata/pull/35)

## [14.2.0] - 2021-11-04

- Add function to return metadata version [#30](https://github.com/paritytech/frame-metadata/pull/30)

## [14.1.0] - 2021-11-03

- Add metadata version v8-v11 [#28](https://github.com/paritytech/frame-metadata/pull/28)
- Combine Map/NMap/DoubleMap StorageEntryTypes [#23](https://github.com/paritytech/frame-metadata/pull/23)

## [14.0.0] - 2021-09-01

## [14.0.0-rc.3] - 2021-08-31

### Added

- Add Runtime type to metadata

## [14.0.0-rc.2] - 2021-08-04

### Changed

Combine Map/NMap/DoubleMap StorageEntryTypes [#23](https://github.com/paritytech/frame-metadata/pull/23)

## [14.0.0-rc.1] - 2021-07-30

### Added

- Metadata V14

pub use mmr::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod mmr {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getChildren"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChildren"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("left"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("right"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLeafIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLeafIndex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("width"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPeakIndexes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPeakIndexes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("width"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("peakIndexes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("width"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hashBranch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hashBranch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("left"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("right"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hashLeaf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hashLeaf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("heightAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("heightAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("height"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("inclusionProof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("inclusionProof"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("root"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("width"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("peaks"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("siblings"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isLeaf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isLeaf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mountainHeight"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mountainHeight"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("size"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numOfPeaks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("numOfPeaks"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("width"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("num"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peakBagging"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("peakBagging"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("width"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("peaks"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peakMapToPeaks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("peakMapToPeaks"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("width"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("peakMap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                        255usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[255]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("peaks"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peakUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("peakUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("width"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevPeakMap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                        255usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[255]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("itemHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextPeakMap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                        255usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[255]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("peaksToPeakMap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("peaksToPeakMap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("width"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("peaks"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("peakMap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                        255usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[255]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rollUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rollUp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("root"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("width"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("peaks"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("itemHashes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MMR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x1Eqa\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x01LW`\x005`\xE0\x1C\x80c\x91\xB4\x19\xAC\x11a\0\xC2W\x80c\xCC\xE3\xC1;\x11a\0\x86W\x80c\xCC\xE3\xC1;\x14a\x03+W\x80c\xDAUo\x92\x14a\x03>W\x80c\xE63N\xDE\x14a\x03aW\x80c\xED\x9FQ\xB3\x14a\x03\x83W\x80c\xF3\x8Bu\x82\x14a\x03\x96W\x80c\xF6\xFD\x88\0\x14a\x03\xA9W`\0\x80\xFD[\x80c\x91\xB4\x19\xAC\x14a\x02\x9FW\x80c\x99}\xB8\x04\x14a\x02\xBFW\x80c\xA2H\xCDO\x14a\x02\xE2W\x80c\xC3e\xC8\xDD\x14a\x02\xF5W\x80c\xC5\"\xE7\xBC\x14a\x03\x19W`\0\x80\xFD[\x80c6a@v\x11a\x01\x14W\x80c6a@v\x14a\x01\xF1W\x80cE*:\x0E\x14a\x02\x04W\x80cI\xAE\x8D\xC3\x14a\x02>W\x80cg\xE8\xF9\x0C\x14a\x02fW\x80cx1~\xF4\x14a\x02yW\x80c\x83\xE2L\x88\x14a\x02\x8CW`\0\x80\xFD[\x80c\x02<#\xDB\x14a\x01QW\x80c$\xC1D9\x14a\x01wW\x80c&\xCF\x02\xAE\x14a\x01\x8CW\x80c-\x12\x94B\x14a\x01\xACW\x80c/..\xA7\x14a\x01\xD1W[`\0\x80\xFD[a\x01da\x01_6`\x04a\x16LV[a\x03\xBCV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01da\x01\x856`\x04a\x16LV[`\x01\x01T\x90V[a\x01\x9Fa\x01\x9A6`\x04a\x17\x18V[a\x03\xDBV[`@Qa\x01n\x91\x90a\x17PV[a\x01\xBFa\x01\xBA6`\x04a\x16LV[a\x05rV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01nV[a\x01\xE4a\x01\xDF6`\x04a\x16LV[a\x05\xB4V[`@Qa\x01n\x91\x90a\x17yV[a\x01da\x01\xFF6`\x04a\x17\xBDV[a\x06\xD8V[a\x01da\x02\x126`\x04a\x17\xE9V[`@\x80Q` \x80\x82\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[a\x02Qa\x02L6`\x04a\x16LV[a\x07\x0FV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01nV[a\x01\xBFa\x02t6`\x04a\x16LV[a\x07\x8BV[a\x01\x9Fa\x02\x876`\x04a\x18\x7FV[a\x07\xF4V[a\x01da\x02\x9A6`\x04a\x16LV[a\x08\xF3V[a\x02\xB2a\x02\xAD6`\x04a\x16LV[a\t)V[`@Qa\x01n\x91\x90a\x19\0V[a\x02\xD2a\x02\xCD6`\x04a\x19\x82V[a\t\xEBV[`@Q\x90\x15\x15\x81R` \x01a\x01nV[a\x01da\x02\xF06`\x04a\x1A%V[a\x0E\x01V[a\x01da\x03\x036`\x04a\x17\xE9V[`\0\x90\x81R`\x03\x91\x90\x91\x01` R`@\x90 T\x90V[a\x01da\x03'6`\x04a\x16LV[T\x90V[a\x02\xD2a\x0396`\x04a\x16LV[a\x0E\xD6V[a\x03Qa\x03L6`\x04a\x17\xE9V[a\x0E\xEEV[`@Qa\x01n\x94\x93\x92\x91\x90a\x1A\x9BV[\x81\x80\x15a\x03mW`\0\x80\xFD[Pa\x03\x81a\x03|6`\x04a\x1A\xCCV[a\x11\x87V[\0[a\x01da\x03\x916`\x04a\x18\x7FV[a\x13KV[a\x01da\x03\xA46`\x04a\x16LV[a\x14\x14V[a\x02\xB2a\x03\xB76`\x04a\x1B\x08V[a\x14LV[`\0a\x03\xC7\x82a\x14\x14V[a\x03\xD5\x90`\x01\x84\x90\x1Ba\x1BLV[\x92\x91PPV[a\x03\xE3a\x16-V[`\0a\x03\xF0\x85`\x01a\x1B_V[\x90P`\0a\x03\xFD\x82a\x08\xF3V[`@\x80Q` \x80\x82\x01\x84\x90R\x81\x83\x01\x88\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90\x91P`\0\x80\x80\x80\x80`\x01[`\xFF\x81\x11a\x05bWa\x04D\x81`\xFFa\x1BLV[\x95P\x81\x15a\x04\x7FW\x8B\x86`\xFF\x81\x10a\x04^Wa\x04^a\x1BrV[` \x02\x01Q\x8A\x87`\xFF\x81\x10a\x04uWa\x04ua\x1BrV[` \x02\x01Ra\x05PV[a\x04\x8A`\x01\x82a\x1BLV[`\x01\x90\x1B\x94P\x8C\x85\x16\x15\x15\x93P\x88\x85\x16\x15\x15\x92P\x87a\x04\xA8\x81a\x1B\x88V[\x98PP\x83\x15a\x04\xD5Wa\x04\xD2\x88\x8D\x88`\xFF\x81\x10a\x04\xC7Wa\x04\xC7a\x1BrV[` \x02\x01Q\x89a\x06\xD8V[\x96P[\x82\x15a\x056W\x83\x15a\x05\x14W\x8B\x86`\xFF\x81\x10a\x04\xF3Wa\x04\xF3a\x1BrV[` \x02\x01Q\x8A\x87`\xFF\x81\x10a\x05\nWa\x05\na\x1BrV[` \x02\x01Ra\x05-V[\x86\x8A\x87`\xFF\x81\x10a\x05'Wa\x05'a\x1BrV[` \x02\x01R[`\x01\x91Pa\x05PV[`\0\x8A\x87`\xFF\x81\x10a\x05JWa\x05Ja\x1BrV[` \x02\x01R[\x80a\x05Z\x81a\x1B\x88V[\x91PPa\x041V[PPPPPPPPP\x93\x92PPPV[`\0`\x01[a\x05\x84`\xFF\x82\x16\x84a\x1B_V[`\x01`\xFF\x83\x16\x1B\x11a\x05\xA2W\x80a\x05\x9A\x81a\x1B\xA1V[\x91PPa\x05wV[a\x05\xAD`\x01\x82a\x1B\xC0V[\x93\x92PPPV[``a\x05\xBF\x82a\x14\x14V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\xD6Wa\x05\xD6a\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xFFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80`\xFF[\x80\x15a\x06{Wa\x06\x19`\x01\x82a\x1BLV[`\x01\x90\x1B\x85\x16\x15a\x06iW`\x01a\x062\x81\x83\x1B\x84a\x1B_V[a\x06<\x91\x90a\x1BLV[\x91P\x81\x84\x84a\x06J\x81a\x1B\x88V[\x95P\x81Q\x81\x10a\x06\\Wa\x06\\a\x1BrV[` \x02` \x01\x01\x81\x81RPP[\x80a\x06s\x81a\x1B\xD9V[\x91PPa\x06\x08V[P\x82Q\x82\x14a\x06\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid bit calculation\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PP\x91\x90PV[`@\x80Q` \x80\x82\x01\x95\x90\x95R\x80\x82\x01\x93\x90\x93R``\x80\x84\x01\x92\x90\x92R\x80Q\x80\x84\x03\x90\x92\x01\x82R`\x80\x90\x92\x01\x90\x91R\x80Q\x91\x01 \x90V[`\0\x80`\x01a\x07\x1D\x84a\x07\x8BV[a\x07'\x91\x90a\x1B\xC0V[`\xFF\x16`\x01\x90\x1B\x83a\x079\x91\x90a\x1BLV[\x91Pa\x07F`\x01\x84a\x1BLV[\x90P\x80\x82\x03a\x07\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x13\x9B\xDD\x08\x18H\x1C\x18\\\x99[\x9D`\xA2\x1B`D\x82\x01R`d\x01a\x06\xC8V[\x91P\x91V[`\0\x81\x81[\x80\x82\x11\x15a\x07\xD8Wa\x07\xA9`\x01`\xFF\x85\x16\x81\x90\x1Ba\x1BLV[a\x07\xB3\x90\x83a\x1BLV[\x91Pa\x07\xBE\x82a\x05rV[\x92Pa\x07\xD1`\x01`\xFF\x85\x16\x81\x90\x1Ba\x1BLV[\x90Pa\x07\x90V[a\x07\xE2\x82\x82a\x1BLV[a\x07\xEC\x90\x84a\x1B\xC0V[\x94\x93PPPPV[a\x07\xFCa\x16-V[\x81Q`\0\x90\x81\x90`\x01[`\xFF\x81\x11a\x08\xA1Wa\x08\x19\x81`\xFFa\x1BLV[\x93Pa\x08&`\x01\x82a\x1BLV[`\x01\x90\x1B\x92P\x86\x83\x16\x15a\x08uW\x85a\x08>\x83a\x1B\xD9V[\x92P\x82\x81Q\x81\x10a\x08QWa\x08Qa\x1BrV[` \x02` \x01\x01Q\x85\x85`\xFF\x81\x10a\x08kWa\x08ka\x1BrV[` \x02\x01Ra\x08\x8FV[`\0\x85\x85`\xFF\x81\x10a\x08\x89Wa\x08\x89a\x1BrV[` \x02\x01R[\x80a\x08\x99\x81a\x1B\x88V[\x91PPa\x08\x06V[P\x80\x15a\x08\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01RvInvalid number of peaks`H\x1B`D\x82\x01R`d\x01a\x06\xC8V[PPP\x92\x91PPV[`\0a\t\0`\x02\x83a\x1B\xF0V[`\x01\x03a\t\x10Wa\x03\xD5\x82a\x03\xBCV[a\t\x1Ea\x01_`\x01\x84a\x1BLV[a\x03\xD5\x90`\x01a\x1B_V[```\0a\t:\x83`\x02\x01Ta\x05\xB4V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\tUWa\tUa\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t~W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81Q\x81\x10\x15a\x06\xD1W\x83`\x03\x01`\0\x83\x83\x81Q\x81\x10a\t\xA5Wa\t\xA5a\x1BrV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 T\x83\x82\x81Q\x81\x10a\t\xCEWa\t\xCEa\x1BrV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\t\xE3\x81a\x1B\x88V[\x91PPa\t\x84V[`\0\x80a\t\xF7\x87a\x03\xBCV[\x90P\x85\x81\x10\x15a\nAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIndex is out of range`X\x1B`D\x82\x01R`d\x01a\x06\xC8V[\x80\x81\x85`@Q` \x01a\nU\x92\x91\x90a\x1C\x12V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\x85\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x88\x14a\n\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid root hash from the peaks`D\x82\x01R`d\x01a\x06\xC8V[`\0\x80`\0a\n\xF7\x8Aa\x05\xB4V[\x90P`\0[\x81Q\x81\x10\x15a\x0BtW\x89\x82\x82\x81Q\x81\x10a\x0B\x18Wa\x0B\x18a\x1BrV[` \x02` \x01\x01Q\x10a\x0BbW\x87\x81\x81Q\x81\x10a\x0B7Wa\x0B7a\x1BrV[` \x02` \x01\x01Q\x92P\x81\x81\x81Q\x81\x10a\x0BSWa\x0BSa\x1BrV[` \x02` \x01\x01Q\x93Pa\x0BtV[\x80a\x0Bl\x81a\x1B\x88V[\x91PPa\n\xFCV[P\x81a\x0B\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x18\\\x99\xD9]\x08\x1A\\\xC8\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`j\x1B`D\x82\x01R`d\x01a\x06\xC8V[`\0\x86Q`\x01a\x0B\xC8\x91\x90a\x1B_V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xDFWa\x0B\xDFa\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x08W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80`\0\x89Q`\x01a\x0C\x1E\x91\x90a\x1CMV[\x90P[`\xFF\x81\x16\x15a\x0C\x81W\x86\x84a\x0C5\x83a\x1CfV[\x92P\x82`\xFF\x16\x81Q\x81\x10a\x0CKWa\x0CKa\x1BrV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x86\x8D\x14a\x0C\x81Wa\x0Cf\x87a\x07\x0FV[\x90\x93P\x91P\x82\x8D\x11a\x0CxW\x82a\x0CzV[\x81[\x96Pa\x0C!V[`\0[\x84Q\x82`\xFF\x16\x10\x15a\r\xA3W\x84\x82`\xFF\x16\x81Q\x81\x10a\x0C\xA5Wa\x0C\xA5a\x1BrV[` \x02` \x01\x01Q\x97P\x81`\xFF\x16`\0\x03a\x0C\xF3W\x8CQ` \x80\x8F\x01\x91\x90\x91 `@\x80Q\x80\x84\x01\x8C\x90R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90R\x80Q\x91\x01 [\x90Pa\r\x91V[\x84a\x0C\xFF`\x01\x84a\x1B\xC0V[`\xFF\x16\x81Q\x81\x10a\r\x12Wa\r\x12a\x1BrV[` \x02` \x01\x01Q`\x01\x89a\r'\x91\x90a\x1BLV[\x03a\r]Wa\x0C\xEC\x88\x8Ca\r<`\x01\x86a\x1B\xC0V[`\xFF\x16\x81Q\x81\x10a\rOWa\rOa\x1BrV[` \x02` \x01\x01Q\x83a\x06\xD8V[a\r\x8E\x88\x82\x8Da\rn`\x01\x87a\x1B\xC0V[`\xFF\x16\x81Q\x81\x10a\r\x81Wa\r\x81a\x1BrV[` \x02` \x01\x01Qa\x06\xD8V[\x90P[\x81a\r\x9B\x81a\x1B\xA1V[\x92PPa\x0C\x84V[\x86\x81\x14a\r\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x12\x18\\\xDA\x19Y\x08\x1C\x19XZ\xC8\x1A\\\xC8\x1A[\x9D\x98[\x1AY`R\x1B`D\x82\x01R`d\x01a\x06\xC8V[P`\x01\x9F\x9EPPPPPPPPPPPPPPPV[`\0a\x0E\r\x84\x84a\x13KV[\x85\x14a\x0E[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid root hash from the peaks`D\x82\x01R`d\x01a\x06\xC8V[\x83`\0a\x0Eh\x82\x86a\x07\xF4V[\x90P`\0[\x84Q\x81\x10\x15a\x0E\xBCWa\x0E\x9A\x83\x83\x87\x84\x81Q\x81\x10a\x0E\x8DWa\x0E\x8Da\x1BrV[` \x02` \x01\x01Qa\x03\xDBV[\x91P\x82a\x0E\xA6\x81a\x1B\x88V[\x93PP\x80\x80a\x0E\xB4\x90a\x1B\x88V[\x91PPa\x0EmV[Pa\x0E\xCB\x82a\x03\x91\x84\x84a\x14LV[\x97\x96PPPPPPPV[`\0a\x0E\xE1\x82a\x07\x8BV[`\xFF\x16`\x01\x14\x90P\x91\x90PV[`\0\x80``\x80\x85`\x01\x01T\x85\x10a\x0F6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkOut of range`\xA0\x1B`D\x82\x01R`d\x01a\x06\xC8V[a\x0F?\x85a\x0E\xD6V[a\x0FxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri'7\xBA\x100\x9062\xB0\xB3`\xB1\x1B`D\x82\x01R`d\x01a\x06\xC8V[\x85T`\x02\x87\x01T\x90\x94P\x92P`\0a\x0F\x8F\x84a\x05\xB4V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xAAWa\x0F\xAAa\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\xD3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0\x80[\x82Q\x81\x10\x15a\x10\x8BW\x88`\x03\x01`\0\x84\x83\x81Q\x81\x10a\x0F\xFBWa\x0F\xFBa\x1BrV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 T\x85\x82\x81Q\x81\x10a\x10$Wa\x10$a\x1BrV[` \x02` \x01\x01\x81\x81RPP\x87\x83\x82\x81Q\x81\x10a\x10CWa\x10Ca\x1BrV[` \x02` \x01\x01Q\x10\x15\x80\x15a\x10WWP\x81\x15[\x15a\x10yW\x82\x81\x81Q\x81\x10a\x10nWa\x10na\x1BrV[` \x02` \x01\x01Q\x91P[\x80a\x10\x83\x81a\x1B\x88V[\x91PPa\x0F\xDAV[P`\0\x80`\0a\x10\x9A\x84a\x07\x8BV[\x90Pa\x10\xA7`\x01\x82a\x1B\xC0V[`\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xC1Wa\x10\xC1a\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xEAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x95P[\x89\x84\x14a\x11yW\x80a\x10\xFF\x81a\x1CfV[\x91PPa\x11\x0B\x84a\x07\x0FV[\x90\x93P\x91P\x82\x8A\x11\x15a\x11\x1EW\x81a\x11 V[\x82[\x93P\x8A`\x03\x01`\0\x84\x8C\x11\x15a\x116W\x84a\x118V[\x83[\x81R` \x01\x90\x81R` \x01`\0 T\x86`\x01\x83a\x11U\x91\x90a\x1B\xC0V[`\xFF\x16\x81Q\x81\x10a\x11hWa\x11ha\x1BrV[` \x02` \x01\x01\x81\x81RPPa\x10\xEEV[PPPPP\x92\x95\x91\x94P\x92PV[`\0\x81\x80Q\x90` \x01 \x90P\x80\x83`\x04\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Qa\x11\xB6\x91\x90a\x1C\xB7V[`@Q\x80\x91\x03\x90 \x14a\x11\xDFW`\0\x81\x81R`\x04\x84\x01` R`@\x90 a\x11\xDD\x83\x82a\x1D|V[P[`\0a\x12!\x84`\x01\x01T`\x01a\x11\xF5\x91\x90a\x1B_V[`@\x80Q` \x80\x82\x01\x93\x90\x93R\x80\x82\x01\x86\x90R\x81Q\x80\x82\x03\x83\x01\x81R``\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[\x90P\x80\x84`\x03\x01`\0\x86`\x01\x01T`\x01a\x12;\x91\x90a\x1B_V[\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x84`\x02\x01`\0\x82\x82Ta\x12b\x91\x90a\x1B_V[\x92PP\x81\x90UP`\0a\x12x\x85`\x02\x01Ta\x05\xB4V[\x90Pa\x12\x87\x85`\x02\x01Ta\x03\xBCV[`\x01\x86\x01U\x80Q`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xA8Wa\x12\xA8a\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xD1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x132Wa\x13\x03\x87\x84\x83\x81Q\x81\x10a\x12\xF6Wa\x12\xF6a\x1BrV[` \x02` \x01\x01Qa\x15oV[\x82\x82\x81Q\x81\x10a\x13\x15Wa\x13\x15a\x1BrV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x13*\x81a\x1B\x88V[\x91PPa\x12\xD7V[Pa\x13A\x86`\x02\x01T\x82a\x13KV[\x90\x95UPPPPPV[`\0\x80a\x13W\x84a\x03\xBCV[\x90P\x82Qa\x13d\x85a\x14\x14V[\x14a\x13\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FReceived invalid number of peaks`D\x82\x01R`d\x01a\x06\xC8V[\x80\x81\x84`@Q` \x01a\x13\xC5\x92\x91\x90a\x1C\x12V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\xF5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x92\x91PPV[`\0\x81[\x80\x15a\x14FWa\x14)`\x02\x82a\x1B\xF0V[`\x01\x03a\x14>W\x81a\x14:\x81a\x1B\x88V[\x92PP[`\x01\x1Ca\x14\x18V[P\x91\x90PV[```\0a\x14Y\x84a\x14\x14V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14sWa\x14sa\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\x9CW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0\x80[`\xFF\x81\x10\x15a\x15\x1DW`\0\x85\x82`\xFF\x81\x10a\x14\xC0Wa\x14\xC0a\x1BrV[` \x02\x01Q\x14a\x15\x0BW\x84\x81`\xFF\x81\x10a\x14\xDCWa\x14\xDCa\x1BrV[` \x02\x01Q\x84\x83a\x14\xEC\x81a\x1B\x88V[\x94P\x81Q\x81\x10a\x14\xFEWa\x14\xFEa\x1BrV[` \x02` \x01\x01\x81\x81RPP[\x80a\x15\x15\x81a\x1B\x88V[\x91PPa\x14\xA3V[P\x81\x81\x14a\x15gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01RvInvalid number of peaks`H\x1B`D\x82\x01R`d\x01a\x06\xC8V[PP\x92\x91PPV[`\0\x82`\x01\x01T\x82\x11\x15a\x15\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkOut of range`\xA0\x1B`D\x82\x01R`d\x01a\x06\xC8V[`\0\x82\x81R`\x03\x84\x01` R`@\x90 Ta\x16\x16W`\0\x80a\x15\xD5\x84a\x07\x0FV[\x91P\x91P`\0a\x15\xE5\x86\x84a\x15oV[\x90P`\0a\x15\xF3\x87\x84a\x15oV[\x90Pa\x16\0\x86\x83\x83a\x06\xD8V[`\0\x87\x81R`\x03\x89\x01` R`@\x90 UPPPP[P`\0\x90\x81R`\x03\x91\x90\x91\x01` R`@\x90 T\x90V[`@Q\x80a\x1F\xE0\x01`@R\x80`\xFF\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x16^W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x16\xA3Wa\x16\xA3a\x16eV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x16\xBCW`\0\x80\xFD[`@Qa\x1F\xE0\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x16\xE0Wa\x16\xE0a\x16eV[`@R\x83\x01\x81\x85\x82\x11\x15a\x16\xF3W`\0\x80\xFD[\x84[\x82\x81\x10\x15a\x17\rW\x805\x82R` \x91\x82\x01\x91\x01a\x16\xF5V[P\x91\x95\x94PPPPPV[`\0\x80`\0a  \x84\x86\x03\x12\x15a\x17.W`\0\x80\xFD[\x835\x92Pa\x17?\x85` \x86\x01a\x16\xABV[\x91Pa \0\x84\x015\x90P\x92P\x92P\x92V[a\x1F\xE0\x81\x01\x81\x83`\0[`\xFF\x81\x10\x15a\x08\xEAW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x17ZV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x17\xB1W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x17\x95V[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17\xD2W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xFCW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x82`\x1F\x83\x01\x12a\x18\x1CW`\0\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x82\x11\x15a\x187Wa\x187a\x16eV[\x81`\x05\x1Ba\x18F\x82\x82\x01a\x16{V[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x18`W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x0E\xCBW\x825\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x18fV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\x92W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xAFW`\0\x80\xFD[a\x18\xBB\x85\x82\x86\x01a\x18\x0BV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x18\xF5W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x18\xD9V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x05\xAD` \x83\x01\x84a\x18\xC5V[`\0\x82`\x1F\x83\x01\x12a\x19$W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19=Wa\x19=a\x16eV[a\x19P`\x1F\x82\x01`\x1F\x19\x16` \x01a\x16{V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x19eW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x19\x9BW`\0\x80\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x19\xC7W`\0\x80\xFD[a\x19\xD3\x8A\x83\x8B\x01a\x19\x13V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15a\x19\xE9W`\0\x80\xFD[a\x19\xF5\x8A\x83\x8B\x01a\x18\x0BV[\x93P`\xA0\x89\x015\x91P\x80\x82\x11\x15a\x1A\x0BW`\0\x80\xFD[Pa\x1A\x18\x89\x82\x8A\x01a\x18\x0BV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1A;W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1A`W`\0\x80\xFD[a\x1Al\x88\x83\x89\x01a\x18\x0BV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x1A\x82W`\0\x80\xFD[Pa\x1A\x8F\x87\x82\x88\x01a\x18\x0BV[\x91PP\x92\x95\x91\x94P\x92PV[\x84\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0a\x1A\xBA`\x80\x83\x01\x85a\x18\xC5V[\x82\x81\x03``\x84\x01Ra\x0E\xCB\x81\x85a\x18\xC5V[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xDFW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xFCW`\0\x80\xFD[a\x18\xBB\x85\x82\x86\x01a\x19\x13V[`\0\x80a \0\x83\x85\x03\x12\x15a\x1B\x1CW`\0\x80\xFD[\x825\x91Pa\x1B-\x84` \x85\x01a\x16\xABV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xD5Wa\x03\xD5a\x1B6V[\x80\x82\x01\x80\x82\x11\x15a\x03\xD5Wa\x03\xD5a\x1B6V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1B\x9AWa\x1B\x9Aa\x1B6V[P`\x01\x01\x90V[`\0`\xFF\x82\x16`\xFF\x81\x03a\x1B\xB7Wa\x1B\xB7a\x1B6V[`\x01\x01\x92\x91PPV[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x03\xD5Wa\x03\xD5a\x1B6V[`\0\x81a\x1B\xE8Wa\x1B\xE8a\x1B6V[P`\0\x19\x01\x90V[`\0\x82a\x1C\rWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x82\x81R`\0` \x80\x83\x01\x84Q\x82\x86\x01`\0[\x82\x81\x10\x15a\x1C@W\x81Q\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x1C$V[P\x91\x97\x96PPPPPPPV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x03\xD5Wa\x03\xD5a\x1B6V[`\0`\xFF\x82\x16\x80a\x1CyWa\x1Cya\x1B6V[`\0\x19\x01\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1C\x97W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x14FWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x80\x83Ta\x1C\xC5\x81a\x1C\x83V[`\x01\x82\x81\x16\x80\x15a\x1C\xDDW`\x01\x81\x14a\x1C\xF2Wa\x1D!V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x1D!V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x1D\x18W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x1C\xFFV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\x1F\x82\x11\x15a\x1DwW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x1DTWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1DsW\x82\x81U`\x01\x01a\x1D`V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x95Wa\x1D\x95a\x16eV[a\x1D\xA9\x81a\x1D\xA3\x84Ta\x1C\x83V[\x84a\x1D-V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1D\xDEW`\0\x84\x15a\x1D\xC6WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1DsV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x1E\rW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1D\xEEV[P\x85\x82\x10\x15a\x1E+W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \x98\xA7\xA3w|\xCE\xCAv\x8BI\xB6\xDD\r\xD8\xFD\x02h\x8F?{]'\xA9\x01w\xC3\xCB\xC6c\xF4\x80\x03dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static MMR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x01LW`\x005`\xE0\x1C\x80c\x91\xB4\x19\xAC\x11a\0\xC2W\x80c\xCC\xE3\xC1;\x11a\0\x86W\x80c\xCC\xE3\xC1;\x14a\x03+W\x80c\xDAUo\x92\x14a\x03>W\x80c\xE63N\xDE\x14a\x03aW\x80c\xED\x9FQ\xB3\x14a\x03\x83W\x80c\xF3\x8Bu\x82\x14a\x03\x96W\x80c\xF6\xFD\x88\0\x14a\x03\xA9W`\0\x80\xFD[\x80c\x91\xB4\x19\xAC\x14a\x02\x9FW\x80c\x99}\xB8\x04\x14a\x02\xBFW\x80c\xA2H\xCDO\x14a\x02\xE2W\x80c\xC3e\xC8\xDD\x14a\x02\xF5W\x80c\xC5\"\xE7\xBC\x14a\x03\x19W`\0\x80\xFD[\x80c6a@v\x11a\x01\x14W\x80c6a@v\x14a\x01\xF1W\x80cE*:\x0E\x14a\x02\x04W\x80cI\xAE\x8D\xC3\x14a\x02>W\x80cg\xE8\xF9\x0C\x14a\x02fW\x80cx1~\xF4\x14a\x02yW\x80c\x83\xE2L\x88\x14a\x02\x8CW`\0\x80\xFD[\x80c\x02<#\xDB\x14a\x01QW\x80c$\xC1D9\x14a\x01wW\x80c&\xCF\x02\xAE\x14a\x01\x8CW\x80c-\x12\x94B\x14a\x01\xACW\x80c/..\xA7\x14a\x01\xD1W[`\0\x80\xFD[a\x01da\x01_6`\x04a\x16LV[a\x03\xBCV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01da\x01\x856`\x04a\x16LV[`\x01\x01T\x90V[a\x01\x9Fa\x01\x9A6`\x04a\x17\x18V[a\x03\xDBV[`@Qa\x01n\x91\x90a\x17PV[a\x01\xBFa\x01\xBA6`\x04a\x16LV[a\x05rV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01nV[a\x01\xE4a\x01\xDF6`\x04a\x16LV[a\x05\xB4V[`@Qa\x01n\x91\x90a\x17yV[a\x01da\x01\xFF6`\x04a\x17\xBDV[a\x06\xD8V[a\x01da\x02\x126`\x04a\x17\xE9V[`@\x80Q` \x80\x82\x01\x94\x90\x94R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90R\x80Q\x91\x01 \x90V[a\x02Qa\x02L6`\x04a\x16LV[a\x07\x0FV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01nV[a\x01\xBFa\x02t6`\x04a\x16LV[a\x07\x8BV[a\x01\x9Fa\x02\x876`\x04a\x18\x7FV[a\x07\xF4V[a\x01da\x02\x9A6`\x04a\x16LV[a\x08\xF3V[a\x02\xB2a\x02\xAD6`\x04a\x16LV[a\t)V[`@Qa\x01n\x91\x90a\x19\0V[a\x02\xD2a\x02\xCD6`\x04a\x19\x82V[a\t\xEBV[`@Q\x90\x15\x15\x81R` \x01a\x01nV[a\x01da\x02\xF06`\x04a\x1A%V[a\x0E\x01V[a\x01da\x03\x036`\x04a\x17\xE9V[`\0\x90\x81R`\x03\x91\x90\x91\x01` R`@\x90 T\x90V[a\x01da\x03'6`\x04a\x16LV[T\x90V[a\x02\xD2a\x0396`\x04a\x16LV[a\x0E\xD6V[a\x03Qa\x03L6`\x04a\x17\xE9V[a\x0E\xEEV[`@Qa\x01n\x94\x93\x92\x91\x90a\x1A\x9BV[\x81\x80\x15a\x03mW`\0\x80\xFD[Pa\x03\x81a\x03|6`\x04a\x1A\xCCV[a\x11\x87V[\0[a\x01da\x03\x916`\x04a\x18\x7FV[a\x13KV[a\x01da\x03\xA46`\x04a\x16LV[a\x14\x14V[a\x02\xB2a\x03\xB76`\x04a\x1B\x08V[a\x14LV[`\0a\x03\xC7\x82a\x14\x14V[a\x03\xD5\x90`\x01\x84\x90\x1Ba\x1BLV[\x92\x91PPV[a\x03\xE3a\x16-V[`\0a\x03\xF0\x85`\x01a\x1B_V[\x90P`\0a\x03\xFD\x82a\x08\xF3V[`@\x80Q` \x80\x82\x01\x84\x90R\x81\x83\x01\x88\x90R\x82Q\x80\x83\x03\x84\x01\x81R``\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90\x91P`\0\x80\x80\x80\x80`\x01[`\xFF\x81\x11a\x05bWa\x04D\x81`\xFFa\x1BLV[\x95P\x81\x15a\x04\x7FW\x8B\x86`\xFF\x81\x10a\x04^Wa\x04^a\x1BrV[` \x02\x01Q\x8A\x87`\xFF\x81\x10a\x04uWa\x04ua\x1BrV[` \x02\x01Ra\x05PV[a\x04\x8A`\x01\x82a\x1BLV[`\x01\x90\x1B\x94P\x8C\x85\x16\x15\x15\x93P\x88\x85\x16\x15\x15\x92P\x87a\x04\xA8\x81a\x1B\x88V[\x98PP\x83\x15a\x04\xD5Wa\x04\xD2\x88\x8D\x88`\xFF\x81\x10a\x04\xC7Wa\x04\xC7a\x1BrV[` \x02\x01Q\x89a\x06\xD8V[\x96P[\x82\x15a\x056W\x83\x15a\x05\x14W\x8B\x86`\xFF\x81\x10a\x04\xF3Wa\x04\xF3a\x1BrV[` \x02\x01Q\x8A\x87`\xFF\x81\x10a\x05\nWa\x05\na\x1BrV[` \x02\x01Ra\x05-V[\x86\x8A\x87`\xFF\x81\x10a\x05'Wa\x05'a\x1BrV[` \x02\x01R[`\x01\x91Pa\x05PV[`\0\x8A\x87`\xFF\x81\x10a\x05JWa\x05Ja\x1BrV[` \x02\x01R[\x80a\x05Z\x81a\x1B\x88V[\x91PPa\x041V[PPPPPPPPP\x93\x92PPPV[`\0`\x01[a\x05\x84`\xFF\x82\x16\x84a\x1B_V[`\x01`\xFF\x83\x16\x1B\x11a\x05\xA2W\x80a\x05\x9A\x81a\x1B\xA1V[\x91PPa\x05wV[a\x05\xAD`\x01\x82a\x1B\xC0V[\x93\x92PPPV[``a\x05\xBF\x82a\x14\x14V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\xD6Wa\x05\xD6a\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xFFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80`\xFF[\x80\x15a\x06{Wa\x06\x19`\x01\x82a\x1BLV[`\x01\x90\x1B\x85\x16\x15a\x06iW`\x01a\x062\x81\x83\x1B\x84a\x1B_V[a\x06<\x91\x90a\x1BLV[\x91P\x81\x84\x84a\x06J\x81a\x1B\x88V[\x95P\x81Q\x81\x10a\x06\\Wa\x06\\a\x1BrV[` \x02` \x01\x01\x81\x81RPP[\x80a\x06s\x81a\x1B\xD9V[\x91PPa\x06\x08V[P\x82Q\x82\x14a\x06\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid bit calculation\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PP\x91\x90PV[`@\x80Q` \x80\x82\x01\x95\x90\x95R\x80\x82\x01\x93\x90\x93R``\x80\x84\x01\x92\x90\x92R\x80Q\x80\x84\x03\x90\x92\x01\x82R`\x80\x90\x92\x01\x90\x91R\x80Q\x91\x01 \x90V[`\0\x80`\x01a\x07\x1D\x84a\x07\x8BV[a\x07'\x91\x90a\x1B\xC0V[`\xFF\x16`\x01\x90\x1B\x83a\x079\x91\x90a\x1BLV[\x91Pa\x07F`\x01\x84a\x1BLV[\x90P\x80\x82\x03a\x07\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x13\x9B\xDD\x08\x18H\x1C\x18\\\x99[\x9D`\xA2\x1B`D\x82\x01R`d\x01a\x06\xC8V[\x91P\x91V[`\0\x81\x81[\x80\x82\x11\x15a\x07\xD8Wa\x07\xA9`\x01`\xFF\x85\x16\x81\x90\x1Ba\x1BLV[a\x07\xB3\x90\x83a\x1BLV[\x91Pa\x07\xBE\x82a\x05rV[\x92Pa\x07\xD1`\x01`\xFF\x85\x16\x81\x90\x1Ba\x1BLV[\x90Pa\x07\x90V[a\x07\xE2\x82\x82a\x1BLV[a\x07\xEC\x90\x84a\x1B\xC0V[\x94\x93PPPPV[a\x07\xFCa\x16-V[\x81Q`\0\x90\x81\x90`\x01[`\xFF\x81\x11a\x08\xA1Wa\x08\x19\x81`\xFFa\x1BLV[\x93Pa\x08&`\x01\x82a\x1BLV[`\x01\x90\x1B\x92P\x86\x83\x16\x15a\x08uW\x85a\x08>\x83a\x1B\xD9V[\x92P\x82\x81Q\x81\x10a\x08QWa\x08Qa\x1BrV[` \x02` \x01\x01Q\x85\x85`\xFF\x81\x10a\x08kWa\x08ka\x1BrV[` \x02\x01Ra\x08\x8FV[`\0\x85\x85`\xFF\x81\x10a\x08\x89Wa\x08\x89a\x1BrV[` \x02\x01R[\x80a\x08\x99\x81a\x1B\x88V[\x91PPa\x08\x06V[P\x80\x15a\x08\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01RvInvalid number of peaks`H\x1B`D\x82\x01R`d\x01a\x06\xC8V[PPP\x92\x91PPV[`\0a\t\0`\x02\x83a\x1B\xF0V[`\x01\x03a\t\x10Wa\x03\xD5\x82a\x03\xBCV[a\t\x1Ea\x01_`\x01\x84a\x1BLV[a\x03\xD5\x90`\x01a\x1B_V[```\0a\t:\x83`\x02\x01Ta\x05\xB4V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\tUWa\tUa\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t~W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81Q\x81\x10\x15a\x06\xD1W\x83`\x03\x01`\0\x83\x83\x81Q\x81\x10a\t\xA5Wa\t\xA5a\x1BrV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 T\x83\x82\x81Q\x81\x10a\t\xCEWa\t\xCEa\x1BrV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\t\xE3\x81a\x1B\x88V[\x91PPa\t\x84V[`\0\x80a\t\xF7\x87a\x03\xBCV[\x90P\x85\x81\x10\x15a\nAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIndex is out of range`X\x1B`D\x82\x01R`d\x01a\x06\xC8V[\x80\x81\x85`@Q` \x01a\nU\x92\x91\x90a\x1C\x12V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\x85\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x88\x14a\n\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid root hash from the peaks`D\x82\x01R`d\x01a\x06\xC8V[`\0\x80`\0a\n\xF7\x8Aa\x05\xB4V[\x90P`\0[\x81Q\x81\x10\x15a\x0BtW\x89\x82\x82\x81Q\x81\x10a\x0B\x18Wa\x0B\x18a\x1BrV[` \x02` \x01\x01Q\x10a\x0BbW\x87\x81\x81Q\x81\x10a\x0B7Wa\x0B7a\x1BrV[` \x02` \x01\x01Q\x92P\x81\x81\x81Q\x81\x10a\x0BSWa\x0BSa\x1BrV[` \x02` \x01\x01Q\x93Pa\x0BtV[\x80a\x0Bl\x81a\x1B\x88V[\x91PPa\n\xFCV[P\x81a\x0B\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x15\x18\\\x99\xD9]\x08\x1A\\\xC8\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`j\x1B`D\x82\x01R`d\x01a\x06\xC8V[`\0\x86Q`\x01a\x0B\xC8\x91\x90a\x1B_V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xDFWa\x0B\xDFa\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x08W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80`\0\x89Q`\x01a\x0C\x1E\x91\x90a\x1CMV[\x90P[`\xFF\x81\x16\x15a\x0C\x81W\x86\x84a\x0C5\x83a\x1CfV[\x92P\x82`\xFF\x16\x81Q\x81\x10a\x0CKWa\x0CKa\x1BrV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x86\x8D\x14a\x0C\x81Wa\x0Cf\x87a\x07\x0FV[\x90\x93P\x91P\x82\x8D\x11a\x0CxW\x82a\x0CzV[\x81[\x96Pa\x0C!V[`\0[\x84Q\x82`\xFF\x16\x10\x15a\r\xA3W\x84\x82`\xFF\x16\x81Q\x81\x10a\x0C\xA5Wa\x0C\xA5a\x1BrV[` \x02` \x01\x01Q\x97P\x81`\xFF\x16`\0\x03a\x0C\xF3W\x8CQ` \x80\x8F\x01\x91\x90\x91 `@\x80Q\x80\x84\x01\x8C\x90R\x80\x82\x01\x92\x90\x92R\x80Q\x80\x83\x03\x82\x01\x81R``\x90\x92\x01\x90R\x80Q\x91\x01 [\x90Pa\r\x91V[\x84a\x0C\xFF`\x01\x84a\x1B\xC0V[`\xFF\x16\x81Q\x81\x10a\r\x12Wa\r\x12a\x1BrV[` \x02` \x01\x01Q`\x01\x89a\r'\x91\x90a\x1BLV[\x03a\r]Wa\x0C\xEC\x88\x8Ca\r<`\x01\x86a\x1B\xC0V[`\xFF\x16\x81Q\x81\x10a\rOWa\rOa\x1BrV[` \x02` \x01\x01Q\x83a\x06\xD8V[a\r\x8E\x88\x82\x8Da\rn`\x01\x87a\x1B\xC0V[`\xFF\x16\x81Q\x81\x10a\r\x81Wa\r\x81a\x1BrV[` \x02` \x01\x01Qa\x06\xD8V[\x90P[\x81a\r\x9B\x81a\x1B\xA1V[\x92PPa\x0C\x84V[\x86\x81\x14a\r\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x12\x18\\\xDA\x19Y\x08\x1C\x19XZ\xC8\x1A\\\xC8\x1A[\x9D\x98[\x1AY`R\x1B`D\x82\x01R`d\x01a\x06\xC8V[P`\x01\x9F\x9EPPPPPPPPPPPPPPPV[`\0a\x0E\r\x84\x84a\x13KV[\x85\x14a\x0E[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid root hash from the peaks`D\x82\x01R`d\x01a\x06\xC8V[\x83`\0a\x0Eh\x82\x86a\x07\xF4V[\x90P`\0[\x84Q\x81\x10\x15a\x0E\xBCWa\x0E\x9A\x83\x83\x87\x84\x81Q\x81\x10a\x0E\x8DWa\x0E\x8Da\x1BrV[` \x02` \x01\x01Qa\x03\xDBV[\x91P\x82a\x0E\xA6\x81a\x1B\x88V[\x93PP\x80\x80a\x0E\xB4\x90a\x1B\x88V[\x91PPa\x0EmV[Pa\x0E\xCB\x82a\x03\x91\x84\x84a\x14LV[\x97\x96PPPPPPPV[`\0a\x0E\xE1\x82a\x07\x8BV[`\xFF\x16`\x01\x14\x90P\x91\x90PV[`\0\x80``\x80\x85`\x01\x01T\x85\x10a\x0F6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkOut of range`\xA0\x1B`D\x82\x01R`d\x01a\x06\xC8V[a\x0F?\x85a\x0E\xD6V[a\x0FxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri'7\xBA\x100\x9062\xB0\xB3`\xB1\x1B`D\x82\x01R`d\x01a\x06\xC8V[\x85T`\x02\x87\x01T\x90\x94P\x92P`\0a\x0F\x8F\x84a\x05\xB4V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xAAWa\x0F\xAAa\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\xD3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0\x80[\x82Q\x81\x10\x15a\x10\x8BW\x88`\x03\x01`\0\x84\x83\x81Q\x81\x10a\x0F\xFBWa\x0F\xFBa\x1BrV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 T\x85\x82\x81Q\x81\x10a\x10$Wa\x10$a\x1BrV[` \x02` \x01\x01\x81\x81RPP\x87\x83\x82\x81Q\x81\x10a\x10CWa\x10Ca\x1BrV[` \x02` \x01\x01Q\x10\x15\x80\x15a\x10WWP\x81\x15[\x15a\x10yW\x82\x81\x81Q\x81\x10a\x10nWa\x10na\x1BrV[` \x02` \x01\x01Q\x91P[\x80a\x10\x83\x81a\x1B\x88V[\x91PPa\x0F\xDAV[P`\0\x80`\0a\x10\x9A\x84a\x07\x8BV[\x90Pa\x10\xA7`\x01\x82a\x1B\xC0V[`\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xC1Wa\x10\xC1a\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xEAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x95P[\x89\x84\x14a\x11yW\x80a\x10\xFF\x81a\x1CfV[\x91PPa\x11\x0B\x84a\x07\x0FV[\x90\x93P\x91P\x82\x8A\x11\x15a\x11\x1EW\x81a\x11 V[\x82[\x93P\x8A`\x03\x01`\0\x84\x8C\x11\x15a\x116W\x84a\x118V[\x83[\x81R` \x01\x90\x81R` \x01`\0 T\x86`\x01\x83a\x11U\x91\x90a\x1B\xC0V[`\xFF\x16\x81Q\x81\x10a\x11hWa\x11ha\x1BrV[` \x02` \x01\x01\x81\x81RPPa\x10\xEEV[PPPPP\x92\x95\x91\x94P\x92PV[`\0\x81\x80Q\x90` \x01 \x90P\x80\x83`\x04\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Qa\x11\xB6\x91\x90a\x1C\xB7V[`@Q\x80\x91\x03\x90 \x14a\x11\xDFW`\0\x81\x81R`\x04\x84\x01` R`@\x90 a\x11\xDD\x83\x82a\x1D|V[P[`\0a\x12!\x84`\x01\x01T`\x01a\x11\xF5\x91\x90a\x1B_V[`@\x80Q` \x80\x82\x01\x93\x90\x93R\x80\x82\x01\x86\x90R\x81Q\x80\x82\x03\x83\x01\x81R``\x90\x91\x01\x90\x91R\x80Q\x91\x01 \x90V[\x90P\x80\x84`\x03\x01`\0\x86`\x01\x01T`\x01a\x12;\x91\x90a\x1B_V[\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\x01\x84`\x02\x01`\0\x82\x82Ta\x12b\x91\x90a\x1B_V[\x92PP\x81\x90UP`\0a\x12x\x85`\x02\x01Ta\x05\xB4V[\x90Pa\x12\x87\x85`\x02\x01Ta\x03\xBCV[`\x01\x86\x01U\x80Q`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xA8Wa\x12\xA8a\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xD1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x132Wa\x13\x03\x87\x84\x83\x81Q\x81\x10a\x12\xF6Wa\x12\xF6a\x1BrV[` \x02` \x01\x01Qa\x15oV[\x82\x82\x81Q\x81\x10a\x13\x15Wa\x13\x15a\x1BrV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x13*\x81a\x1B\x88V[\x91PPa\x12\xD7V[Pa\x13A\x86`\x02\x01T\x82a\x13KV[\x90\x95UPPPPPV[`\0\x80a\x13W\x84a\x03\xBCV[\x90P\x82Qa\x13d\x85a\x14\x14V[\x14a\x13\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FReceived invalid number of peaks`D\x82\x01R`d\x01a\x06\xC8V[\x80\x81\x84`@Q` \x01a\x13\xC5\x92\x91\x90a\x1C\x12V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\xF5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x92\x91PPV[`\0\x81[\x80\x15a\x14FWa\x14)`\x02\x82a\x1B\xF0V[`\x01\x03a\x14>W\x81a\x14:\x81a\x1B\x88V[\x92PP[`\x01\x1Ca\x14\x18V[P\x91\x90PV[```\0a\x14Y\x84a\x14\x14V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14sWa\x14sa\x16eV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\x9CW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0\x80[`\xFF\x81\x10\x15a\x15\x1DW`\0\x85\x82`\xFF\x81\x10a\x14\xC0Wa\x14\xC0a\x1BrV[` \x02\x01Q\x14a\x15\x0BW\x84\x81`\xFF\x81\x10a\x14\xDCWa\x14\xDCa\x1BrV[` \x02\x01Q\x84\x83a\x14\xEC\x81a\x1B\x88V[\x94P\x81Q\x81\x10a\x14\xFEWa\x14\xFEa\x1BrV[` \x02` \x01\x01\x81\x81RPP[\x80a\x15\x15\x81a\x1B\x88V[\x91PPa\x14\xA3V[P\x81\x81\x14a\x15gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01RvInvalid number of peaks`H\x1B`D\x82\x01R`d\x01a\x06\xC8V[PP\x92\x91PPV[`\0\x82`\x01\x01T\x82\x11\x15a\x15\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkOut of range`\xA0\x1B`D\x82\x01R`d\x01a\x06\xC8V[`\0\x82\x81R`\x03\x84\x01` R`@\x90 Ta\x16\x16W`\0\x80a\x15\xD5\x84a\x07\x0FV[\x91P\x91P`\0a\x15\xE5\x86\x84a\x15oV[\x90P`\0a\x15\xF3\x87\x84a\x15oV[\x90Pa\x16\0\x86\x83\x83a\x06\xD8V[`\0\x87\x81R`\x03\x89\x01` R`@\x90 UPPPP[P`\0\x90\x81R`\x03\x91\x90\x91\x01` R`@\x90 T\x90V[`@Q\x80a\x1F\xE0\x01`@R\x80`\xFF\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x16^W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x16\xA3Wa\x16\xA3a\x16eV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x16\xBCW`\0\x80\xFD[`@Qa\x1F\xE0\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x16\xE0Wa\x16\xE0a\x16eV[`@R\x83\x01\x81\x85\x82\x11\x15a\x16\xF3W`\0\x80\xFD[\x84[\x82\x81\x10\x15a\x17\rW\x805\x82R` \x91\x82\x01\x91\x01a\x16\xF5V[P\x91\x95\x94PPPPPV[`\0\x80`\0a  \x84\x86\x03\x12\x15a\x17.W`\0\x80\xFD[\x835\x92Pa\x17?\x85` \x86\x01a\x16\xABV[\x91Pa \0\x84\x015\x90P\x92P\x92P\x92V[a\x1F\xE0\x81\x01\x81\x83`\0[`\xFF\x81\x10\x15a\x08\xEAW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x17ZV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x17\xB1W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x17\x95V[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x17\xD2W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\xFCW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x82`\x1F\x83\x01\x12a\x18\x1CW`\0\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x82\x11\x15a\x187Wa\x187a\x16eV[\x81`\x05\x1Ba\x18F\x82\x82\x01a\x16{V[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x18`W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x0E\xCBW\x825\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x18fV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\x92W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xAFW`\0\x80\xFD[a\x18\xBB\x85\x82\x86\x01a\x18\x0BV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x18\xF5W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x18\xD9V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x05\xAD` \x83\x01\x84a\x18\xC5V[`\0\x82`\x1F\x83\x01\x12a\x19$W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19=Wa\x19=a\x16eV[a\x19P`\x1F\x82\x01`\x1F\x19\x16` \x01a\x16{V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x19eW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x19\x9BW`\0\x80\xFD[\x865\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x19\xC7W`\0\x80\xFD[a\x19\xD3\x8A\x83\x8B\x01a\x19\x13V[\x94P`\x80\x89\x015\x91P\x80\x82\x11\x15a\x19\xE9W`\0\x80\xFD[a\x19\xF5\x8A\x83\x8B\x01a\x18\x0BV[\x93P`\xA0\x89\x015\x91P\x80\x82\x11\x15a\x1A\x0BW`\0\x80\xFD[Pa\x1A\x18\x89\x82\x8A\x01a\x18\x0BV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1A;W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1A`W`\0\x80\xFD[a\x1Al\x88\x83\x89\x01a\x18\x0BV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x1A\x82W`\0\x80\xFD[Pa\x1A\x8F\x87\x82\x88\x01a\x18\x0BV[\x91PP\x92\x95\x91\x94P\x92PV[\x84\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0a\x1A\xBA`\x80\x83\x01\x85a\x18\xC5V[\x82\x81\x03``\x84\x01Ra\x0E\xCB\x81\x85a\x18\xC5V[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\xDFW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xFCW`\0\x80\xFD[a\x18\xBB\x85\x82\x86\x01a\x19\x13V[`\0\x80a \0\x83\x85\x03\x12\x15a\x1B\x1CW`\0\x80\xFD[\x825\x91Pa\x1B-\x84` \x85\x01a\x16\xABV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xD5Wa\x03\xD5a\x1B6V[\x80\x82\x01\x80\x82\x11\x15a\x03\xD5Wa\x03\xD5a\x1B6V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1B\x9AWa\x1B\x9Aa\x1B6V[P`\x01\x01\x90V[`\0`\xFF\x82\x16`\xFF\x81\x03a\x1B\xB7Wa\x1B\xB7a\x1B6V[`\x01\x01\x92\x91PPV[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x03\xD5Wa\x03\xD5a\x1B6V[`\0\x81a\x1B\xE8Wa\x1B\xE8a\x1B6V[P`\0\x19\x01\x90V[`\0\x82a\x1C\rWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x82\x81R`\0` \x80\x83\x01\x84Q\x82\x86\x01`\0[\x82\x81\x10\x15a\x1C@W\x81Q\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x1C$V[P\x91\x97\x96PPPPPPPV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x03\xD5Wa\x03\xD5a\x1B6V[`\0`\xFF\x82\x16\x80a\x1CyWa\x1Cya\x1B6V[`\0\x19\x01\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1C\x97W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x14FWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x80\x83Ta\x1C\xC5\x81a\x1C\x83V[`\x01\x82\x81\x16\x80\x15a\x1C\xDDW`\x01\x81\x14a\x1C\xF2Wa\x1D!V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x1D!V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x1D\x18W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x1C\xFFV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\x1F\x82\x11\x15a\x1DwW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x1DTWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1DsW\x82\x81U`\x01\x01a\x1D`V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x95Wa\x1D\x95a\x16eV[a\x1D\xA9\x81a\x1D\xA3\x84Ta\x1C\x83V[\x84a\x1D-V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1D\xDEW`\0\x84\x15a\x1D\xC6WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1DsV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x1E\rW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1D\xEEV[P\x85\x82\x10\x15a\x1E+W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \x98\xA7\xA3w|\xCE\xCAv\x8BI\xB6\xDD\r\xD8\xFD\x02h\x8F?{]'\xA9\x01w\xC3\xCB\xC6c\xF4\x80\x03dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static MMR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MMR<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MMR<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MMR<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MMR<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MMR<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MMR)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MMR<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MMR_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MMR_ABI.clone(),
                MMR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getChildren` (0x49ae8dc3) function
        pub fn get_children(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([73, 174, 141, 195], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLeafIndex` (0x83e24c88) function
        pub fn get_leaf_index(
            &self,
            width: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([131, 226, 76, 136], width)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPeakIndexes` (0x2f2e2ea7) function
        pub fn get_peak_indexes(
            &self,
            width: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([47, 46, 46, 167], width)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSize` (0x023c23db) function
        pub fn get_size(
            &self,
            width: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([2, 60, 35, 219], width)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashBranch` (0x36614076) function
        pub fn hash_branch(
            &self,
            index: ::ethers::core::types::U256,
            left: [u8; 32],
            right: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 97, 64, 118], (index, left, right))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashLeaf` (0x452a3a0e) function
        pub fn hash_leaf(
            &self,
            index: ::ethers::core::types::U256,
            data_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([69, 42, 58, 14], (index, data_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `heightAt` (0x67e8f90c) function
        pub fn height_at(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([103, 232, 249, 12], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inclusionProof` (0x997db804) function
        pub fn inclusion_proof(
            &self,
            root: [u8; 32],
            width: ::ethers::core::types::U256,
            index: ::ethers::core::types::U256,
            value: ::ethers::core::types::Bytes,
            peaks: ::std::vec::Vec<[u8; 32]>,
            siblings: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [153, 125, 184, 4],
                    (root, width, index, value, peaks, siblings),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isLeaf` (0xcce3c13b) function
        pub fn is_leaf(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([204, 227, 193, 59], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mountainHeight` (0x2d129442) function
        pub fn mountain_height(
            &self,
            size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([45, 18, 148, 66], size)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numOfPeaks` (0xf38b7582) function
        pub fn num_of_peaks(
            &self,
            width: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 139, 117, 130], width)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peakBagging` (0xed9f51b3) function
        pub fn peak_bagging(
            &self,
            width: ::ethers::core::types::U256,
            peaks: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([237, 159, 81, 179], (width, peaks))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peakMapToPeaks` (0xf6fd8800) function
        pub fn peak_map_to_peaks(
            &self,
            width: ::ethers::core::types::U256,
            peak_map: [[u8; 32]; 255],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([246, 253, 136, 0], (width, peak_map))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peakUpdate` (0x26cf02ae) function
        pub fn peak_update(
            &self,
            width: ::ethers::core::types::U256,
            prev_peak_map: [[u8; 32]; 255],
            item_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [[u8; 32]; 255]> {
            self.0
                .method_hash([38, 207, 2, 174], (width, prev_peak_map, item_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `peaksToPeakMap` (0x78317ef4) function
        pub fn peaks_to_peak_map(
            &self,
            width: ::ethers::core::types::U256,
            peaks: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, [[u8; 32]; 255]> {
            self.0
                .method_hash([120, 49, 126, 244], (width, peaks))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollUp` (0xa248cd4f) function
        pub fn roll_up(
            &self,
            root: [u8; 32],
            width: ::ethers::core::types::U256,
            peaks: ::std::vec::Vec<[u8; 32]>,
            item_hashes: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 72, 205, 79], (root, width, peaks, item_hashes))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MMR<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getChildren` function with signature `getChildren(uint256)` and selector `0x49ae8dc3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getChildren", abi = "getChildren(uint256)")]
    pub struct GetChildrenCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLeafIndex` function with signature `getLeafIndex(uint256)` and selector `0x83e24c88`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getLeafIndex", abi = "getLeafIndex(uint256)")]
    pub struct GetLeafIndexCall {
        pub width: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPeakIndexes` function with signature `getPeakIndexes(uint256)` and selector `0x2f2e2ea7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPeakIndexes", abi = "getPeakIndexes(uint256)")]
    pub struct GetPeakIndexesCall {
        pub width: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSize` function with signature `getSize(uint256)` and selector `0x023c23db`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSize", abi = "getSize(uint256)")]
    pub struct GetSizeCall {
        pub width: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `hashBranch` function with signature `hashBranch(uint256,bytes32,bytes32)` and selector `0x36614076`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hashBranch", abi = "hashBranch(uint256,bytes32,bytes32)")]
    pub struct HashBranchCall {
        pub index: ::ethers::core::types::U256,
        pub left: [u8; 32],
        pub right: [u8; 32],
    }
    ///Container type for all input parameters for the `hashLeaf` function with signature `hashLeaf(uint256,bytes32)` and selector `0x452a3a0e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hashLeaf", abi = "hashLeaf(uint256,bytes32)")]
    pub struct HashLeafCall {
        pub index: ::ethers::core::types::U256,
        pub data_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `heightAt` function with signature `heightAt(uint256)` and selector `0x67e8f90c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "heightAt", abi = "heightAt(uint256)")]
    pub struct HeightAtCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `inclusionProof` function with signature `inclusionProof(bytes32,uint256,uint256,bytes,bytes32[],bytes32[])` and selector `0x997db804`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "inclusionProof",
        abi = "inclusionProof(bytes32,uint256,uint256,bytes,bytes32[],bytes32[])"
    )]
    pub struct InclusionProofCall {
        pub root: [u8; 32],
        pub width: ::ethers::core::types::U256,
        pub index: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::Bytes,
        pub peaks: ::std::vec::Vec<[u8; 32]>,
        pub siblings: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `isLeaf` function with signature `isLeaf(uint256)` and selector `0xcce3c13b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isLeaf", abi = "isLeaf(uint256)")]
    pub struct IsLeafCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mountainHeight` function with signature `mountainHeight(uint256)` and selector `0x2d129442`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mountainHeight", abi = "mountainHeight(uint256)")]
    pub struct MountainHeightCall {
        pub size: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `numOfPeaks` function with signature `numOfPeaks(uint256)` and selector `0xf38b7582`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "numOfPeaks", abi = "numOfPeaks(uint256)")]
    pub struct NumOfPeaksCall {
        pub width: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `peakBagging` function with signature `peakBagging(uint256,bytes32[])` and selector `0xed9f51b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "peakBagging", abi = "peakBagging(uint256,bytes32[])")]
    pub struct PeakBaggingCall {
        pub width: ::ethers::core::types::U256,
        pub peaks: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `peakMapToPeaks` function with signature `peakMapToPeaks(uint256,bytes32[255])` and selector `0xf6fd8800`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "peakMapToPeaks", abi = "peakMapToPeaks(uint256,bytes32[255])")]
    pub struct PeakMapToPeaksCall {
        pub width: ::ethers::core::types::U256,
        pub peak_map: [[u8; 32]; 255],
    }
    ///Container type for all input parameters for the `peakUpdate` function with signature `peakUpdate(uint256,bytes32[255],bytes32)` and selector `0x26cf02ae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "peakUpdate", abi = "peakUpdate(uint256,bytes32[255],bytes32)")]
    pub struct PeakUpdateCall {
        pub width: ::ethers::core::types::U256,
        pub prev_peak_map: [[u8; 32]; 255],
        pub item_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `peaksToPeakMap` function with signature `peaksToPeakMap(uint256,bytes32[])` and selector `0x78317ef4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "peaksToPeakMap", abi = "peaksToPeakMap(uint256,bytes32[])")]
    pub struct PeaksToPeakMapCall {
        pub width: ::ethers::core::types::U256,
        pub peaks: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `rollUp` function with signature `rollUp(bytes32,uint256,bytes32[],bytes32[])` and selector `0xa248cd4f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rollUp", abi = "rollUp(bytes32,uint256,bytes32[],bytes32[])")]
    pub struct RollUpCall {
        pub root: [u8; 32],
        pub width: ::ethers::core::types::U256,
        pub peaks: ::std::vec::Vec<[u8; 32]>,
        pub item_hashes: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum MMRCalls {
        GetChildren(GetChildrenCall),
        GetLeafIndex(GetLeafIndexCall),
        GetPeakIndexes(GetPeakIndexesCall),
        GetSize(GetSizeCall),
        HashBranch(HashBranchCall),
        HashLeaf(HashLeafCall),
        HeightAt(HeightAtCall),
        InclusionProof(InclusionProofCall),
        IsLeaf(IsLeafCall),
        MountainHeight(MountainHeightCall),
        NumOfPeaks(NumOfPeaksCall),
        PeakBagging(PeakBaggingCall),
        PeakMapToPeaks(PeakMapToPeaksCall),
        PeakUpdate(PeakUpdateCall),
        PeaksToPeakMap(PeaksToPeakMapCall),
        RollUp(RollUpCall),
    }
    impl ::ethers::core::abi::AbiDecode for MMRCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetChildrenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetChildren(decoded));
            }
            if let Ok(decoded) = <GetLeafIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLeafIndex(decoded));
            }
            if let Ok(decoded) = <GetPeakIndexesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPeakIndexes(decoded));
            }
            if let Ok(decoded) = <GetSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSize(decoded));
            }
            if let Ok(decoded) = <HashBranchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HashBranch(decoded));
            }
            if let Ok(decoded) = <HashLeafCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HashLeaf(decoded));
            }
            if let Ok(decoded) = <HeightAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HeightAt(decoded));
            }
            if let Ok(decoded) = <InclusionProofCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InclusionProof(decoded));
            }
            if let Ok(decoded) = <IsLeafCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsLeaf(decoded));
            }
            if let Ok(decoded) = <MountainHeightCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MountainHeight(decoded));
            }
            if let Ok(decoded) = <NumOfPeaksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NumOfPeaks(decoded));
            }
            if let Ok(decoded) = <PeakBaggingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PeakBagging(decoded));
            }
            if let Ok(decoded) = <PeakMapToPeaksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PeakMapToPeaks(decoded));
            }
            if let Ok(decoded) = <PeakUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PeakUpdate(decoded));
            }
            if let Ok(decoded) = <PeaksToPeakMapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PeaksToPeakMap(decoded));
            }
            if let Ok(decoded) = <RollUpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollUp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MMRCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetChildren(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLeafIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPeakIndexes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HashBranch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashLeaf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HeightAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InclusionProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsLeaf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MountainHeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumOfPeaks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PeakBagging(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PeakMapToPeaks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PeakUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PeaksToPeakMap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RollUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MMRCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetChildren(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLeafIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPeakIndexes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashBranch(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashLeaf(element) => ::core::fmt::Display::fmt(element, f),
                Self::HeightAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::InclusionProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsLeaf(element) => ::core::fmt::Display::fmt(element, f),
                Self::MountainHeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumOfPeaks(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeakBagging(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeakMapToPeaks(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeakUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeaksToPeakMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollUp(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetChildrenCall> for MMRCalls {
        fn from(value: GetChildrenCall) -> Self {
            Self::GetChildren(value)
        }
    }
    impl ::core::convert::From<GetLeafIndexCall> for MMRCalls {
        fn from(value: GetLeafIndexCall) -> Self {
            Self::GetLeafIndex(value)
        }
    }
    impl ::core::convert::From<GetPeakIndexesCall> for MMRCalls {
        fn from(value: GetPeakIndexesCall) -> Self {
            Self::GetPeakIndexes(value)
        }
    }
    impl ::core::convert::From<GetSizeCall> for MMRCalls {
        fn from(value: GetSizeCall) -> Self {
            Self::GetSize(value)
        }
    }
    impl ::core::convert::From<HashBranchCall> for MMRCalls {
        fn from(value: HashBranchCall) -> Self {
            Self::HashBranch(value)
        }
    }
    impl ::core::convert::From<HashLeafCall> for MMRCalls {
        fn from(value: HashLeafCall) -> Self {
            Self::HashLeaf(value)
        }
    }
    impl ::core::convert::From<HeightAtCall> for MMRCalls {
        fn from(value: HeightAtCall) -> Self {
            Self::HeightAt(value)
        }
    }
    impl ::core::convert::From<InclusionProofCall> for MMRCalls {
        fn from(value: InclusionProofCall) -> Self {
            Self::InclusionProof(value)
        }
    }
    impl ::core::convert::From<IsLeafCall> for MMRCalls {
        fn from(value: IsLeafCall) -> Self {
            Self::IsLeaf(value)
        }
    }
    impl ::core::convert::From<MountainHeightCall> for MMRCalls {
        fn from(value: MountainHeightCall) -> Self {
            Self::MountainHeight(value)
        }
    }
    impl ::core::convert::From<NumOfPeaksCall> for MMRCalls {
        fn from(value: NumOfPeaksCall) -> Self {
            Self::NumOfPeaks(value)
        }
    }
    impl ::core::convert::From<PeakBaggingCall> for MMRCalls {
        fn from(value: PeakBaggingCall) -> Self {
            Self::PeakBagging(value)
        }
    }
    impl ::core::convert::From<PeakMapToPeaksCall> for MMRCalls {
        fn from(value: PeakMapToPeaksCall) -> Self {
            Self::PeakMapToPeaks(value)
        }
    }
    impl ::core::convert::From<PeakUpdateCall> for MMRCalls {
        fn from(value: PeakUpdateCall) -> Self {
            Self::PeakUpdate(value)
        }
    }
    impl ::core::convert::From<PeaksToPeakMapCall> for MMRCalls {
        fn from(value: PeaksToPeakMapCall) -> Self {
            Self::PeaksToPeakMap(value)
        }
    }
    impl ::core::convert::From<RollUpCall> for MMRCalls {
        fn from(value: RollUpCall) -> Self {
            Self::RollUp(value)
        }
    }
    ///Container type for all return fields from the `getChildren` function with signature `getChildren(uint256)` and selector `0x49ae8dc3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetChildrenReturn {
        pub left: ::ethers::core::types::U256,
        pub right: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getLeafIndex` function with signature `getLeafIndex(uint256)` and selector `0x83e24c88`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetLeafIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPeakIndexes` function with signature `getPeakIndexes(uint256)` and selector `0x2f2e2ea7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetPeakIndexesReturn {
        pub peak_indexes: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `getSize` function with signature `getSize(uint256)` and selector `0x023c23db`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSizeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hashBranch` function with signature `hashBranch(uint256,bytes32,bytes32)` and selector `0x36614076`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HashBranchReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hashLeaf` function with signature `hashLeaf(uint256,bytes32)` and selector `0x452a3a0e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HashLeafReturn(pub [u8; 32]);
    ///Container type for all return fields from the `heightAt` function with signature `heightAt(uint256)` and selector `0x67e8f90c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HeightAtReturn {
        pub height: u8,
    }
    ///Container type for all return fields from the `inclusionProof` function with signature `inclusionProof(bytes32,uint256,uint256,bytes,bytes32[],bytes32[])` and selector `0x997db804`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct InclusionProofReturn(pub bool);
    ///Container type for all return fields from the `isLeaf` function with signature `isLeaf(uint256)` and selector `0xcce3c13b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsLeafReturn(pub bool);
    ///Container type for all return fields from the `mountainHeight` function with signature `mountainHeight(uint256)` and selector `0x2d129442`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MountainHeightReturn(pub u8);
    ///Container type for all return fields from the `numOfPeaks` function with signature `numOfPeaks(uint256)` and selector `0xf38b7582`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NumOfPeaksReturn {
        pub num: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `peakBagging` function with signature `peakBagging(uint256,bytes32[])` and selector `0xed9f51b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PeakBaggingReturn(pub [u8; 32]);
    ///Container type for all return fields from the `peakMapToPeaks` function with signature `peakMapToPeaks(uint256,bytes32[255])` and selector `0xf6fd8800`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PeakMapToPeaksReturn {
        pub peaks: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `peakUpdate` function with signature `peakUpdate(uint256,bytes32[255],bytes32)` and selector `0x26cf02ae`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PeakUpdateReturn {
        pub next_peak_map: [[u8; 32]; 255],
    }
    ///Container type for all return fields from the `peaksToPeakMap` function with signature `peaksToPeakMap(uint256,bytes32[])` and selector `0x78317ef4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PeaksToPeakMapReturn {
        pub peak_map: [[u8; 32]; 255],
    }
    ///Container type for all return fields from the `rollUp` function with signature `rollUp(bytes32,uint256,bytes32[],bytes32[])` and selector `0xa248cd4f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RollUpReturn {
        pub new_root: [u8; 32],
    }
}

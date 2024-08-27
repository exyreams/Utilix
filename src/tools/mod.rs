/// This module contains a collection of utility functions and structs
/// for performing common data manipulations and conversions.
///
///  - **base64_encoder:** Implements functionality to encode and decode data using base64 encoding.
///  - **date_converter:** Provides tools to convert dates between different formats like RFC3339, RFC2822, ISO8601, Unix timestamps, and human-readable formats.
///  - **hash_generator:** Generates various hash values (SHA-1, SHA-256, SHA-384, SHA-512) from an input string.
///  - **number_base_converter:**  Facilitates the conversion of numbers between different number bases.
///  - **password_generator:** Generates secure random passwords with configurable lengths and character sets.
///  - **qr_code_generator:** Encodes data into QR codes that can be visualized and scanned.
///  - **uuid_generator:** Generates universally unique identifiers (UUIDs) conforming to RFC4122.
///
pub mod base64_encoder;
pub mod color_converter;
pub mod date_converter;
pub mod hash_generator;
pub mod number_base_converter;
pub mod password_generator;
pub mod qr_code_generator;
pub mod uuid_generator;

//! Methods to manipulate the XML databases.

use unity::prelude::*;

#[unity::from_offset("App", "Database", "Release")]
pub fn database_release(method_info: OptionalMethod);

#[unity::from_offset("App", "Database", "Load")]
pub fn database_load(method_info: OptionalMethod);

#[unity::from_offset("App", "Database", "Completed")]
pub fn database_completed(method_info: OptionalMethod);

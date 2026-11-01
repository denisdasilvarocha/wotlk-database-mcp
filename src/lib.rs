//! MCP server for the World of Warcraft: Wrath of the Lich King database.

mod db;
mod format;
mod kinds;
mod provision;
mod schema_version;
mod server;

pub use db::DatabaseError;
pub use provision::fetch_database;
pub use schema_version::SCHEMA_VERSION;
pub use server::WotlkServer;

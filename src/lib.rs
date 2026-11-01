//! MCP server for the World of Warcraft: Wrath of the Lich King database.

mod db;
mod format;
mod kinds;
mod server;

pub use server::WotlkServer;

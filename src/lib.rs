//! Embedding surface of szmcp: the ZIM reader and the HTML to Markdown
//! converter, without the MCP server, the CLI, Xapian search or the ZIM
//! writer. Build with `default-features = false`; the `szmcp` binary keeps
//! the default `server` + `xapian` features and compiles these same files.
#![allow(dead_code)]

#[path = "cleanup.rs"]
pub mod cleanup;
#[path = "entities.rs"]
pub mod entities;
#[path = "html.rs"]
pub mod html;
#[path = "html2md.rs"]
pub mod html2md;
#[path = "htmldom.rs"]
pub mod htmldom;
#[path = "infobox_html.rs"]
pub mod infobox_html;
#[path = "markdown.rs"]
pub mod markdown;
#[path = "tables.rs"]
pub mod tables;
#[path = "util.rs"]
pub mod util;
#[path = "wikil10n.rs"]
pub mod wikil10n;
#[path = "zim.rs"]
pub mod zim;
#[path = "zimcommon.rs"]
pub mod zimcommon;

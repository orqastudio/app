/// Plugin name collision detection across installed plugins.
pub mod collision;
/// Plugin filesystem discovery: scans `plugins/` for installed plugin directories.
pub mod discovery;
/// Plugin installer: fetches and installs plugins from local paths or GitHub.
pub mod installer;
/// Plugin lockfile management: records installed plugin versions and source URLs.
pub mod lockfile;
/// Plugin manifest reader: deserializes `orqa-plugin.json` into typed structs.
pub mod manifest;
/// Plugin registry: lists available plugins from the configured registry source.
pub mod registry;

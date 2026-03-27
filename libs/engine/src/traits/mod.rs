// Storage trait module declarations for the orqa-engine crate.
//
// This module exposes abstract storage interfaces that each access layer implements.
// The engine defines what operations storage must support; the app, daemon, and CLI
// each provide their own implementation (file-based, SQLite, or in-memory).

pub mod storage;

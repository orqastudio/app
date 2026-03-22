---
id: IDEA-6f1d94a8
title: Live file browser widget for directory structure in documents
description: Replace static directory structure sections in documents with a live file browser markdown widget. Configure an entry point folder and it renders the current structure. Could pull descriptions from comment-based metadata.
status: captured
created: 2026-03-15
updated: 2026-03-15
horizon: next
research-needed: []
relationships:
  - target: PILLAR-569581e0
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

## Motivation

Documentation pages that describe project structure (e.g., module layouts, directory trees) contain static ASCII directory trees that go stale immediately when files are added, moved, or removed. A live file browser widget embedded in the markdown renderer would replace static trees with a dynamically rendered view of the actual filesystem at a configured path. This keeps structure documentation accurate without manual maintenance.

## Sketch

A markdown directive syntax (e.g., `:::filebrowser path="backend/src-tauri/src"`) that the document renderer replaces with a live tree component. The component calls a Tauri command to list the directory at render time. Descriptions could be sourced from file-level comment metadata (linked with IDEA-50d6e149) or from README files in subdirectories. Collapsed by default with expand-on-click behaviour for deep trees.

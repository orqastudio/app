---

id: RES-d6d344c9
type: discovery-research
title: Notification strategy
description: "Research on notification channels, event-to-channel mapping, and phased implementation for OrqaStudio"
status: completed
created: 2026-03-14
updated: 2026-03-14
---

## Overview

OrqaStudio needs a notification strategy that routes events to the right channel based on urgency, user attention state, and context. Three channels are available: toast notifications (in-viewport), notification panel (in-app persistent), and desktop notifications (OS-level).

## Toast Notifications

Toast notifications are already implemented (TASK-97e1fa39) using the sonner library via shadcn-svelte's Toaster component.

### When to Use

- **Immediate feedback** -- action confirmations ("Session saved", "Artifact created", "Settings updated")
- **Transient warnings** -- non-blocking issues ("Connection retry in 5s", "Search index outdated")
- **Errors requiring attention** -- failures that need acknowledgment but not navigation ("Save failed -- retry?")
- **Background task completion** -- "Index rebuild complete", "Export finished"

### When NOT to Use

- Events that need to be reviewed later (use notification panel)
- Events while the app is minimised (use desktop notification)
- Events that require multi-step response (use a dialog or dedicated UI)

### Variants

| Variant | Use Case | Auto-dismiss |
| --------- | ---------- | ------------- |
| `success` | Confirmations | Yes (3s) |
| `info` | Informational updates | Yes (5s) |
| `warning` | Non-blocking issues | Yes (8s) |
| `error` | Failures | No (manual dismiss) |

## In-App Notification Panel

### Design Sketch

A slide-out panel (right side, 360px width) accessible from a bell icon in the header. The icon shows an unread count badge when notifications are pending.

```text
+------------------------------------------+
| Notifications                    Mark all |
+------------------------------------------+
| [dot] Integrity check: 3 warnings   2m   |
|       in EPIC-7394ba2a, RULE-0be7765e             |
+------------------------------------------+
| [dot] Task TASK-c4a09d02 completed       15m   |
|       Body template validation            |
+------------------------------------------+
|       Session auto-saved            1h    |
|       12 messages persisted               |
+------------------------------------------+
```

### What Events Populate It

| Event Category | Examples | Retention |
| ---------------- | ---------- | ----------- |
| **Integrity findings** | Broken links, missing inverses, schema violations | Until resolved |
| **Task lifecycle** | Task started, completed, blocked | 24 hours |
| **Session events** | Auto-save, session resume, context loss warning | Current session |
| **Agent activity** | Agent delegation, completion report, review verdict | 24 hours |
| **System health** | Search index stale, database migration needed | Until resolved |

### Data Model

```typescript
interface AppNotification {
  id: string;
  timestamp: Date;
  channel: "panel";
  category: string;
  title: string;
  body: string;
  read: boolean;
  dismissed: boolean;
  artifactId?: string;  // Navigate to artifact on click
  actions?: Array<{ label: string; action: string }>;
}
```

### Storage

Notifications should be stored in SQLite (ephemeral session data per AD-859ed163). They are not governance artifacts. Table: `notifications` with columns matching the data model above. Cleared on app restart or after retention period.

## Desktop Notifications (Tauri Notification Plugin)

### Tauri Integration

Tauri provides `@tauri-apps/plugin-notification` which wraps native OS notification APIs. This gives OrqaStudio access to:

- Windows: Windows notification center
- macOS: NSUserNotificationCenter
- Linux: libnotify / D-Bus notifications

### When to Escalate to Desktop

Desktop notifications are for events that need attention when the user is NOT looking at the app:

| Event | Desktop? | Rationale |
| ------- | ---------- | ----------- |
| Long-running build complete | Yes | User may have switched to another app |
| Agent task completed (>30s) | Yes | Agent work happens in background |
| Integrity check found errors | Yes | Critical quality issue |
| Session about to expire | Yes | Data loss prevention |
| Toast-level events | No | User is already in-app |
| Panel-level routine events | No | Not urgent enough for OS interrupt |

### Permission Model

Desktop notifications require user permission. OrqaStudio should:

1. Never request permission on first launch
2. Offer an opt-in toggle in Settings > Notifications
3. Request OS permission only when the user enables the toggle
4. Gracefully degrade to panel-only if permission is denied

### Configuration

```typescript
interface NotificationPreferences {
  desktopEnabled: boolean;
  desktopCategories: string[];  // Which categories escalate to desktop
  quietHours?: { start: string; end: string };  // No desktop during these hours
}
```

## Event-to-Channel Mapping

| Event | Toast | Panel | Desktop |
| ------- | :-----: | :-----: | :-------: |
| Action confirmation (save, create, delete) | Y | - | - |
| Transient warning (retry, stale data) | Y | - | - |
| Error (save failed, command error) | Y | Y | - |
| Integrity finding (warning) | - | Y | - |
| Integrity finding (error) | Y | Y | Y* |
| Task started | - | Y | - |
| Task completed | Y | Y | Y* |
| Task blocked | Y | Y | - |
| Agent delegation | - | Y | - |
| Agent completion report | Y | Y | Y* |
| Review verdict (PASS) | Y | Y | - |
| Review verdict (FAIL) | Y | Y | Y* |
| Session auto-save | - | Y | - |
| Session resume | Y | - | - |
| Long-running operation complete | Y | Y | Y* |
| Search index rebuilt | Y | Y | - |
| System health warning | Y | Y | Y* |

`Y*` = Only if desktop notifications are enabled and the category is in `desktopCategories`.

## Recommendation: Phased Implementation

### Phase 1: Toast Foundation (DONE -- TASK-97e1fa39)

Toast notifications via sonner/shadcn Toaster. Covers immediate feedback for user actions and transient errors.

### Phase 2: Notification Service

Create a central `NotificationService` (Svelte store in `$lib/stores/notifications.svelte.ts`) that:

- Accepts events with category, severity, and content
- Routes to toast via sonner API
- Queues for panel display
- Provides reactive state for unread count

This is the routing layer that all other notification sources call into. No UI changes beyond the service.

### Phase 3: Notification Panel

Build the slide-out panel UI:

- Bell icon with unread badge in app header
- Panel component with notification list
- Read/dismiss/mark-all-read actions
- Click-to-navigate for artifact-linked notifications
- SQLite persistence for notification history

### Phase 4: Desktop Notifications

Add Tauri notification plugin:

- Settings toggle for desktop notification opt-in
- Category-level configuration
- Quiet hours support
- Permission request flow
- Escalation logic in NotificationService

### Phase 5: Event Source Integration

Wire notification sources into the service:

- Integrity checker findings
- Agent lifecycle events (delegation, completion, review)
- Session events (auto-save, resume, context loss)
- System health monitoring

Each phase is independently useful. Phase 2 (service) is the critical foundation that all subsequent phases depend on.

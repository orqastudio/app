---
id: security-audit
layer: canon
title: "Security Audit"
name: security-audit
description: |
  Security auditing methodology: secret management, capability/permission systems,
  file system scoping, input validation, dependency auditing, and vulnerability
  classification. Portable across desktop and web applications.
  Use when: Auditing application security, reviewing permissions, checking for
  credential exposure, validating input handling, or assessing dependency risk.
version: 1.0.0
tags: [security, audit, permissions, secrets, validation]
user-invocable: true
---


Security auditing methodology for applications. This skill teaches *what to check and how* — the specific security mechanisms (e.g., Tauri capabilities, keyring APIs) come from the project's technology skills.

## Security Domains

### 1. Secret Management

- Secrets (API keys, tokens, credentials) MUST use OS-level secure storage
- NEVER store secrets in databases, config files, `.env` files, or source code
- Secrets loaded into memory only when needed and not persisted in logs
- All external API calls use HTTPS — no plaintext HTTP
- Log operations but NEVER log sensitive values

### 2. Permission / Capability Systems

- Apply principle of least privilege — only grant permissions the app actually needs
- Each permission must be documented with its justification
- Audit for overly broad grants (wildcard paths, unrestricted access)
- File system access must be scoped to intended directories

### 3. Process Spawning / Shell Access

- Shell/process access MUST be scoped to specific binaries — no arbitrary command execution
- Validate that process spawning config restricts to named executables only
- Inter-process communication should use structured formats, not shell pipes

### 4. File System Access Scoping

- Applications never accept arbitrary file paths from untrusted input without validation
- Path traversal prevention: canonicalize paths and verify they are within scope
- Temporary files in designated temp directories only

### 5. Input Validation

- Every external-facing function must validate its inputs before processing
- String inputs checked for injection (SQL, path traversal, command injection)
- Numeric inputs bounds-checked
- Enum inputs validated against known variants
- Error messages to users do not leak internal details — internals go to logs only

### 6. Data Integrity

- Parameterized queries only — no string concatenation in SQL
- Foreign key constraints enforced
- No sensitive data in general-purpose storage when secure storage exists

## Security Audit Checklist

### Dependency Audit

- Check for known vulnerabilities in all dependencies (language-specific audit tools)
- Review transitive dependencies for supply chain risk

### Permission Audit

- No wildcard/unrestricted permissions without documented justification
- Process spawning scoped to specific binaries
- File system access scoped to intended directories
- Content security policies prevent injection

### Code Audit

- Search for hardcoded strings resembling API keys or tokens
- Search for `unsafe` blocks — each must have documented justification
- Search for dynamic code execution (eval, exec)
- Verify all database queries use parameterized inputs
- Verify all file paths are validated and scoped before use
- Verify all user-facing function inputs are validated

### Runtime Audit

- Verify HTTPS for all external API calls
- Verify secure storage is used for credentials
- Verify error messages don't leak internal paths or stack traces

## Vulnerability Classification

| Severity | Examples |
|----------|---------|
| **Critical** | Remote code execution, credential exposure, arbitrary file access |
| **High** | Path traversal, SQL injection, privilege escalation, unscoped shell access |
| **Medium** | Information disclosure, missing input validation, overly broad permissions |
| **Low** | Missing security headers, verbose error messages, unused permissions |

## Critical Rules

- NEVER approve code that stores secrets in plaintext
- NEVER approve wildcard permissions without explicit justification
- NEVER approve unsanitized input reaching queries or shell commands
- NEVER approve unscoped process spawning
- NEVER approve disabled security features without documented justification
- Security findings at Critical or High severity are release blockers
- Document all accepted security risks with explicit justification

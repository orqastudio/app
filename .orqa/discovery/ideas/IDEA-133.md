---
id: IDEA-af2d6611
type: idea
title: "Skills learning approach from Singularity — adaptive agent skill discovery"
description: "Investigate the skills learning approach from Singularity (https://github.com/Shmayro/singularity-claude) for potential integration into the OrqaStudio app (Rust backend) and/or the Claude Code connector. Could enable agents to learn and adapt their skill usage over time."
status: captured
created: 2026-03-20
updated: 2026-03-20
relationships:
  - target: PILLAR-94b281db
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

# IDEA-133: Singularity Skills Learning

Investigate [Singularity](https://github.com/Shmayro/singularity-claude)'s approach to skills learning for potential integration. Could complement OrqaStudio's skill injection system with adaptive learning — agents discover which skills are most useful for which tasks and improve their selection over time.

Potential implementation paths:
- **In-app (Rust)**: The ONNX embeddings server could track skill usage → outcome correlations
- **Claude Code connector**: The prompt injector could learn from session outcomes which skills produced better results
- **Both**: The app tracks the data, the connector acts on it

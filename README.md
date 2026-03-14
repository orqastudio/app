[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# OrqaStudio Types

Shared TypeScript types for the OrqaStudio ecosystem. This package provides all the type definitions used across OrqaStudio packages, including artifact graph types, session/message types, project configuration types, and shared constants.

## Install

```bash
npm install @orqastudio/types
```

## Usage

```typescript
import type { ArtifactNode, ArtifactRef, GraphStats } from "@orqastudio/types";
import { ARTIFACT_TYPES, INVERSE_MAP, SINGLE_REF_FIELDS } from "@orqastudio/types";
```

Constants can also be imported from a dedicated subpath:

```typescript
import { INVERSE_MAP, SINGLE_REF_FIELDS, ARRAY_REF_FIELDS } from "@orqastudio/types/constants";
```

## License

[Apache-2.0](LICENSE)

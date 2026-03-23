![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# Logger

Centralised structured logging library for OrqaStudio — structured log output with dashboard forwarding.

## Usage

```typescript
import { createLogger } from '@orqastudio/logger';

const log = createLogger('my-module');
log.info('Starting up');
log.error('Something went wrong', { error });
```

## Development

```bash
npm install
npm run build
```

## License

BSL-1.1 — see [LICENSE](LICENSE) for details.

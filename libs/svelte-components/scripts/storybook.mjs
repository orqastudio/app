/**
 * Start Storybook on the port defined in infrastructure/ports.json.
 * This is the single source of truth for the storybook port — never hardcode it here.
 */
import ports from '../../../infrastructure/ports.json' with { type: 'json' };
import { spawnSync } from 'child_process';

const port = ports.services.storybook.port;
const result = spawnSync('storybook', ['dev', '-p', String(port)], { stdio: 'inherit', shell: true });
process.exit(result.status ?? 0);

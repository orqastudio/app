import { base } from "@orqastudio/plugin-svelte/eslint";

export default [
  ...base,
  {
    ignores: ["dist/", "node_modules/"],
  },
];

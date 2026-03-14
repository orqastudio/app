import { base } from "@orqastudio/eslint-config";

export default [
  ...base,
  {
    ignores: ["dist/", "node_modules/"],
  },
];

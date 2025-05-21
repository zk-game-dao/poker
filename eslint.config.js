import eslint from '@eslint/js';
import reactPlugin from 'eslint-plugin-react';
import tseslint from 'typescript-eslint';
import globals from "globals";

export default tseslint.config(
  eslint.configs.recommended,
  tseslint.configs.recommended,
  {
    files: ["**/*.ts", "**/*.tsx", "**/*.js", "**/*.jsx"],
    ignores: [
      "**/node_modules/**", // Ignore all node_modules
      "**/dist/**",         // Ignore any dist folder at any level
      "**/build/**",        // Ignore any build folder at any level
    ],
    rules: {
      // Allow any for now
      "@typescript-eslint/no-explicit-any": "off",
    },
  },

  {
    // disable type-aware linting on JS files
    files: ['**/*.js'],
    extends: [tseslint.configs.disableTypeChecked],
  },

  {
    files: ["*.config.ts", "*.config.js"],
    languageOptions: {
      sourceType: "commonjs",
      globals: {
        ...globals.node,
        ...globals.amd,
      },
    },
  },

  {
    files: ['**/*.{js,mjs,cjs,jsx,mjsx,ts,tsx,mtsx}'],
    ...reactPlugin.configs.flat.recommended,
  },

  {
    files: ["**/*.jsx", "**/*.tsx"],
    rules: {
      "react/prop-types": "off", // Disable prop-types as we use TypeScript for type checking
      "react/no-unescaped-entities": "off", // Disable the rule that prevents unescaped entities
      "react/jsx-filename-extension": [
        "error",
        {
          extensions: [".jsx", ".tsx"],
        },
      ],
      "react/react-in-jsx-scope": "off",
    },
  }
);

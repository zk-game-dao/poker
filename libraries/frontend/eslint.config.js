import baseConfig from '../../eslint.config.js';

export default [
  ...baseConfig,
  {
    ignores: [
      "dist/**", // Explicitly ignore this folder
    ],
  },
]

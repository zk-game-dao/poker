/** @type {import('ts-jest').JestConfigWithTsJest} **/
export default {
  "transform": {
    "^.+\\.tsx?$": ["ts-jest", { /* ts-jest config goes here in Jest */ }],
    // "^.+\\.css$": "<rootDir>/config/cssTransform.js"
  },
  "globals": {
    "ts-jest": {
      "tsconfig": "tsconfig.test.json"
    }
  },
  "moduleNameMapper": {
    "\\.(css|less|scss|sass)$": "identity-obj-proxy"
  },
  "transformIgnorePatterns": [
    "node_modules/(?!react-json-view-lite)"
  ]
}
;
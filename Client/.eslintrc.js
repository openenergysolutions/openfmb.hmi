module.exports = {
  "root": true,
  "ignorePatterns": [
    "projects/**/*"
  ],
  "extends": [
    "plugin:@angular-eslint/recommended",
    "plugin:@angular-eslint/template/process-inline-templates",
    "plugin:@typescript-eslint/recommended"
  ],
  "parser": "@typescript-eslint/parser",
  "parserOptions": {
    "ecmaVersion": 2020,
    "sourceType": "module",
    "project": [
      "tsconfig.json"
    ],
    "tsconfigRootDir": __dirname,
    "createDefaultProgram": true
  },
  "plugins": ["@typescript-eslint", "deprecation"],
  "overrides": [
    {
      "files": [
        "*.html"
      ],
      "extends": [
        "plugin:@angular-eslint/template/recommended"
      ]
    }
  ],
  "rules": {
    "@typescript-eslint/no-unused-expressions": "off",
    "@angular-eslint/directive-selector": [
      "error",
      {
        "type": "attribute",
        "prefix": "app",
        "style": "camelCase"
      }
    ],
    "@angular-eslint/component-selector": [
      "error",
      {
        "type": "element",
        "prefix": "app",
        "style": "kebab-case"
      }
    ],
    "@typescript-eslint/no-inferrable-types": "off",
    "@typescript-eslint/no-explicit-any": ["off"]   
  }
}

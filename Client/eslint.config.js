const {
    defineConfig,
    globalIgnores,
} = require("eslint/config");

const tsParser = require("@typescript-eslint/parser");
const typescriptEslint = require("@typescript-eslint/eslint-plugin");
const deprecation = require("eslint-plugin-deprecation");
const js = require("@eslint/js");

const {
    FlatCompat,
} = require("@eslint/eslintrc");

const compat = new FlatCompat({
    baseDirectory: __dirname,
    recommendedConfig: js.configs.recommended,
    allConfig: js.configs.all
});

module.exports = defineConfig([{
    extends: compat.extends(
        "plugin:@angular-eslint/recommended",
        "plugin:@angular-eslint/template/process-inline-templates",
        "plugin:@typescript-eslint/recommended",
    ),

    languageOptions: {
        parser: tsParser,
        "ecmaVersion": 2020,
        "sourceType": "module",

        parserOptions: {
            "project": ["tsconfig.json"],
            "tsconfigRootDir": __dirname,
            "createDefaultProgram": true,
        },
    },

    plugins: {
        "@typescript-eslint": typescriptEslint,
        deprecation,
    },

    "rules": {
        "@typescript-eslint/no-unused-expressions": "off",

        "@angular-eslint/directive-selector": ["error", {
            "type": "attribute",
            "prefix": "app",
            "style": "camelCase",
        }],

        "@angular-eslint/component-selector": ["error", {
            "type": "element",
            "prefix": "app",
            "style": "kebab-case",
        }],

        "@typescript-eslint/no-inferrable-types": "off",
        "@typescript-eslint/no-explicit-any": ["off"],
        "@typescript-eslint/no-unused-vars": [
            "error",
            { "argsIgnorePattern": "^_" }
        ]
    },
}, globalIgnores(["projects/**/*"]), {
    files: ["**/*.html"],
    extends: compat.extends("plugin:@angular-eslint/template/recommended"),
}, globalIgnores(["**/dist"])]);

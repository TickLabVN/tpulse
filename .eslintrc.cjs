module.exports = {
  root: true,
  env: { browser: true, es2020: true },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/stylistic-type-checked',
    'plugin:react/recommended',
    'plugin:react/jsx-runtime',
    'plugin:react-hooks/recommended'
  ],
  ignorePatterns: ['dist', '.eslintrc.cjs'],
  parser: '@typescript-eslint/parser',
  plugins: ['react-refresh', 'react', '@typescript-eslint'],
  rules: {
    '@typescript-eslint/no-unused-vars': 'error',
    'react-refresh/only-export-components': 'off',
    'eqeqeq': 'error',
    'no-console': 'warn',
    '@typescript-eslint/consistent-type-definitions': 'off',
    'react/prop-types': 'off',
    '@typescript-eslint/no-var-requires': 'off',
    '@typescript-eslint/ban-ts-comment': 'off'
  },
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: ['./tsconfig.json', './tsconfig.node.json'],
    tsconfigRootDir: __dirname
  },
  settings: {
    react: {
      version: 'detect'
    }
  }
};

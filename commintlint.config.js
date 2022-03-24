module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'scope-enum': [2, 'always', ['client', 'api', 'linter', 'ci']],
    'scope-empty': [0, 'never'],
    'type-enum': [
      2,
      'always',
      [
        'chore',
        'feat',
        'fix',
        'docs',
        'perf',
        'style',
        'refactor',
        'test',
        'revert',
        'tools',
      ],
    ],
    'type-empty': [2, 'never'],
  },
};
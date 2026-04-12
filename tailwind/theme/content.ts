import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_CONTENT: ThemeConfig['content'] = {
  empty: "''",
  bash: "'$'",
  ['list-item']: "'• '",
} as const;

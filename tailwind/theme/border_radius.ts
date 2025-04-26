import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_BORDER_RADIUS: ThemeConfig['borderRadius'] = {
  2: '0.5rem',
  3: '0.75em',
  5: '1.25rem',
  10: '2.5rem',
  18: '4.5rem',
  25: '6.25rem',
  full: '9999px',
} as const;

import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_BORDER_RADIUS: ThemeConfig['borderRadius'] = {
  // TODO: les convertir tous en em
  2: '8px',
  3: '0.6em',
  5: '20px',
  10: '40px',
  18: '72px',
  25: '100px',
  full: '9999px',
} as const;

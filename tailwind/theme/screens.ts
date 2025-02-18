import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_SCREENS: ThemeConfig['screens'] = {
  'max-lg': {
    max: '1024px',
  },
  lg: {
    min: '1024px',
  },
  xl: {
    min: '1280px',
  },
} as const;

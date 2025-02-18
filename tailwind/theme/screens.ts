import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_SCREENS: ThemeConfig['screens'] = {
  xl: {
    min: '1280px',
  },
} as const;

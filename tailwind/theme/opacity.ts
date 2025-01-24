import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_OPACITY: ThemeConfig['opacity'] = {
  '0': '0',
  '30': '0.3',
  '33': '0.33',
  '50': '0.5',
  '66': '0.66',
  '90': '0.9',
  '100': '1',
} as const;

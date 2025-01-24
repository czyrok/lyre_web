import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_BLUR: ThemeConfig['blur'] = {
  // We must divide the Figma blue value by 2
  4: '4px',
  9: '9px',
  12: '12px',
} as const;

import { ThemeConfig } from 'tailwindcss/types/config';

export const EXTENDED_THEME_SIZE: {
  width: ThemeConfig['width'];
  minWidth: ThemeConfig['minWidth'];
  maxWidth: ThemeConfig['maxWidth'];
  height: ThemeConfig['height'];
  minHeight: ThemeConfig['minHeight'];
  maxHeight: ThemeConfig['maxHeight'];
} = {
  width: {
    'button-icon': '1.5em',
    'brand-badge-logo': '58%',
  },
  minWidth: {},
  maxWidth: {
    'landing-page-shape': '950px',
  },
  height: {
    'button-icon': '1.5em',
  },
  minHeight: {},
  maxHeight: {},
} as const;

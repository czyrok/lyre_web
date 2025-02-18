import { ThemeConfig } from 'tailwindcss/types/config';

export const EXTENDED_THEME_CONTAINER: ThemeConfig['container'] = {
  center: false,
  screens: {
    'home-section-text': {
      max: '103',
    },
  },
  padding: {
    'home-section': '6',
  },
} as const;

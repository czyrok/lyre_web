import { ThemeConfig } from 'tailwindcss/types/config';

export const EXTENDED_THEME_SIZE: {
  width: ThemeConfig['width'];
  height: ThemeConfig['height'];
} = {
  width: {
    'button-icon': '1.5em',
    'brand-badge-logo': '58%',
    7: '56px',
    13: '104px',
    33: '264px',
    41: '328px',
  },
  height: {
    'button-icon': '1.5em',
    4: '32px',
    7: '56px',
    28: '224px',
    33: '264px',
  },
} as const;

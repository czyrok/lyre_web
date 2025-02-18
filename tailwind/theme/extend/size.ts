import { ThemeConfig } from 'tailwindcss/types/config';

export const EXTENDED_THEME_SIZE: {
  width: ThemeConfig['width'];
  minWidth: ThemeConfig['minWidth'];
  height: ThemeConfig['height'];
  minHeight: ThemeConfig['minHeight'];
} = {
  width: {
    'button-icon': '1.5em',
    'brand-badge-logo': '58%',
    'contact-photo': '328px',
    7: '56px',
    13: '104px',
    33: '264px',
    41: '328px',
  },
  minWidth: {
    'contact-photo': '328px',
  },
  height: {
    'button-icon': '1.5em',
    'contact-photo': '328px',
    4: '32px',
    7: '56px',
    28: '224px',
    33: '264px',
  },
  minHeight: {
    'contact-photo': '328px',
  },
} as const;

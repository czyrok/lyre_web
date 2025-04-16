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
    'contact-photo': '328px',
    7: '56px',
    13: '104px',
    33: '264px',
    41: '328px',
  },
  minWidth: {
    'contact-photo': '328px',
  },
  maxWidth: {
    'landing-page-shape': '950px',
    62: '496px',
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
  maxHeight: {},
} as const;

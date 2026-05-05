import { ThemeConfig } from 'tailwindcss/types/config';

// By default, root EM is set to `16px`
// so, each value is from Figma but divided by 16 to have the value in EM

export const THEME_SPACING: ThemeConfig['spacing'] = {
  'middle-screen-shift': '-50dvh',
  'l7/1': '-7%',
  l4: '-2rem',
  0: '0rem',
  '0,5': '0.25rem',
  1: '0.5rem',
  '1,5': '0.75rem',
  2: '1rem',
  '2,5': '1.25rem',
  3: '1.5rem',
  '3,5': '1.75rem',
  4: '2rem',
  5: '2.5rem',
  6: '3rem',
  7: '3.5rem',
  8: '4rem',
  9: '4.5rem',
  10: '5rem',
  11: '5.5rem',
  14: '7rem',
  16: '8rem',
  25: '12.5rem',
  28: '14rem',
  33: '16.5rem',
  37: '18.5rem',
  41: '20.5rem',
  47: '23.5rem',
  48: '24rem',
  62: '31rem',
  103: '51.5rem',
  119: '59.5rem',
  210: '105rem',
  // Gap values to use them in margin
  level1: '1rem',
  level2: '2rem',
  level3: '4.5rem',
  level4: '7rem',
} as const;

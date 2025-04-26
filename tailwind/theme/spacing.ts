import { ThemeConfig } from 'tailwindcss/types/config';

// By default, root EM is set to `16px`
// so, each value is from Figma but divided by 16 to have the value in EM

export const THEME_SPACING: ThemeConfig['spacing'] = {
  'middle-screen-shift': '-50dvh',
  'l7/1': '-7%',
  l4: '-2em',
  0: '0em',
  '0,5': '0.25em',
  1: '0.5em',
  '1,5': '0.75em',
  2: '1em',
  '2,5': '1.25em',
  3: '1.5em',
  '3,5': '1.75em',
  4: '2em',
  5: '2.5em',
  6: '3em',
  7: '3.5em',
  8: '4em',
  9: '4.5em',
  10: '5em',
  11: '5.5em',
  14: '7em',
  16: '8em',
  25: '12.5em',
  28: '14em',
  33: '16.5em',
  37: '18.5em',
  41: '20.5em',
  47: '23.5em',
  62: '31em',
  103: '51.5em',
  119: '59.5em',
  210: '105em',
} as const;

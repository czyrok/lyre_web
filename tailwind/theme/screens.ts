import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_SCREENS: ThemeConfig['screens'] = {
  sm: {
    min: '640px',
  },
  'max-sm': {
    max: '640px',
  },
  'v-sm': {
    raw: '(min-height:640px)',
  },
  'max-v-sm': {
    raw: '(max-height:640px)',
  },
  md: {
    min: '768px',
  },
  'max-md': {
    max: '768px',
  },
  lg: {
    min: '1024px',
  },
  'max-lg': {
    max: '1024px',
  },
  xl: {
    min: '1280px',
  },
  'max-xl': {
    max: '1280px',
  },
  'v-xl': {
    raw: '(min-height:1280px)',
  },
  'max-v-xl': {
    raw: '(max-height:1280px)',
  },
  '2xl': {
    min: '1536px',
  },
  'max-2xl': {
    max: '1536px',
  },
} as const;

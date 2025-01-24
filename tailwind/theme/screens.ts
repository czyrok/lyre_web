import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_SCREENS: ThemeConfig['screens'] = {
  // TODO: a faire
  tablet: '640px',
  // => @media (min-width: 640px) { ... }
  laptop: '1024px',
  // => @media (min-width: 1024px) { ... }
  desktop: '1280px',
  // => @media (min-width: 1280px) { ... }
} as const;

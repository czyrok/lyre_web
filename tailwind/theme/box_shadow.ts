import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_BOX_SHADOW: ThemeConfig['boxShadow'] = {
  'button-inside': '0px 1px 4px 0px #000000 inset',
  'dropdown-outside': '0px 2px 4px 0px #00000025',
} as const;

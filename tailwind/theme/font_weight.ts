import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_FONT_WEIGHT: ThemeConfig['fontWeight'] = {
  'button-text': '530',
  'link-text': '530',
  'tag-text': '300',
  'brand-name': '600',
  'title-xl': '900',
  'title-lg': '600',
  //// There is a difference of 100 between here and Figma
  regular: '300',
  //// There is a difference of 200 between here and Figma
  light: '100',
} as const;

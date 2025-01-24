import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_FONT_WEIGHT: ThemeConfig['fontWeight'] = {
  'button-text': '530',
  'link-text': '530',
  'tag-text': '300',
  'brand-name': '600',
  'project-card-thumbnail-title': '400',
  'pagination-text': '400',
} as const;

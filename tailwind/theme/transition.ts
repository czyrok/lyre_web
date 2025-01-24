import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_TRANSITION: {
  transitionProperty: ThemeConfig['transitionProperty'];
  transitionDuration: ThemeConfig['transitionDuration'];
  transitionTimingFunction: ThemeConfig['transitionTimingFunction'];
} = {
  transitionProperty: {
    'button-background': 'opacity, background-color',
    'checkbox-icon': 'opacity',
    'link-text-and-icon': 'color',
    'project-thumbnail-background-and-border': 'opacity, outline-color',
    'nav-bar-item-background': 'background-color',
  },
  transitionDuration: {
    'button-background': '150ms',
    'checkbox-icon': '50ms',
    'link-text-and-icon': '150ms',
    'project-thumbnail-background-and-border': '150ms',
    'nav-bar-item-background': '150ms',
  },
  transitionTimingFunction: {
    'button-background': 'ease-in-out',
    'checkbox-icon': 'ease-in-out',
    'link-text-and-icon': 'ease-in-out',
    'project-thumbnail-background-and-border': 'ease-in-out',
    'nav-bar-item-background': 'ease-in-out',
  },
} as const;

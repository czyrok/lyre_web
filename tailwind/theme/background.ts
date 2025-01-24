import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_BACKGROUND: {
  backgroundSize: ThemeConfig['backgroundSize'];
  backgroundImage: ThemeConfig['backgroundImage'];
  backgroundPosition: ThemeConfig['backgroundPosition'];
} = {
  backgroundImage: {
    'bubble-pattern': "url('./button_rectangle_background.svg')",
    'button-gradient':
      'linear-gradient(100deg, var(--tw-gradient-from) 46%, var(--tw-gradient-to) 95.83%)',
  },
  backgroundSize: {
    'bubble-pattern': '4px 4px',
  },
  backgroundPosition: {
    'bubble-pattern': 'repeat',
  },
} as const;

import { ThemeConfig } from 'tailwindcss/types/config';

export const THEME_BACKGROUND: {
  backgroundSize: ThemeConfig['backgroundSize'];
  backgroundImage: ThemeConfig['backgroundImage'];
  backgroundPosition: ThemeConfig['backgroundPosition'];
} = {
  backgroundImage: {
    'bubble-pattern': "url('backgrounds/button_rectangle_background.svg')",
    'button-gradient':
      'linear-gradient(100deg, var(--tw-gradient-from) 46%, var(--tw-gradient-to) 95.83%)',
    'title-gradient':
      'linear-gradient(90deg, var(--tw-gradient-from) 0%, var(--tw-gradient-to) 100%)',
    'landing-page-cover-gradient':
      'linear-gradient(180deg, var(--tw-gradient-from) 0%, var(--tw-gradient-to) 82%)',
    'secondary-page-layout-cover-gradient':
      'linear-gradient(180deg, var(--tw-gradient-from) 0%, var(--tw-gradient-to) 100%)',
  },
  backgroundSize: {
    'bubble-pattern': '4px 4px',
  },
  backgroundPosition: {
    'bubble-pattern': 'repeat',
  },
} as const;

import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addSecondaryLinkComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.secondary-link': {
      // Children
      '.link-text': {
        // Colors
        [`@apply ${classPrefix}text-purple-blue-400`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}text-purple-blue-500`]: {},
        },

        // Others
        [`@apply ${classPrefix}font-link-text ${classPrefix}font-geist-mono`]: {},
        [`@apply ${classPrefix}transition-link-text-and-icon ${classPrefix}duration-link-text-and-icon ${classPrefix}ease-link-text-and-icon`]:
          {},
      },

      '.link-icon': {
        [`@apply ${classPrefix}inline-block`]: {},

        // Colors
        [`@apply ${classPrefix}text-purple-blue-400`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}text-purple-blue-500`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}w-button-icon ${classPrefix}h-button-icon`]: {},

        // Others
        [`@apply ${classPrefix}transition-link-text-and-icon ${classPrefix}duration-link-text-and-icon ${classPrefix}ease-link-text-and-icon`]:
          {},
      },

      '&:has(.link-text, .link-icon)': {
        [`@apply ${classPrefix}inline-flex ${classPrefix}items-center`]: {},
      },

      // Hover state
      '&:hover': {
        // Children
        '.link-text': {
          // Colors
          [`@apply ${classPrefix}text-purple-blue-300`]: {},
          [`${darkModeContext} &`]: {
            [`@apply ${classPrefix}text-purple-blue-400`]: {},
          },
        },

        '.link-icon': {
          // Colors
          [`@apply ${classPrefix}text-purple-blue-300`]: {},
          [`${darkModeContext} &`]: {
            [`@apply ${classPrefix}text-purple-blue-400`]: {},
          },
        },
      },

      // Focus state
      ':is(&:focus, a:focus &)': {
        // Colors
        [`@apply ${classPrefix}ring-green-400`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}ring-green-600`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}rounded-25`]: {},
        [`@apply ${classPrefix}ring-3 ${classPrefix}ring-offset-6`]: {},
      },

      // Variants
      '&.link-size-xl': {
        // Sizing
        [`@apply ${classPrefix}text-6`]: {},

        // Children
        '.link-icon svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },

        '&:has(.link-text, .link-icon)': {
          [`@apply ${classPrefix}gap-1,5`]: {},
        },

        '&:has(.link-icon):not(:has(.link-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.link-size-lg': {
        // Sizing
        [`@apply ${classPrefix}text-5`]: {},

        // Children
        '.link-icon svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },

        '&:has(.link-text, .link-icon)': {
          [`@apply ${classPrefix}gap-1`]: {},
        },

        '&:has(.link-icon):not(:has(.link-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.link-size-md': {
        // Sizing
        [`@apply ${classPrefix}text-4`]: {},

        // Children
        '.link-icon svg path': {
          [`@apply ${classPrefix}stroke-5`]: {},
        },

        '&:has(.link-text, .link-icon)': {
          [`@apply ${classPrefix}gap-1`]: {},
        },

        '&:has(.link-icon):not(:has(.link-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.link-size-sm': {
        // Sizing
        [`@apply ${classPrefix}text-3`]: {},

        // Children
        '.link-icon svg path': {
          [`@apply ${classPrefix}stroke-5`]: {},
        },

        '&:has(.link-text, .link-icon)': {
          [`@apply ${classPrefix}gap-0,5`]: {},
        },

        '&:has(.link-icon):not(:has(.link-text))': {
          [`@apply ${classPrefix}p-1,5`]: {},
        },
      },
    },
  });
};

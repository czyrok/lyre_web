import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addSecondaryButtonComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.secondary-button': {
      [`@apply ${classPrefix}inline-flex`]: {},

      // Colors
      [`@apply ${classPrefix}outline-purple-blue-400`]: {},
      [`${darkModeContext} &`]: {
        [`@apply ${classPrefix}outline-purple-blue-500`]: {},
      },

      // Sizing
      [`@apply ${classPrefix}rounded-25`]: {},
      [`@apply ${classPrefix}outline ${classPrefix}outline-offset-l2 ${classPrefix}outline-2`]: {},

      // Others
      [`@apply ${classPrefix}transition-button-background ${classPrefix}duration-button-background ${classPrefix}ease-button-background`]:
        {},

      // Children
      '.button-text': {
        // Colors
        [`@apply ${classPrefix}text-purple-blue-400`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}text-purple-blue-500`]: {},
        },

        // Others
        [`@apply ${classPrefix}font-button-text ${classPrefix}font-geist-mono`]: {},
      },

      '.button-icon': {
        [`@apply ${classPrefix}inline-block`]: {},

        // Colors
        [`@apply ${classPrefix}text-purple-blue-400`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}text-purple-blue-500`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}w-button-icon ${classPrefix}h-button-icon`]: {},
      },

      '&:has(.button-text):has(.button-icon)': {
        [`@apply ${classPrefix}items-center`]: {},
      },

      // Hover state
      '&:hover': {
        // Colors
        [`@apply ${classPrefix}bg-purple-blue-50/90`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}bg-purple-blue-950/90`]: {},
        },
      },

      // Focus state
      ':is(&:focus, a:focus &)': {
        // Colors
        [`@apply ${classPrefix}ring-green-400`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}ring-green-600`]: {},
        },
        //// This needs to correspond to page background
        [`@apply ${classPrefix}ring-offset-white-50`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}ring-offset-black-950`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}ring-3 ${classPrefix}ring-offset-3`]: {},
      },

      // Variants
      '&.button-size-xl': {
        // Sizing
        [`@apply ${classPrefix}px-5 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-6`]: {},

        // Children
        '.button-icon svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },

        '&:has(.button-text, .button-icon)': {
          [`@apply ${classPrefix}gap-1,5`]: {},
        },

        '&:has(.button-icon):not(:has(.button-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.button-size-lg': {
        // Sizing
        [`@apply ${classPrefix}px-4 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-5`]: {},

        // Children
        '.button-icon svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },

        '&:has(.button-text, .button-icon)': {
          [`@apply ${classPrefix}gap-1`]: {},
        },

        '&:has(.button-icon):not(:has(.button-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.button-size-md': {
        // Sizing
        [`@apply ${classPrefix}px-4 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-4`]: {},

        // Children
        '.button-icon svg path': {
          [`@apply ${classPrefix}stroke-5`]: {},
        },

        '&:has(.button-text, .button-icon)': {
          [`@apply ${classPrefix}gap-1`]: {},
        },

        '&:has(.button-icon):not(:has(.button-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },

        // Variants
        '&.button-ping': {
          '&::after': {
            // Sizing
            [`@apply ${classPrefix}w-1,5 ${classPrefix}h-1,5`]: {},
            [`@apply ${classPrefix}top-0,5 ${classPrefix}right-0,5`]: {},
          },
        },
      },

      '&.button-size-sm': {
        // Sizing
        [`@apply ${classPrefix}px-3 ${classPrefix}py-1,5`]: {},
        [`@apply ${classPrefix}text-3`]: {},

        // Children
        '.button-icon svg path': {
          [`@apply ${classPrefix}stroke-5`]: {},
        },

        '&:has(.button-text, .button-icon)': {
          [`@apply ${classPrefix}gap-0,5`]: {},
        },

        '&:has(.button-icon):not(:has(.button-text))': {
          [`@apply ${classPrefix}p-1,5`]: {},
        },
      },

      '&.button-ping': {
        [`@apply ${classPrefix}relative`]: {},

        '&::after': {
          [`@apply ${classPrefix}content-empty ${classPrefix}absolute ${classPrefix}animate-ping`]:
            {},

          // Colors
          [`@apply ${classPrefix}bg-purple-blue-400`]: {},
          [`${darkModeContext} &`]: {
            [`@apply ${classPrefix}bg-purple-blue-500`]: {},
          },

          // Sizing
          [`@apply ${classPrefix}rounded-full`]: {},
        },
      },
    },
  });
};

import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addPrimaryButtonComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.primary-button': {
      [`@apply ${classPrefix}inline-flex`]: {},

      [`@apply ${classPrefix}relative`]: {},
      // Ensure that content stays above the pseudo-element
      '& > *': {
        [`@apply ${classPrefix}relative ${classPrefix}z-10`]: {},
      },

      // Background image
      '&::after': {
        [`@apply ${classPrefix}content-empty ${classPrefix}inset-0 ${classPrefix}absolute ${classPrefix}z-0`]:
          {},

        // Background image
        [`@apply ${classPrefix}bg-bubble-pattern`]: {},

        // Sizing (required for the shadow)
        [`@apply ${classPrefix}rounded-25`]: {},

        // Others (we need to set here the shadow due to the z-index)
        [`@apply ${classPrefix}shadow-button-inside`]: {},
      },

      // Colors
      [`@apply ${classPrefix}bg-button-gradient`]: {},
      [`@apply ${classPrefix}from-purple-blue-400 ${classPrefix}to-purple-blue-500`]: {},
      [`${darkModeContext} &`]: {
        [`@apply ${classPrefix}from-purple-blue-700 ${classPrefix}to-purple-blue-800`]: {},
      },

      // Sizing (required for the background gradient)
      [`@apply ${classPrefix}rounded-25`]: {},

      // Hover pseudo element
      '&::before': {
        [`@apply ${classPrefix}content-empty ${classPrefix}inset-0 ${classPrefix}absolute ${classPrefix}z-0 ${classPrefix}opacity-0`]:
          {},

        // Colors
        [`@apply ${classPrefix}bg-purple-blue-500`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}bg-purple-blue-800`]: {},
        },

        // Sizing (required for the background color)
        [`@apply ${classPrefix}rounded-25`]: {},

        // Others
        [`@apply ${classPrefix}transition-button-background ${classPrefix}duration-button-background ${classPrefix}ease-button-background`]:
          {},
      },

      // Children
      '.button-text': {
        // Colors
        [`@apply ${classPrefix}text-purple-blue-50`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}text-purple-blue-100`]: {},
        },

        // Others
        [`@apply ${classPrefix}font-button-text ${classPrefix}font-geist-mono`]: {},
      },

      '.button-icon': {
        [`@apply ${classPrefix}inline-block`]: {},

        // Colors
        [`@apply ${classPrefix}text-purple-blue-50`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}text-purple-blue-100`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}w-button-icon ${classPrefix}h-button-icon`]: {},

        // Children
        '& svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },
      },

      '&:has(.button-text):has(.button-icon)': {
        [`@apply ${classPrefix}items-center`]: {},
      },

      // Hover state
      '&:hover::before': {
        [`@apply ${classPrefix}opacity-100`]: {},
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
    },
  });
};

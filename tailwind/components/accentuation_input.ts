import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addAccentuationInputComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    // TODO: width !!!!
    '.accentuation-input': {
      [`@apply ${classPrefix}relative`]: {},
      // Ensure that content stays above the pseudo-element
      '& > *': {
        [`@apply ${classPrefix}relative ${classPrefix}z-10`]: {},
      },

      // Background image
      '&::before': {
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
      [`@apply ${classPrefix}bg-green-100`]: {},
      [darkModeContext]: {
        [`@apply ${classPrefix}bg-green-700`]: {},
      },

      // Sizing (required for the background gradient)
      [`@apply ${classPrefix}rounded-25`]: {},

      // Children
      '.input-text': {
        [`@apply ${classPrefix}bg-transparent`]: {},
        '&:focus': {
          [`@apply ${classPrefix}outline-none`]: {},
        },

        // Colors
        '&::placeholder': {
          [`@apply ${classPrefix}text-green-950/30`]: {},
          [darkModeContext]: {
            [`@apply ${classPrefix}text-green-950/30`]: {},
          },
        },
        [`@apply ${classPrefix}text-green-950`]: {},
        [darkModeContext]: {
          [`@apply ${classPrefix}text-green-950`]: {},
        },

        // Others
        [`@apply ${classPrefix}font-button-text ${classPrefix}font-geist-mono`]: {},
      },

      '.input-icon': {
        [`@apply ${classPrefix}inline-block`]: {},

        // Colors
        [`@apply ${classPrefix}text-green-950`]: {},
        [darkModeContext]: {
          [`@apply ${classPrefix}text-green-950`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}w-button-icon ${classPrefix}h-button-icon`]: {},

        // Children
        '& svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },
      },

      '&:has(.input-text, .input-icon)': {
        [`@apply ${classPrefix}inline-flex ${classPrefix}items-center`]: {},
      },

      // Focus state
      '&:has(.input-text:focus)': {
        // Colors
        [`@apply ${classPrefix}ring-green-400`]: {},
        [darkModeContext]: {
          [`@apply ${classPrefix}ring-green-600`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}ring-3 ${classPrefix}ring-offset-3`]: {},
      },

      // Variants
      '&.input-size-xl': {
        // Sizing
        [`@apply ${classPrefix}px-5 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-6`]: {},

        // Children
        '.input-icon svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },

        '&:has(.input-text, .input-icon)': {
          [`@apply ${classPrefix}gap-1,5`]: {},
        },

        '&:has(.input-icon):not(:has(.input-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.input-size-lg': {
        // Sizing
        [`@apply ${classPrefix}px-4 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-5`]: {},

        // Children
        '.input-icon svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },

        '&:has(.input-text, .input-icon)': {
          [`@apply ${classPrefix}gap-1`]: {},
        },

        '&:has(.input-icon):not(:has(.input-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.input-size-md': {
        // Sizing
        [`@apply ${classPrefix}px-4 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-4`]: {},

        // Children
        '.input-icon svg path': {
          [`@apply ${classPrefix}stroke-5`]: {},
        },

        '&:has(.input-text, .input-icon)': {
          [`@apply ${classPrefix}gap-1`]: {},
        },

        '&:has(.input-icon):not(:has(.input-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.input-size-sm': {
        // Sizing
        [`@apply ${classPrefix}px-3 ${classPrefix}py-1,5`]: {},
        [`@apply ${classPrefix}text-3`]: {},

        // Children
        '.input-icon svg path': {
          [`@apply ${classPrefix}stroke-5`]: {},
        },

        '&:has(.input-text, .input-icon)': {
          [`@apply ${classPrefix}gap-0,5`]: {},
        },

        '&:has(.input-icon):not(:has(.input-text))': {
          [`@apply ${classPrefix}p-1,5`]: {},
        },
      },
    },
  });
};

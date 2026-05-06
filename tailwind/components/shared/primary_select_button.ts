import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addPrimarySelectButtonComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.primary-select': {
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
      [`@apply dark:${classPrefix}from-purple-blue-700 dark:${classPrefix}to-purple-blue-800`]: {},

      // Sizing (required for the background gradient)
      [`@apply ${classPrefix}rounded-25`]: {},

      // Hover pseudo element
      '&::before': {
        [`@apply ${classPrefix}content-empty ${classPrefix}inset-0 ${classPrefix}absolute ${classPrefix}z-0 ${classPrefix}opacity-0`]:
          {},

        // Colors
        [`@apply ${classPrefix}bg-purple-blue-500`]: {},
        [`@apply dark:${classPrefix}bg-purple-blue-800`]: {},

        // Sizing (required for the background color)
        [`@apply ${classPrefix}rounded-25`]: {},

        // Others
        [`@apply ${classPrefix}transition-button-background ${classPrefix}duration-button-background ${classPrefix}ease-button-background`]:
          {},
      },

      // Children
      '.select-text': {
        // Colors
        [`@apply ${classPrefix}text-purple-blue-50`]: {},
        [`@apply dark:${classPrefix}text-purple-blue-100`]: {},

        // Others
        [`@apply ${classPrefix}font-button-text ${classPrefix}font-geist-mono`]: {},
      },

      '.select-icon': {
        [`@apply ${classPrefix}inline-block`]: {},

        // Colors
        [`@apply ${classPrefix}text-purple-blue-50`]: {},
        [`@apply dark:${classPrefix}text-purple-blue-100`]: {},

        // Sizing
        [`@apply ${classPrefix}w-button-icon ${classPrefix}h-button-icon`]: {},

        // Children
        '& svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },
      },

      '.select-left-group': {
        [`@apply ${classPrefix}inline-flex`]: {},
      },

      '&:has(.select-text):has(.select-icon)': {
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
        [`@apply dark:${classPrefix}ring-green-600`]: {},

        //// This needs to correspond to page background
        [`@apply ${classPrefix}ring-offset-white-50`]: {},
        //// This needs to correspond to page background
        [`@apply dark:${classPrefix}ring-offset-black-950`]: {},

        // Sizing
        [`@apply ${classPrefix}ring-3 ${classPrefix}ring-offset-3`]: {},
      },

      // Variants
      '&.select-size-xl': {
        // Sizing
        [`@apply ${classPrefix}px-4 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-6`]: {},

        // Children
        '.select-icon svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },

        '&:has(.select-text, .select-icon)': {
          [`@apply ${classPrefix}gap-2,5`]: {},
        },

        '.select-left-group': {
          [`@apply ${classPrefix}gap-1,5`]: {},
        },

        '&:has(.select-icon):not(:has(.select-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.select-size-lg': {
        // Sizing
        [`@apply ${classPrefix}px-3,5 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-5`]: {},

        // Children
        '.select-icon svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },

        '&:has(.select-text, .select-icon)': {
          [`@apply ${classPrefix}gap-2`]: {},
        },

        '.select-left-group': {
          [`@apply ${classPrefix}gap-1`]: {},
        },

        '&:has(.select-icon):not(:has(.select-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.select-size-md': {
        // Sizing
        [`@apply ${classPrefix}px-3 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-4`]: {},

        // Children
        '.select-icon svg path': {
          [`@apply ${classPrefix}stroke-5`]: {},
        },

        '&:has(.select-text, .select-icon)': {
          [`@apply ${classPrefix}gap-2`]: {},
        },

        '.select-left-group': {
          [`@apply ${classPrefix}gap-1`]: {},
        },

        '&:has(.select-icon):not(:has(.select-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.select-size-sm': {
        // Sizing
        [`@apply ${classPrefix}px-2,5 ${classPrefix}py-1,5`]: {},
        [`@apply ${classPrefix}text-3`]: {},

        // Children
        '.select-icon svg path': {
          [`@apply ${classPrefix}stroke-5`]: {},
        },

        '&:has(.select-text, .select-icon)': {
          [`@apply ${classPrefix}gap-1,5`]: {},
        },

        '.select-left-group': {
          [`@apply ${classPrefix}gap-0,5`]: {},
        },

        '&:has(.select-icon):not(:has(.select-text))': {
          [`@apply ${classPrefix}p-1,5`]: {},
        },
      },
    },
  });
};

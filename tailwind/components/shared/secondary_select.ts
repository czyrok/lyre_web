import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addSecondarySelectComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.secondary-select': {
      // Colors
      [`@apply ${classPrefix}outline-purple-blue-400`]: {},
      [`@apply dark:${classPrefix}outline-purple-blue-500`]: {},

      // Sizing
      [`@apply ${classPrefix}rounded-25`]: {},
      [`@apply ${classPrefix}outline ${classPrefix}outline-offset-l2 ${classPrefix}outline-2`]: {},

      // Others
      [`@apply ${classPrefix}transition-button-background ${classPrefix}duration-button-background ${classPrefix}ease-button-background`]:
        {},

      // Children
      '& select': {
        [`@apply ${classPrefix}hidden`]: {},
      },

      '.select-text': {
        // Colors
        [`@apply ${classPrefix}text-purple-blue-400`]: {},
        [`@apply dark:${classPrefix}text-purple-blue-500`]: {},

        // Others
        [`@apply ${classPrefix}font-button-text ${classPrefix}font-geist-mono`]: {},
      },

      '.select-icon': {
        [`@apply ${classPrefix}inline-block`]: {},

        // Colors
        [`@apply ${classPrefix}text-purple-blue-400`]: {},
        [`@apply dark:${classPrefix}text-purple-blue-500`]: {},

        // Sizing
        [`@apply ${classPrefix}w-button-icon ${classPrefix}h-button-icon`]: {},
      },

      '&:has(.select-text, .select-icon)': {
        [`@apply ${classPrefix}inline-flex ${classPrefix}items-center`]: {},
      },

      // Focus state
      '&:focus': {
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
        [`@apply ${classPrefix}px-5 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-6`]: {},

        // Children
        '.select-icon svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },

        '&:has(.select-text, .select-icon)': {
          [`@apply ${classPrefix}gap-1,5`]: {},
        },

        '&:has(.select-icon):not(:has(.select-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.select-size-lg': {
        // Sizing
        [`@apply ${classPrefix}px-4 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-5`]: {},

        // Children
        '.select-icon svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },

        '&:has(.select-text, .select-icon)': {
          [`@apply ${classPrefix}gap-1`]: {},
        },

        '&:has(.select-icon):not(:has(.select-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.select-size-md': {
        // Sizing
        [`@apply ${classPrefix}px-4 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-4`]: {},

        // Children
        '.select-icon svg path': {
          [`@apply ${classPrefix}stroke-5`]: {},
        },

        '&:has(.select-text, .select-icon)': {
          [`@apply ${classPrefix}gap-1`]: {},
        },

        '&:has(.select-icon):not(:has(.select-text))': {
          [`@apply ${classPrefix}p-2`]: {},
        },
      },

      '&.select-size-sm': {
        // Sizing
        [`@apply ${classPrefix}px-3 ${classPrefix}py-1,5`]: {},
        [`@apply ${classPrefix}text-3`]: {},

        // Children
        '.select-icon svg path': {
          [`@apply ${classPrefix}stroke-5`]: {},
        },

        '&:has(.select-text, .select-icon)': {
          [`@apply ${classPrefix}gap-0,5`]: {},
        },

        '&:has(.select-icon):not(:has(.select-text))': {
          [`@apply ${classPrefix}p-1,5`]: {},
        },
      },
    },
  });
};

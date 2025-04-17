import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addSecondaryCheckboxComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.secondary-checkbox': {
      [`@apply ${classPrefix}inline-flex ${classPrefix}items-center`]: {},
      [`@apply ${classPrefix}select-none`]: {},

      // Children
      '.checkbox-input': {
        //// To make the focus works, we must use opacity 0 instead of visibility `hidden`
        [`@apply ${classPrefix}opacity-0 ${classPrefix}z-l1 ${classPrefix}w-0 ${classPrefix}h-0 ${classPrefix}absolute`]:
          {},
      },

      '.checkbox-box': {
        [`@apply ${classPrefix}inline-flex`]: {},

        // Colors
        [`@apply ${classPrefix}outline-purple-blue-400`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}outline-purple-blue-500`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}rounded-3`]: {}, // Not the same in Figma
        [`@apply ${classPrefix}outline ${classPrefix}outline-offset-l2 ${classPrefix}outline-2`]:
          {},

        // Others
        [`@apply ${classPrefix}transition-button-background ${classPrefix}duration-button-background ${classPrefix}ease-button-background`]:
          {},

        // Children
        '.box-icon': {
          [`@apply ${classPrefix}inline-block`]: {},

          // Colors
          [`@apply ${classPrefix}text-purple-blue-400`]: {},
          [`${darkModeContext} &`]: {
            [`@apply ${classPrefix}text-purple-blue-500`]: {},
          },

          // Sizing
          [`@apply ${classPrefix}w-button-icon ${classPrefix}h-button-icon`]: {},

          // Others
          [`@apply ${classPrefix}transition-checkbox-icon ${classPrefix}duration-checkbox-icon ${classPrefix}ease-checkbox-icon`]:
            {},

          // Default state
          [`@apply ${classPrefix}opacity-0`]: {},
        },
      },

      '.checkbox-text': {
        // Colors
        [`@apply ${classPrefix}text-black-700`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}text-white-200`]: {},
        },

        // Others
        [`@apply ${classPrefix}font-button-text ${classPrefix}font-geist-mono`]: {},
      },

      // Checked state
      '&:has(.checkbox-input:checked) .checkbox-box': {
        // Colors
        [`@apply ${classPrefix}bg-purple-blue-50/90`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}bg-purple-blue-950/90`]: {},
        },

        // Children
        '.box-icon': {
          [`@apply ${classPrefix}opacity-100`]: {},
        },
      },

      // Focus state
      '&:has(.checkbox-input:focus) .checkbox-box': {
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
      '&.checkbox-size-xl': {
        // Sizing
        [`@apply ${classPrefix}text-8`]: {},
        [`@apply ${classPrefix}gap-2,5`]: {},

        // Children
        '.checkbox-box': {
          [`@apply ${classPrefix}p-1`]: {},

          // Children
          'svg path': {
            [`@apply ${classPrefix}stroke-3`]: {},
          },
        },
      },

      '&.checkbox-size-lg': {
        // Sizing
        [`@apply ${classPrefix}text-6`]: {},
        [`@apply ${classPrefix}gap-2`]: {},

        // Children
        '.checkbox-box': {
          [`@apply ${classPrefix}p-1`]: {},

          // Children
          'svg path': {
            [`@apply ${classPrefix}stroke-3`]: {},
          },
        },
      },

      '&.checkbox-size-md': {
        // Sizing
        [`@apply ${classPrefix}text-5`]: {},
        [`@apply ${classPrefix}gap-2`]: {},

        // Children
        '.checkbox-box': {
          [`@apply ${classPrefix}p-1`]: {},

          // Children
          'svg path': {
            [`@apply ${classPrefix}stroke-4`]: {},
          },
        },
      },

      '&.checkbox-size-sm': {
        // Sizing
        [`@apply ${classPrefix}text-4`]: {},
        [`@apply ${classPrefix}gap-2`]: {},

        // Children
        '.checkbox-box': {
          [`@apply ${classPrefix}p-1`]: {},

          // Children
          'svg path': {
            [`@apply ${classPrefix}stroke-4`]: {},
          },
        },
      },
    },
  });
};

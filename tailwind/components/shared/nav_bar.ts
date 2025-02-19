import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addNavBarComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.nav-bar': {
      [`@apply ${classPrefix}relative ${classPrefix}w-fit ${classPrefix}flex`]: {},
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
      },

      // Colors
      [`@apply ${classPrefix}bg-purple-blue-400/30`]: {},
      [darkModeContext]: {
        [`@apply ${classPrefix}bg-purple-blue-700/30`]: {},
      },

      // Sizing
      [`@apply ${classPrefix}rounded-25`]: {},

      // Others
      [`@apply ${classPrefix}backdrop-blur-12`]: {},

      // Children
      '.nav-bar-item': {
        // Sizing
        [`@apply ${classPrefix}px-4 ${classPrefix}py-2`]: {},
        [`@apply ${classPrefix}text-5`]: {},
        [`@apply ${classPrefix}rounded-25`]: {},

        // Others
        [`@apply ${classPrefix}transition-nav-bar-item-background ${classPrefix}duration-nav-bar-item-background ${classPrefix}ease-nav-bar-item-background`]:
          {},

        // Hover state
        '&:hover': {
          // Colors
          [`@apply ${classPrefix}bg-purple-blue-300/66`]: {},
          [darkModeContext]: {
            [`@apply ${classPrefix}bg-purple-blue-800/66`]: {},
          },
        },

        // Children
        '.item-text': {
          // Colors
          [`@apply ${classPrefix}text-purple-blue-800`]: {},
          [darkModeContext]: {
            [`@apply ${classPrefix}text-purple-blue-300`]: {},
          },

          // Others
          [`@apply ${classPrefix}font-button-text ${classPrefix}font-geist-mono`]: {},
        },

        '.item-icon': {
          [`@apply ${classPrefix}inline-block`]: {},

          // Colors
          [`@apply ${classPrefix}text-purple-blue-800`]: {},
          [darkModeContext]: {
            [`@apply ${classPrefix}text-purple-blue-300`]: {},
          },

          // Sizing
          [`@apply ${classPrefix}w-button-icon ${classPrefix}h-button-icon`]: {},

          // Children
          '& svg path': {
            [`@apply ${classPrefix}stroke-4`]: {},
          },
        },

        '&:has(.item-text, .item-icon)': {
          [`@apply ${classPrefix}inline-flex ${classPrefix}items-center ${classPrefix}gap-1`]: {},
        },
      },

      // Active page state for children
      '[aria-current="page"] .nav-bar-item': {
        // Children
        '.item-text': {
          // Colors
          [`@apply ${classPrefix}text-purple-blue-600`]: {},
          [darkModeContext]: {
            [`@apply ${classPrefix}text-purple-blue-50`]: {},
          },
        },

        '.item-icon': {
          // Colors
          [`@apply ${classPrefix}text-purple-blue-600`]: {},
          [darkModeContext]: {
            [`@apply ${classPrefix}text-purple-blue-50`]: {},
          },
        },
      },

      // Focus state for a child
      'a:focus .nav-bar-item': {
        // Colors
        [`@apply ${classPrefix}outline-green-400`]: {},
        [darkModeContext]: {
          [`@apply ${classPrefix}outline-green-600`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}outline ${classPrefix}outline-3 ${classPrefix}outline-offset-3`]: {},
      },
    },
  });
};

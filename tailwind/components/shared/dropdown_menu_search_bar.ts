import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addDropdownMenuSearchBarComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.dropdown-menu-search-bar': {
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
        [`@apply ${classPrefix}rounded-3`]: {},

        // Others (we need to set here the shadow due to the z-index)
        [`@apply ${classPrefix}shadow-button-inside`]: {},
      },

      // Colors
      [`@apply ${classPrefix}bg-purple-blue-300/80`]: {},
      [`@apply dark:${classPrefix}bg-purple-blue-700/80`]: {},

      // Sizing
      [`@apply ${classPrefix}px-2,5 ${classPrefix}py-2`]: {},
      [`@apply ${classPrefix}text-4`]: {},

      // Sizing (required for the background gradient)
      [`@apply ${classPrefix}rounded-3`]: {},

      // Children
      '.search-bar-text-input': {
        [`@apply ${classPrefix}bg-transparent ${classPrefix}w-full`]: {},
        '&:focus': {
          [`@apply ${classPrefix}outline-none`]: {},
        },

        // Colors
        '&::placeholder': {
          [`@apply ${classPrefix}text-purple-blue-950/50`]: {},
          [`@apply dark:${classPrefix}text-purple-blue-100/50`]: {},
        },
        [`@apply ${classPrefix}text-purple-blue-950`]: {},
        [`@apply dark:${classPrefix}text-purple-blue-100`]: {},

        // Others
        [`@apply ${classPrefix}font-button-text ${classPrefix}font-geist-mono`]: {},
      },

      '.search-bar-text-icon': {
        [`@apply ${classPrefix}inline-block`]: {},

        // Colors
        [`@apply ${classPrefix}text-purple-blue-950`]: {},
        [`@apply dark:${classPrefix}text-purple-blue-100`]: {},

        // Sizing
        [`@apply ${classPrefix}w-button-icon ${classPrefix}h-button-icon`]: {},

        // Children
        '& svg path': {
          [`@apply ${classPrefix}stroke-4`]: {},
        },
      },

      '&:has(.search-bar-text-input, .search-bar-text-icon)': {
        [`@apply ${classPrefix}inline-flex ${classPrefix}items-center`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-1`]: {},
      },

      // Focus state
      '&:has(.search-bar-text-input:focus)': {
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
    },
  });
};

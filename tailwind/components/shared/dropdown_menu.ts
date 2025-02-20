import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addDropdownMenuComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.dropdown-menu': {
      [`@apply ${classPrefix}absolute ${classPrefix}flex-col`]: {},
      //// Positioning
      [`@apply ${classPrefix}inset-auto`]: {},

      // Colors
      [`@apply ${classPrefix}bg-white-100/50`]: {},
      [darkModeContext]: {
        [`@apply ${classPrefix}bg-black-600/50`]: {},
      },

      // Sizing
      [`@apply ${classPrefix}py-1`]: {},
      [`@apply ${classPrefix}rounded-5`]: {},

      // Others
      [`@apply ${classPrefix}backdrop-blur-9`]: {},
      [`@apply ${classPrefix}shadow-dropdown-menu-outside`]: {},

      // Opened state
      [`&:popover-open`]: {
        //// We set the display here to avoid to interfere
        //// with the default behavior of the popover
        [`@apply ${classPrefix}inline-flex`]: {},
      },

      // Children
      '.dropdown-menu-item': {
        // Sizing
        [`@apply ${classPrefix}px-2 ${classPrefix}py-1`]: {},
      },

      // Variants
      [`&.dropdown-menu-right`]: {
        // Positioning
        [`@apply ${classPrefix}top-[anchor(top)] ${classPrefix}left-[anchor(right)]`]: {},
        [`@apply ${classPrefix}ms-3`]: {},
      },
    },
  });
};

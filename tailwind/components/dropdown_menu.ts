import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addDropdownMenuComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.dropdown-menu': {
      [`@apply ${classPrefix}inline-flex ${classPrefix}flex-col`]: {},

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

      // Children
      '.dropdown-menu-item': {
        // Sizing
        [`@apply ${classPrefix}px-2 ${classPrefix}py-1`]: {},
      },
    },
  });
};

import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addDropdownComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.dropdown': {
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
      [`@apply ${classPrefix}shadow-dropdown-outside`]: {},

      // Children
      '.dropdown-item': {
        // Sizing
        [`@apply ${classPrefix}px-2 ${classPrefix}py-1`]: {},
      },
    },
  });
};

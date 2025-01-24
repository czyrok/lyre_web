import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addTagComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.tag': {
      // Colors
      [`@apply ${classPrefix}bg-blue-300`]: {},
      [darkModeContext]: {
        [`@apply ${classPrefix}bg-blue-400`]: {},
      },
      [`@apply ${classPrefix}text-blue-700`]: {},
      [darkModeContext]: {
        [`@apply ${classPrefix}text-blue-950`]: {},
      },

      // Sizing
      [`@apply ${classPrefix}px-3 ${classPrefix}py-1`]: {},
      [`@apply ${classPrefix}text-4`]: {},
      [`@apply ${classPrefix}rounded-25`]: {},

      // Others
      [`@apply ${classPrefix}font-tag-text ${classPrefix}font-geist`]: {},
    },
  });
};

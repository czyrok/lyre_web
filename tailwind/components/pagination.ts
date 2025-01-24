import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addPaginationComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.pagination': {
      [`@apply ${classPrefix}w-fit ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center`]:
        {},

      // Sizing
      [`@apply ${classPrefix}gap-level1`]: {},

      '.pagination-text': {
        // Colors
        [`@apply ${classPrefix}text-black-950`]: {},
        [darkModeContext]: {
          [`@apply ${classPrefix}text-white-50`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}text-4`]: {},

        // Others
        [`@apply ${classPrefix}font-pagination-text ${classPrefix}font-geist`]: {},
      },
    },
  });
};

import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addProjectSearchPage: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.secondary-page-layout': {
      // Children
      '.project-search-page-top-part': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level2`]: {},

        // Children
        '.top-part-filter': {
          [`@apply ${classPrefix}flex ${classPrefix}flex-wrap ${classPrefix}flex-row`]: {},

          // Sizing
          [`@apply ${classPrefix}gap-level1`]: {},
        },
      },

      '.project-search-page-middle-part': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level3`]: {},

        // Children
        '.middle-part-info': {
          [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center`]: {},

          // Sizing
          [`@apply ${classPrefix}gap-level2`]: {},
        },

        '.middle-part-list': {
          [`@apply ${classPrefix}flex ${classPrefix}flex-wrap ${classPrefix}justify-center`]: {},

          // Sizing
          [`@apply ${classPrefix}gap-level2`]: {},
        },
      },

      '.project-search-page-bottom-part': {
        [`@apply ${classPrefix}flex ${classPrefix}justify-center`]: {},
      },
    },
  });
};

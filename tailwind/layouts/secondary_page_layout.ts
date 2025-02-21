import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addSecondaryPageLayout: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix, darkModeContext }
) => {
  addComponents({
    '.secondary-page-layout': {
      // Children
      '.secondary-page-layout-intro': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center`]: {},

        // Colors
        [`@apply ${classPrefix}bg-secondary-page-layout-cover-gradient`]: {},
        [`@apply ${classPrefix}from-blue-100 ${classPrefix}to-white-50`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}from-blue-900 ${classPrefix}to-black-950`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}pt-16 ${classPrefix}pb-4`]: {},
      },

      '.secondary-page-layout-content': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

        // Sizing
        // TODO: responsive padding
        [`@apply ${classPrefix}px-14 ${classPrefix}py-6 ${classPrefix}mb-16`]: {},
        [`@apply ${classPrefix}gap-level4`]: {},
      },
    },
  });
};

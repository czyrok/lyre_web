import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addProjectCardComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.project-card': {
      [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

      // Sizing
      [`@apply ${classPrefix}p-2`]: {},
      [`@apply ${classPrefix}gap-1`]: {},

      '.card-title': {
        // Colors
        [`@apply ${classPrefix}text-black-950`]: {},
        [darkModeContext]: {
          [`@apply ${classPrefix}text-white-50`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}text-7`]: {},

        // Others
        [`@apply ${classPrefix}font-project-card-thumbnail-title ${classPrefix}font-geist-mono`]:
          {},
      },

      '.card-tag-container': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-row`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-1`]: {},
      },
    },
  });
};

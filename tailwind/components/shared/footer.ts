import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addFooterComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.footer': {
      [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

      // Sizing
      [`@apply ${classPrefix}gap-level4`]: {},
      [`@apply ${classPrefix}p-4`]: {},

      // Children
      '.footer-more-part': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level2`]: {},

        // Children
        '.more-part-text': {
          [`@apply ${classPrefix}text-center`]: {},

          // Colors
          [`@apply ${classPrefix}text-black-950`]: {},
          [darkModeContext]: {
            [`@apply ${classPrefix}text-white-50`]: {},
          },

          // Sizing
          [`@apply ${classPrefix}text-6`]: {},

          // Others
          [`@apply ${classPrefix}font-regular ${classPrefix}font-geist`]: {},
        },

        '.more-part-actions': {
          [`@apply ${classPrefix}flex ${classPrefix}flex-wrap ${classPrefix}flex-row ${classPrefix}justify-center`]:
            {},

          // Sizing
          [`@apply ${classPrefix}gap-level1`]: {},
        },
      },

      '.footer-bottom-part': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level2`]: {},

        // Children
        '.bottom-part-separator': {
          [`@apply ${classPrefix}border-t-footer-separator`]: {},

          // Colors
          [`@apply ${classPrefix}text-white-200`]: {},
          [darkModeContext]: {
            [`@apply ${classPrefix}text-black-800`]: {},
          },
        },

        '.bottom-part-settings': {
          [`@apply ${classPrefix}flex ${classPrefix}flex-wrap ${classPrefix}flex-row ${classPrefix}justify-between`]:
            {},

          // Sizing
          [`@apply ${classPrefix}gap-level1`]: {},
        },

        '.bottom-part-copyright-text': {
          // Colors
          [`@apply ${classPrefix}text-black-950`]: {},
          [darkModeContext]: {
            [`@apply ${classPrefix}text-white-50`]: {},
          },

          // Sizing
          [`@apply ${classPrefix}text-5`]: {},

          // Others
          [`@apply ${classPrefix}font-light ${classPrefix}font-geist`]: {},
        },
      },
    },
  });
};

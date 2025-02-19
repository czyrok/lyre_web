import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addSectionContainerComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.landing-page-section-container': {
      [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center`]: {},

      // Sizing
      [`@apply ${classPrefix}p-6`]: {},
      [`@apply ${classPrefix}gap-level3`]: {},

      // Children
      '.section-text': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

        // Sizing
        [`@apply ${classPrefix}max-w-103`]: {},
        [`@apply ${classPrefix}gap-level2`]: {},
      },

      '.section-actions': {
        [`@apply ${classPrefix}w-full ${classPrefix}flex ${classPrefix}flex-wrap ${classPrefix}flex-row ${classPrefix}justify-center`]:
          {},

        // Sizing
        [`@apply ${classPrefix}gap-level1`]: {},
      },
    },
  });
};

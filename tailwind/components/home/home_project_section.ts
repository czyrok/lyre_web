import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addHomeProjectSectionComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.project-section': {
      [`@apply ${classPrefix}container-home-section ${classPrefix}mx-auto ${classPrefix}flex ${classPrefix}flex-col`]:
        {},

      // Sizing
      [`@apply ${classPrefix}gap-level3`]: {},

      // Children
      '.section-text': {
        [`@apply ${classPrefix}container-home-section-text ${classPrefix}flex ${classPrefix}flex-col`]:
          {},

        // Sizing
        [`@apply ${classPrefix}gap-level2`]: {},
      },

      '.section-projects': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-wrap`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level2`]: {},
      },

      '.section-actions': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-row`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level1`]: {},
      },
    },
  });
};

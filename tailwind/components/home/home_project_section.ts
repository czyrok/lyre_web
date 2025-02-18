import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addHomeProjectSectionComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.home-project-section': {
      // Children
      '.section-projects': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-wrap ${classPrefix}justify-center`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level2`]: {},
      },
    },
  });
};

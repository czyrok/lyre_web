import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addProjectSectionComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.landing-page-project-section': {
      // Children
      '.section-projects': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-wrap ${classPrefix}justify-center`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level2`]: {},
      },
    },
  });
};

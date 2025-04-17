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
        [`@apply ${classPrefix}container-level2`]: {},
      },

      // Overrides
      '.anchor': {
        // Apply variants
        [`@apply v-xl:xl:${classPrefix}anchor-middle-screen-target`]: {},
        [`@apply max-xl:${classPrefix}anchor-top-target`]: {},
        [`@apply max-v-xl:${classPrefix}anchor-top-target`]: {},
      },
    },
  });
};

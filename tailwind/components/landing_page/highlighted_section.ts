import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addHighlightedSectionComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix, darkModeContext }
) => {
  addComponents({
    '.landing-page-highlighted-section': {
      // Colors
      [`@apply ${classPrefix}bg-blue-100`]: {},
      [`${darkModeContext} &`]: {
        [`@apply ${classPrefix}bg-blue-950`]: {},
      },

      // Sizing
      [`@apply ${classPrefix}py-6 sm:${classPrefix}py-7 md:${classPrefix}py-10`]: {},

      // Children
      '.section-text-pair': {
        //// The use of media queries replaces the trick of using min and max on texts in Figma
        [`@apply ${classPrefix}flex ${classPrefix}flex-col xl:${classPrefix}flex-row`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level3`]: {},
      },

      // Overrides
      '.section-text': {
        [`@apply ${classPrefix}max-w-210`]: {},
      },
      '.anchor': {
        // Apply variants
        [`@apply v-sm:xl:${classPrefix}anchor-middle-screen-target`]: {},
        [`@apply max-xl:${classPrefix}anchor-top-target`]: {},
        [`@apply max-v-sm:${classPrefix}anchor-top-target`]: {},
      },
    },
  });
};

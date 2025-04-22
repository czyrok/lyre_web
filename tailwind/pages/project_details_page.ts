import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addProjectDetailsPage: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.secondary-page-layout': {
      // Children
      '.project-details-page-top-part': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level2`]: {},

        // Children
        '.top-part-intro': {
          // Children
          '.intro-details': {
            [`@apply ${classPrefix}flex ${classPrefix}flex-wrap`]: {},

            // Sizing
            [`@apply ${classPrefix}x-container-level2`]: {},

            // Children
            // TODO: bug de responsive revient à la ligne puis revient
            '.details-tags': {
              [`@apply ${classPrefix}flex ${classPrefix}flex-wrap ${classPrefix}items-center`]: {},

              // Sizing
              [`@apply ${classPrefix}gap-level1`]: {},
            },
          },
        },

        '.top-part-actions': {
          [`@apply ${classPrefix}flex ${classPrefix}flex-wrap`]: {},

          // Sizing
          [`@apply ${classPrefix}gap-level1`]: {},
        },
      },

      '.project-details-page-middle-part': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center`]: {},

        // Children
        '.middle-part-text': {
          [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

          // Sizing
          [`@apply ${classPrefix}max-w-119`]: {},
          [`@apply ${classPrefix}container-level2`]: {},
          //// Used for skeleton
          [`@apply ${classPrefix}w-full`]: {},

          // Children
          '.text-content': {
            [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

            // Sizing
            [`@apply ${classPrefix}max-w-119`]: {},
            [`@apply ${classPrefix}container-level2`]: {},
            //// Used for skeleton
            [`@apply ${classPrefix}w-full`]: {},
          },
        },
      },

      '.project-details-page-bottom-part': {
        [`@apply ${classPrefix}flex ${classPrefix}justify-center`]: {},
      },
    },
  });
};

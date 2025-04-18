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
      '.section-text-timeline': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col 2xl:${classPrefix}flex-row`]: {},

        // Sizing
        [`@apply ${classPrefix}container-level3`]: {},

        // Children
        '.timeline-item': {
          [`@apply ${classPrefix}grid ${classPrefix}grid-flow-row lg:${classPrefix}grid-flow-col 2xl:${classPrefix}grid-flow-row`]:
            {},

          // Sizing
          [`@apply ${classPrefix}container-level2`]: {},

          // Children
          '.item-date': {
            [`@apply ${classPrefix}text-center`]: {},

            // Colors
            [`@apply ${classPrefix}bg-blue-200`]: {},
            [`@apply ${classPrefix}text-blue-950`]: {},
            [`${darkModeContext} &`]: {
              [`@apply ${classPrefix}bg-blue-900`]: {},
              [`@apply ${classPrefix}text-blue-50`]: {},
            },

            // Sizing
            [`@apply ${classPrefix}h-fit lg:${classPrefix}min-w-33 2xl:${classPrefix}w-full`]: {},
            [`@apply ${classPrefix}p-2`]: {},
            [`@apply ${classPrefix}text-6`]: {},
            [`@apply ${classPrefix}rounded-25`]: {},

            // Others
            [`@apply ${classPrefix}font-light ${classPrefix}font-geist`]: {},
          },

          [`&:nth-child(even)`]: {
            '.item-date': {
              [`@apply lg:${classPrefix}order-2`]: {},
            },
          },
        },
      },

      // Overrides
      '.section-text': {
        [`@apply ${classPrefix}max-w-210`]: {},
        [`@apply ${classPrefix}container-level3`]: {},
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

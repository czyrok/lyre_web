import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addContactSectionComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.landing-page-contact-section': {
      [`@apply ${classPrefix}flex ${classPrefix}flex-col lg:${classPrefix}flex-row ${classPrefix}justify-center`]:
        {},

      // Children
      '.section-photo': {
        [`@apply ${classPrefix}relative ${classPrefix}overflow-hidden ${classPrefix}select-none`]:
          {},

        // Sizing
        [`@apply ${classPrefix}w-41 ${classPrefix}min-w-41 ${classPrefix}h-41 ${classPrefix}min-h-41`]:
          {},
        [`@apply ${classPrefix}rounded-full`]: {},

        // Border pseudo element
        '&::after': {
          [`@apply ${classPrefix}content-empty ${classPrefix}inset-0 ${classPrefix}absolute ${classPrefix}z-0`]:
            {},

          // Colors
          [`@apply ${classPrefix}border-white-300/70`]: {},
          [`@apply dark:${classPrefix}border-black-800/70`]: {},

          // Sizing
          [`@apply ${classPrefix}border-6`]: {},
          //// Required for the border
          [`@apply ${classPrefix}rounded-full`]: {},
        },
      },

      '.section-right': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level3`]: {},
      },

      // Overrides
      '.section-actions': {
        [`@apply lg:${classPrefix}justify-start`]: {},
      },
      '.anchor': {
        // Apply variants
        [`@apply v-sm:lg:${classPrefix}anchor-middle-screen-target`]: {},
        [`@apply max-lg:${classPrefix}anchor-top-target`]: {},
        [`@apply max-v-sm:${classPrefix}anchor-top-target`]: {},
      },
    },
  });
};

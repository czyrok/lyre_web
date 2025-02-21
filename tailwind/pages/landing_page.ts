import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addLandingPage: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix, darkModeContext }
) => {
  addComponents({
    '.landing-page': {
      // Colors
      [`@apply ${classPrefix}bg-white-50`]: {},

      // Children
      '.landing-page-cover': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}h-dvh`]: {},

        // Colors
        [`@apply ${classPrefix}bg-landing-page-cover-gradient`]: {},
        [`@apply ${classPrefix}from-blue-100 ${classPrefix}to-white-50`]: {},
        [darkModeContext]: {
          [`@apply ${classPrefix}from-blue-900 ${classPrefix}to-black-950`]: {},
        },

        '.cover-middle-part': {
          [`@apply ${classPrefix}relative ${classPrefix}grow ${classPrefix}flex ${classPrefix}items-center ${classPrefix}justify-center`]:
            {},

          // Children
          '.middle-part-shape': {
            [`@apply ${classPrefix}absolute ${classPrefix}inset-0 ${classPrefix}mx-auto`]: {},

            // Colors
            [`@apply ${classPrefix}text-blue-100`]: {},
            [darkModeContext]: {
              [`@apply ${classPrefix}text-blue-950`]: {},
            },

            // Sizing
            [`@apply ${classPrefix}max-w-landing-page-shape`]: {},
          },

          '.middle-part-brand-wrapper': {
            [`@apply ${classPrefix}z-10`]: {},
          },
        },

        '.cover-bottom-part': {
          [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center ${classPrefix}gap-level3`]:
            {},

          // Children
          '.bottom-part-catchphrase': {
            [`@apply ${classPrefix}text-center`]: {},

            // Colors
            [`@apply ${classPrefix}text-black-950`]: {},
            [darkModeContext]: {
              [`@apply ${classPrefix}text-white-50`]: {},
            },

            // Sizing
            [`@apply ${classPrefix}text-6`]: {},

            // Others
            [`@apply ${classPrefix}font-light ${classPrefix}font-geist`]: {},
          },

          '.bottom-part-discover': {
            [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center ${classPrefix}gap-level1`]:
              {},

            // Sizing
            [`@apply ${classPrefix}pb-3`]: {},

            // Children
            '.discover-scroll-down': {
              // Colors
              [`@apply ${classPrefix}text-green-500`]: {},
              [darkModeContext]: {
                [`@apply ${classPrefix}text-green-600`]: {},
              },

              // Sizing
              [`@apply ${classPrefix}w-3`]: {},

              // Children
              '& svg path': {
                [`@apply ${classPrefix}stroke-3`]: {},
              },
            },
          },
        },
      },

      '.landing-page-content': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

        // Sizing
        [`@apply ${classPrefix}py-25`]: {},
        [`@apply ${classPrefix}gap-16`]: {},
      },
    },
  });
};

import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addBrandComponent: ComponentBuilderFunction = ({ addComponents }, { classPrefix }) => {
  addComponents({
    '.brand': {
      [`@apply ${classPrefix}w-fit ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center`]:
        {},

      // Children
      '.brand-badge': {
        [`@apply ${classPrefix}flex ${classPrefix}justify-center ${classPrefix}items-center`]: {},

        // Colors
        [`@apply ${classPrefix}bg-blue-200`]: {},
        [`@apply dark:${classPrefix}bg-blue-800`]: {},

        // Sizing
        [`@apply ${classPrefix}rounded-full`]: {},

        // Children
        '.badge-logo': {
          [`@apply ${classPrefix}h-fit`]: {},

          // Colors
          [`@apply ${classPrefix}text-blue-50`]: {},
          [`@apply dark:${classPrefix}text-blue-200`]: {},

          // Sizing
          [`@apply ${classPrefix}w-brand-badge-logo`]: {},
        },
      },

      '.brand-name': {
        // Colors
        [`@apply ${classPrefix}text-black-950`]: {},
        [`@apply dark:${classPrefix}text-white-50`]: {},

        // Others
        [`@apply ${classPrefix}font-brand-name ${classPrefix}font-geist-mono`]: {},
      },

      // Variants
      '&.brand-size-xl': {
        // Sizing
        [`@apply ${classPrefix}gap-6`]: {},

        // Children
        '.brand-badge': {
          // Sizing
          [`@apply ${classPrefix}w-33 ${classPrefix}h-33`]: {},
        },

        '.brand-name': {
          // Sizing
          [`@apply ${classPrefix}text-12`]: {},
        },

        '&:not(:has(.brand-badge)) .brand-name': {
          // Sizing
          [`@apply ${classPrefix}text-24`]: {},
        },
      },

      '&.brand-size-lg': {
        // Children
        '.brand-badge': {
          // Sizing
          [`@apply ${classPrefix}w-25 ${classPrefix}h-25`]: {},
        },
      },

      '&.brand-size-sm': {
        // Children
        '.brand-badge': {
          // Sizing
          [`@apply ${classPrefix}w-10 ${classPrefix}h-10`]: {},
        },
      },
    },
  });
};

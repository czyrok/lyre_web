import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addH1Base: ComponentBuilderFunction = (
  { addBase, addComponents },
  { classPrefix }
) => {
  addBase({
    h1: {
      [`@apply ${classPrefix}w-fit ${classPrefix}bg-clip-text ${classPrefix}text-transparent`]: {},

      // Colors
      [`@apply ${classPrefix}bg-bash-gradient`]: {},
      [`@apply ${classPrefix}from-blue-800 ${classPrefix}to-blue-500`]: {},
      [`@apply dark:${classPrefix}from-blue-400 dark:${classPrefix}to-blue-200`]: {},

      // Others
      [`@apply ${classPrefix}font-geist-mono`]: {},

      '&::before': {
        // Colors
        [`@apply ${classPrefix}text-blue-800`]: {},
        [`@apply dark:${classPrefix}text-blue-400`]: {},

        // Others
        [`@apply ${classPrefix}content-bash`]: {},
      },

      // Variants
      [`&.${classPrefix}title-size-xl`]: {
        // Sizing
        [`@apply ${classPrefix}text-13 sm:${classPrefix}text-15 md:${classPrefix}text-20`]: {},

        // Others
        [`@apply ${classPrefix}font-title-xl`]: {},
      },

      [`&.${classPrefix}title-size-lg`]: {
        // Sizing
        [`@apply ${classPrefix}text-10 sm:${classPrefix}text-12 md:${classPrefix}text-15`]: {},

        // Others
        [`@apply ${classPrefix}font-title-lg`]: {},
      },
    },
  });

  addComponents({
    '.h1-skeleton': {
      [`@apply ${classPrefix}animate-pulse ${classPrefix}box-content`]: {},

      // Colors
      [`@apply ${classPrefix}bg-white-200 dark:${classPrefix}bg-black-700`]: {},

      // Sizing
      [`@apply ${classPrefix}w-[5em] ${classPrefix}h-[1em]`]: {},
      [`@apply ${classPrefix}my-[0.25em]`]: {},
      [`@apply ${classPrefix}rounded-10`]: {},

      // Variants
      [`&.title-size-xl`]: {
        // Sizing
        [`@apply ${classPrefix}text-13 sm:${classPrefix}text-15 md:${classPrefix}text-20`]: {},
      },

      [`&.title-size-lg`]: {
        // Sizing
        [`@apply ${classPrefix}text-10 sm:${classPrefix}text-12 md:${classPrefix}text-15`]: {},
      },
    },
  });
};

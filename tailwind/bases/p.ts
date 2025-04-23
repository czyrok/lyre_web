import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addPBase: ComponentBuilderFunction = ({ addBase, addComponents }, { classPrefix }) => {
  addBase({
    p: {
      // Colors
      [`@apply ${classPrefix}text-black-950`]: {},
      [`@apply dark:${classPrefix}text-white-50`]: {},

      // Sizing
      [`@apply ${classPrefix}text-4 sm:${classPrefix}text-5 md:${classPrefix}text-6`]: {},

      // Others
      [`@apply ${classPrefix}font-regular ${classPrefix}font-geist`]: {},
    },
  });

  addComponents({
    '.p-skeleton': {
      [`@apply ${classPrefix}animate-pulse`]: {},

      // Colors
      [`@apply ${classPrefix}bg-white-200 dark:${classPrefix}bg-black-700`]: {},

      // Sizing
      [`@apply ${classPrefix}w-full ${classPrefix}h-[1.5em]`]: {},
      '&:last-child': {
        [`@apply ${classPrefix}w-[9em]`]: {},
      },
      [`@apply ${classPrefix}rounded-25`]: {},
      [`@apply ${classPrefix}text-4 sm:${classPrefix}text-5 md:${classPrefix}text-6`]: {},
    },
  });
};

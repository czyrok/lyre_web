import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addProjectTagComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.project-tag': {
      // Colors
      [`@apply ${classPrefix}bg-blue-300`]: {},
      [`@apply ${classPrefix}text-blue-700`]: {},
      [`@apply dark:${classPrefix}bg-blue-400`]: {},
      [`@apply dark:${classPrefix}text-blue-950`]: {},

      // Sizing
      [`@apply ${classPrefix}px-3 ${classPrefix}py-1`]: {},
      [`@apply ${classPrefix}text-4`]: {},
      [`@apply ${classPrefix}rounded-25`]: {},

      // Others
      [`@apply ${classPrefix}font-tag-text ${classPrefix}font-geist`]: {},
    },

    '.project-tag-skeleton': {
      [`@apply ${classPrefix}animate-pulse ${classPrefix}box-content`]: {},

      // Colors
      [`@apply ${classPrefix}bg-white-200`]: {},
      [`@apply dark:${classPrefix}bg-black-700`]: {},

      // Sizing
      [`@apply ${classPrefix}w-[2.5em] ${classPrefix}h-[1.5em]`]: {},
      '&:last-child': {
        [`@apply ${classPrefix}w-[4em]`]: {},
      },
      [`@apply ${classPrefix}px-3 ${classPrefix}py-1`]: {},
      [`@apply ${classPrefix}text-4`]: {},
      [`@apply ${classPrefix}rounded-25`]: {},
    },
  });
};

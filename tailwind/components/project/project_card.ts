import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addProjectCardComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.project-card': {
      [`@apply ${classPrefix}flex ${classPrefix}flex-col`]: {},

      // Sizing
      [`@apply ${classPrefix}p-2`]: {},
      [`@apply ${classPrefix}gap-1`]: {},

      // Children
      '.card-title': {
        // Colors
        [`@apply ${classPrefix}text-black-950`]: {},
        [`@apply dark:${classPrefix}text-white-50`]: {},

        // Sizing
        [`@apply ${classPrefix}text-7`]: {},

        // Others
        [`@apply ${classPrefix}font-regular ${classPrefix}font-geist-mono`]: {},
      },

      '.card-tag-container': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-row`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-1`]: {},
      },
    },

    '.project-card-skeleton': {
      [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}animate-pulse`]: {},

      // Sizing
      [`@apply ${classPrefix}p-2`]: {},
      [`@apply ${classPrefix}gap-1`]: {},

      // Children
      '.card-skeleton-title': {
        // Colors
        [`@apply ${classPrefix}bg-white-200`]: {},
        [`@apply dark:${classPrefix}bg-black-700`]: {},

        // Sizing
        [`@apply ${classPrefix}w-[6em] ${classPrefix}h-[1.5em]`]: {},
        [`@apply ${classPrefix}rounded-18`]: {},
        [`@apply ${classPrefix}text-7`]: {},
      },

      '.card-skeleton-tag-container': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-row`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-1`]: {},
      },
    },
  });
};

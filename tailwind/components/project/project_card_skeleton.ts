import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addProjectCardSkeletonComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.project-card-skeleton': {
      [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}animate-pulse`]: {},

      // Sizing
      [`@apply ${classPrefix}p-2`]: {},
      [`@apply ${classPrefix}gap-1`]: {},

      // Children
      '.card-thumbnail-skeleton': {
        // Colors
        [`@apply ${classPrefix}bg-white-200`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}bg-black-500`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}w-41 ${classPrefix}h-28`]: {},
        [`@apply ${classPrefix}rounded-5`]: {},
      },

      '.card-title-skeleton': {
        // Colors
        [`@apply ${classPrefix}bg-white-200`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}bg-black-500`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}w-[104px] ${classPrefix}h-[35px]`]: {},
        [`@apply ${classPrefix}rounded-18`]: {},
      },

      '.card-tag-skeleton-container': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-row`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-1`]: {},

        // Children
        '.card-tag-skeleton': {
          // Colors
          [`@apply ${classPrefix}bg-white-200`]: {},
          [`${darkModeContext} &`]: {
            [`@apply ${classPrefix}bg-black-500`]: {},
          },

          // Sizing
          [`@apply ${classPrefix}w-[80px] ${classPrefix}h-[36px]`]: {},
          '&:last-child': {
            [`@apply ${classPrefix}w-[101px]`]: {},
          },
          [`@apply ${classPrefix}rounded-25`]: {},
        },
      },
    },
  });
};

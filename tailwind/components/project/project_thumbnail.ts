import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addProjectThumbnailComponent: ComponentBuilderFunction = (
  { addComponents },
  { darkModeContext, classPrefix }
) => {
  addComponents({
    '.project-thumbnail': {
      [`@apply ${classPrefix}inline-block ${classPrefix}relative ${classPrefix}overflow-visible ${classPrefix}select-none`]:
        {},

      // Colors
      [`@apply ${classPrefix}outline-white-200`]: {},
      [`${darkModeContext} &`]: {
        [`@apply ${classPrefix}outline-black-500`]: {},
      },

      // Sizing
      [`@apply ${classPrefix}w-41 ${classPrefix}h-28`]: {},
      [`@apply ${classPrefix}rounded-5`]: {},
      [`@apply ${classPrefix}outline ${classPrefix}outline-offset-l3 ${classPrefix}outline-3`]: {},

      // Others
      [`@apply ${classPrefix}transition-project-thumbnail-background-and-border ${classPrefix}duration-project-thumbnail-background-and-border ${classPrefix}ease-project-thumbnail-background-and-border`]:
        {},

      // Hover state
      '&:has(.thumbnail-hover):hover': {
        [`@apply ${classPrefix}outline-transparent`]: {},
      },
      '&:hover .thumbnail-hover': {
        [`@apply ${classPrefix}opacity-100`]: {},
      },

      // Focus state
      ':is(&:has(.thumbnail-hover):focus, a:focus &:has(.thumbnail-hover))': {
        // Colors
        [`@apply ${classPrefix}ring-green-400`]: {},
        //// This needs to correspond to page background
        [`@apply ${classPrefix}ring-offset-white-50`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}ring-green-600`]: {},
          [`@apply ${classPrefix}ring-offset-black-950`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}ring-3 ${classPrefix}ring-offset-3`]: {},
      },

      // Children
      '.thumbnail-image': {
        [`@apply ${classPrefix}overflow-hidden ${classPrefix}pointer-events-none`]: {},

        // Sizing (required for the image)
        [`@apply ${classPrefix}w-full ${classPrefix}h-full`]: {},
        [`@apply ${classPrefix}rounded-5`]: {},
      },

      '.thumbnail-hover': {
        [`@apply ${classPrefix}flex ${classPrefix}justify-center ${classPrefix}items-center ${classPrefix}inset-0 ${classPrefix}absolute ${classPrefix}z-0 ${classPrefix}opacity-0`]:
          {},

        // Colors
        [`@apply ${classPrefix}bg-purple-blue-200/33`]: {},
        [`@apply ${classPrefix}outline-purple-blue-400`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}bg-purple-blue-500/33`]: {},
          [`@apply ${classPrefix}outline-purple-blue-500`]: {},
        },

        // Sizing (required for the border)
        [`@apply ${classPrefix}rounded-5`]: {},

        // Others
        [`@apply ${classPrefix}backdrop-blur-4`]: {},
        [`@apply ${classPrefix}outline ${classPrefix}outline-offset-l3 ${classPrefix}outline-3`]:
          {},
        [`@apply ${classPrefix}transition-project-thumbnail-background-and-border ${classPrefix}duration-project-thumbnail-background-and-border ${classPrefix}ease-project-thumbnail-background-and-border`]:
          {},

        '.thumbnail-hover-icon': {
          [`@apply ${classPrefix}inline-block`]: {},

          // Colors
          [`@apply ${classPrefix}text-purple-blue-400`]: {},
          [`${darkModeContext} &`]: {
            [`@apply ${classPrefix}text-purple-blue-500`]: {},
          },

          // Sizing
          [`@apply ${classPrefix}w-7 ${classPrefix}h-7`]: {},

          // Children
          '& svg path': {
            [`@apply ${classPrefix}stroke-3`]: {},
          },
        },
      },

      '.thumbnail-brand': {
        [`@apply ${classPrefix}absolute`]: {},

        // Sizing (required for the border)
        [`@apply ${classPrefix}right-l7/1 ${classPrefix}bottom-l7/1`]: {},
      },
    },

    '.project-thumbnail-skeleton': {
      [`@apply ${classPrefix}inline-block ${classPrefix}relative ${classPrefix}overflow-visible`]:
        {},

      // Children
      '.thumbnail-skeleton-image': {
        [`@apply ${classPrefix}animate-pulse`]: {},

        // Colors
        [`@apply ${classPrefix}bg-white-200`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}bg-black-700`]: {},
        },

        // Sizing
        [`@apply ${classPrefix}w-41 ${classPrefix}h-28`]: {},
        [`@apply ${classPrefix}rounded-5`]: {},
      },

      '.thumbnail-skeleton-brand': {
        [`@apply ${classPrefix}absolute ${classPrefix}animate-none`]: {},

        // Sizing (required for the border)
        [`@apply ${classPrefix}right-l7/1 ${classPrefix}bottom-l7/1`]: {},
      },
    },
  });
};

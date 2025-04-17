import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addH1Base: ComponentBuilderFunction = (
  { addBase },
  { darkModeContext, classPrefix }
) => {
  addBase({
    h1: {
      [`@apply ${classPrefix}w-fit ${classPrefix}bg-clip-text ${classPrefix}text-transparent`]: {},

      // Colors
      [`@apply ${classPrefix}bg-bash-gradient`]: {},
      [`@apply ${classPrefix}from-blue-800 ${classPrefix}to-blue-500`]: {},
      [`${darkModeContext} &`]: {
        [`@apply ${classPrefix}from-blue-400 ${classPrefix}to-blue-200`]: {},
      },

      // Others
      [`@apply ${classPrefix}font-geist-mono`]: {},

      '&::before': {
        // Colors
        [`@apply ${classPrefix}text-blue-800`]: {},
        [`${darkModeContext} &`]: {
          [`@apply ${classPrefix}text-blue-400`]: {},
        },

        // Others
        [`@apply ${classPrefix}content-bash`]: {},
      },

      // Variants
      [`&.${classPrefix}title-size-xl`]: {
        // Sizing
        [`@apply ${classPrefix}text-15 sm:${classPrefix}text-20`]: {},

        // Others
        [`@apply ${classPrefix}font-title-xl`]: {},
      },

      // Variants
      [`&.${classPrefix}title-size-lg`]: {
        // Sizing
        [`@apply ${classPrefix}text-10 sm:${classPrefix}text-15`]: {},

        // Others
        [`@apply ${classPrefix}font-title-lg`]: {},
      },
    },
  });
};

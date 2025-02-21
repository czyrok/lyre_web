import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addBodyBase: ComponentBuilderFunction = (
  { addBase },
  { classPrefix, darkModeContext }
) => {
  addBase({
    body: {
      // Colors
      [`@apply ${classPrefix}bg-white-50`]: {},

      [`&:has(#style-settings${darkModeContext})`]: {
        [`@apply ${classPrefix}bg-black-950`]: {},
      },
    },
  });
};

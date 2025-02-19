import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addPBase: ComponentBuilderFunction = (
  { addBase },
  { darkModeContext, classPrefix }
) => {
  addBase({
    p: {
      // Colors
      [`@apply ${classPrefix}text-black-950`]: {},
      [darkModeContext]: {
        [`@apply ${classPrefix}text-white-50`]: {},
      },

      // Sizing
      [`@apply ${classPrefix}text-6`]: {},

      // Others
      [`@apply ${classPrefix}font-regular ${classPrefix}font-geist`]: {},
    },
  });
};

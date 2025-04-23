import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addBodyBase: ComponentBuilderFunction = ({ addBase }, { classPrefix }) => {
  addBase({
    body: {
      // Colors
      // If you modify this, don't forget to update focus states
      [`@apply ${classPrefix}bg-white-50`]: {},

      [`&:has(#style-settings.${classPrefix}dark)`]: {
        [`@apply ${classPrefix}bg-black-950`]: {},
      },
    },
  });
};

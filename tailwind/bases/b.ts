import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addBBase: ComponentBuilderFunction = ({ addBase }, { classPrefix }) => {
  addBase({
    b: {
      // Colors
      [`@apply ${classPrefix}text-blue-500`]: {},
      [`@apply dark:${classPrefix}text-blue-400`]: {},
    },
  });
};

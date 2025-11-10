import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addStrongBase: ComponentBuilderFunction = ({ addBase }, { classPrefix }) => {
  addBase({
    strong: {
      // Colors
      [`@apply ${classPrefix}text-blue-500`]: {},
      [`@apply dark:${classPrefix}text-blue-400`]: {},
    },
  });
};

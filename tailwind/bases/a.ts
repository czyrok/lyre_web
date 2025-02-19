import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addABase: ComponentBuilderFunction = ({ addBase }, { classPrefix }) => {
  addBase({
    a: {
      '&:focus': {
        //// Removes the default outline (not removed by Tailwind)
        [`@apply ${classPrefix}outline-0`]: {},
      },
    },
  });
};

import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addHtmlBase: ComponentBuilderFunction = ({ addBase }, { classPrefix }) => {
  addBase({
    html: {
      // Others
      [`@apply ${classPrefix}scroll-smooth`]: {},
    },
  });
};

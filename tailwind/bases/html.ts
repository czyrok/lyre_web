import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addHtmlBase: ComponentBuilderFunction = ({ addBase }, { classPrefix }) => {
  addBase({
    html: {
      // Others
      [`@apply ${classPrefix}scroll-smooth`]: {},

      //// https://css-tricks.com/fixing-smooth-scrolling-with-find-on-page/
      '@media screen and (prefers-reduced-motion: reduce)': {
        [`@apply ${classPrefix}scroll-auto`]: {},
      },
    },
  });
};

import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addLiBase: ComponentBuilderFunction = ({ addBase }, { classPrefix }) => {
  addBase({
    li: {
      // Colors
      [`@apply ${classPrefix}text-black-950`]: {},
      [`@apply dark:${classPrefix}text-white-50`]: {},

      // Sizing
      [`@apply ${classPrefix}text-6`]: {},
      [`@apply ${classPrefix}ml-1`]: {},

      // Others
      [`@apply ${classPrefix}font-regular ${classPrefix}font-geist`]: {},

      '&::before': {
        [`@apply ${classPrefix}content-list-item`]: {},
      },
    },
  });
};

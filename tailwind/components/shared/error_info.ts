import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addErrorInfoComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.error-info': {
      [`@apply ${classPrefix}bg-clip-text ${classPrefix}text-transparent`]: {},

      // Colors
      [`@apply ${classPrefix}bg-bash-gradient`]: {},
      [`@apply ${classPrefix}from-red-800 ${classPrefix}to-red-500`]: {},
      [`@apply dark:${classPrefix}from-red-400 dark:${classPrefix}to-red-200`]: {},

      // Sizing
      [`@apply ${classPrefix}text-5`]: {},

      // Others
      [`@apply ${classPrefix}font-light ${classPrefix}font-geist`]: {},

      '&::before': {
        // Colors
        [`@apply ${classPrefix}text-red-800`]: {},
        [`@apply dark:${classPrefix}text-red-400`]: {},

        // Others
        [`@apply ${classPrefix}content-bash`]: {},
      },
    },
  });
};

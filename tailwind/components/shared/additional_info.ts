import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addAdditionalInfoComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.additional-info': {
      // Colors
      [`@apply ${classPrefix}text-black-950`]: {},
      [`@apply dark:${classPrefix}text-white-50`]: {},

      // Sizing
      [`@apply ${classPrefix}text-5`]: {},

      // Others
      [`@apply ${classPrefix}font-light ${classPrefix}font-geist`]: {},
    },
  });
};

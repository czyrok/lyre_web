import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addContainerLevel4Component: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.container-level4': {
      // Sizing
      [`@apply ${classPrefix}gap-8 sm:${classPrefix}gap-10 md:${classPrefix}gap-level4`]: {},
    },
  });
};

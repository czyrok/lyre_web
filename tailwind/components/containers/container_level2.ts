import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addContainerLevel2Component: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.container-level2': {
      // Sizing
      [`@apply ${classPrefix}gap-1 sm:${classPrefix}gap-2 md:${classPrefix}gap-level2`]: {},
    },
  });
};

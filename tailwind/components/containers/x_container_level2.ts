import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addXContainerLevel2Component: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.x-container-level2': {
      // Sizing
      [`@apply ${classPrefix}gap-x-1 sm:${classPrefix}gap-x-2 md:${classPrefix}gap-x-level2`]: {},
    },
  });
};

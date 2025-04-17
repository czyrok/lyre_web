import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addContainerLevel3Component: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.container-level3': {
      // Sizing
      [`@apply ${classPrefix}gap-4 sm:${classPrefix}gap-5 md:${classPrefix}gap-level3`]: {},
    },
  });
};

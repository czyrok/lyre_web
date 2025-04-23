import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addInfoErrorPageLayout: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.info-error-page-layout': {
      // Children
      '.info-error-page-layout-errors': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center`]: {},

        // Sizing
        [`@apply ${classPrefix}gap-level2`]: {},
      },
    },
  });
};

import { ComponentBuilderFunction } from '../types/component_builder_function';

export const addNotFoundErrorPageLayout: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.not-found-error-page-layout': {
      // Children
      '.not-found-error-page-layout-home-back': {
        [`@apply ${classPrefix}flex ${classPrefix}flex-col ${classPrefix}items-center`]: {},

        // Sizing
        [`@apply ${classPrefix}container-level2`]: {},
      },
    },
  });
};

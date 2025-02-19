import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addAnchorComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.anchor': {
      [`@apply ${classPrefix}absolute ${classPrefix}z-l1 ${classPrefix}invisible ${classPrefix}top-0`]:
        {},

      // Apply variants
      [`@apply ${classPrefix}anchor-top-target`]: {},

      // Modify parent
      [`:has(> &)`]: {
        [`@apply ${classPrefix}relative`]: {},
      },
    },
  });

  // Variants to be applied directly depending on media queries
  addComponents({
    '.anchor-middle-screen-target': {
      [`@apply ${classPrefix}top-2/4`]: {},

      // Sizing
      [`@apply ${classPrefix}mt-middle-screen-shift`]: {},
    },
  });

  addComponents({
    '.anchor-top-target': {
      // Sizing
      [`@apply ${classPrefix}mt-l4`]: {},
    },
  });
};

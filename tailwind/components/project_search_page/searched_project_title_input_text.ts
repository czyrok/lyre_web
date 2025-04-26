import { ComponentBuilderFunction } from '../../types/component_builder_function';

export const addSearchedProjectTitleInputTextComponent: ComponentBuilderFunction = (
  { addComponents },
  { classPrefix }
) => {
  addComponents({
    '.searched-project-title-input-text': {
      // Sizing
      [`@apply ${classPrefix}max-w-62 ${classPrefix}w-full`]: {},
    },
  });
};

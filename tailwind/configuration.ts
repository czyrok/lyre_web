import { Config } from 'tailwindcss';
import plugin from 'tailwindcss/plugin';
import { addH1Base } from './bases/h1';
import { addParagraphBase } from './bases/paragraph';
import { addHomeProjectSectionComponent } from './components/home/home_project_section';
import { addProjectCardComponent } from './components/project/project_card';
import { addProjectCardSkeletonComponent } from './components/project/project_card_skeleton';
import { addProjectTagComponent } from './components/project/project_tag';
import { addProjectThumbnailComponent } from './components/project/project_thumbnail';
import { addAccentuationButtonComponent } from './components/shared/accentuation_button';
import { addAccentuationInputTextComponent } from './components/shared/accentuation_input_text';
import { addBrandComponent } from './components/shared/brand';
import { addDropdownMenuComponent } from './components/shared/dropdown_menu';
import { addNavBarComponent } from './components/shared/nav_bar';
import { addPaginationComponent } from './components/shared/pagination';
import { addPrimaryButtonComponent } from './components/shared/primary_button';
import { addSecondaryButtonComponent } from './components/shared/secondary_button';
import { addSecondaryCheckboxComponent } from './components/shared/secondary_checkbox';
import { addSecondaryLinkComponent } from './components/shared/secondary_link';
import { addSecondarySelectComponent } from './components/shared/secondary_select';
import { THEME_BACKGROUND } from './theme/background';
import { THEME_BLUR } from './theme/blur';
import { THEME_BORDER_RADIUS } from './theme/border_radius';
import { THEME_BORDER_WIDTH } from './theme/border_width';
import { THEME_BOX_SHADOW } from './theme/box_shadow';
import { THEME_COLORS } from './theme/colors';
import { THEME_CONTENT } from './theme/content';
import { EXTENDED_THEME_CONTAINER } from './theme/extend/container';
import { EXTENDED_THEME_GAP } from './theme/extend/gap';
import { EXTENDED_THEME_SIZE } from './theme/extend/size';
import { THEME_FONT_FAMILY } from './theme/font_family';
import { THEME_FONT_SIZE } from './theme/font_size';
import { THEME_FONT_WEIGHT } from './theme/font_weight';
import { THEME_OPACITY } from './theme/opacity';
import { THEME_OUTLINE_OFFSET } from './theme/outline_offset';
import { THEME_OUTLINE_WIDTH } from './theme/outline_width';
import { THEME_RING_OFFSET_WIDTH } from './theme/ring_offset_width';
import { THEME_RING_WIDTH } from './theme/ring_width';
import { THEME_SCREENS } from './theme/screens';
import { THEME_SPACING } from './theme/spacing';
import { THEME_STROKE_WIDTH } from './theme/stroke_width';
import { THEME_TRANSITION } from './theme/transition';

const DARK_MODE_CLASS = 'dark';
const TAILWIND_CLASS_PREFIX = 'tw-';

export const TAILWIND_CONFIGURATION: Config = {
  //// https://tailwindcss.com/docs/content-configuration
  content: ['./src/**/*.rs'],
  darkMode: 'selector',
  prefix: TAILWIND_CLASS_PREFIX,
  theme: {
    screens: THEME_SCREENS,
    borderRadius: THEME_BORDER_RADIUS,
    borderWidth: THEME_BORDER_WIDTH,
    colors: THEME_COLORS,
    fontFamily: THEME_FONT_FAMILY,
    fontSize: THEME_FONT_SIZE,
    spacing: THEME_SPACING,
    opacity: THEME_OPACITY,
    blur: THEME_BLUR,
    content: THEME_CONTENT,
    ringWidth: THEME_RING_WIDTH,
    ringOffsetWidth: THEME_RING_OFFSET_WIDTH,
    outlineWidth: THEME_OUTLINE_WIDTH,
    outlineOffset: THEME_OUTLINE_OFFSET,
    strokeWidth: THEME_STROKE_WIDTH,
    fontWeight: THEME_FONT_WEIGHT,
    boxShadow: THEME_BOX_SHADOW,
    ...THEME_TRANSITION,
    ...THEME_BACKGROUND,
    extend: {
      // Gap is here to avoid to override the spacing values defined above
      gap: EXTENDED_THEME_GAP,
      container: EXTENDED_THEME_CONTAINER,
      ...EXTENDED_THEME_SIZE,
    },
  },
  plugins: [
    plugin(function (pluginApi) {
      const darkModeConfig: string = pluginApi.config('darkMode', 'selector');

      let darkModeContext = `.${DARK_MODE_CLASS} &`;
      let darkModeContextForBases = `.${TAILWIND_CLASS_PREFIX}${DARK_MODE_CLASS} &`;

      if (darkModeConfig === 'media') {
        darkModeContext = '@media (prefers-color-scheme: dark)';
        darkModeContextForBases = '@media (prefers-color-scheme: dark)';
      }

      addH1Base(pluginApi, {
        darkModeContext: darkModeContextForBases,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addParagraphBase(pluginApi, {
        darkModeContext: darkModeContextForBases,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });

      addProjectTagComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addPrimaryButtonComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addAccentuationButtonComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addSecondaryButtonComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addProjectCardSkeletonComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addProjectThumbnailComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addProjectCardComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addNavBarComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addSecondarySelectComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addAccentuationInputTextComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addSecondaryLinkComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addPaginationComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addBrandComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addSecondaryCheckboxComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addDropdownMenuComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addHomeProjectSectionComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
    }),
  ],
} as const;

export default TAILWIND_CONFIGURATION;

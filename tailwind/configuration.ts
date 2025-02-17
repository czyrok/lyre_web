import { Config } from 'tailwindcss';
import plugin from 'tailwindcss/plugin';
import { addAccentuationButtonComponent } from './components/accentuation_button';
import { addAccentuationInputComponent } from './components/accentuation_input';
import { addBrandComponent } from './components/brand';
import { addDropdownComponent } from './components/dropdown';
import { addNavBarComponent } from './components/nav_bar';
import { addPaginationComponent } from './components/pagination';
import { addPrimaryButtonComponent } from './components/primary_button';
import { addProjectCardComponent } from './components/project_card';
import { addProjectCardSkeletonComponent } from './components/project_card_skeleton';
import { addProjectTagComponent } from './components/project_tag';
import { addProjectThumbnailComponent } from './components/project_thumbnail';
import { addSecondaryButtonComponent } from './components/secondary_button';
import { addSecondaryCheckboxComponent } from './components/secondary_checkbox';
import { addSecondaryLinkComponent } from './components/secondary_link';
import { addSecondarySelectComponent } from './components/secondary_select';
import { THEME_BACKGROUND } from './theme/background';
import { THEME_BLUR } from './theme/blur';
import { THEME_BORDER_RADIUS } from './theme/border_radius';
import { THEME_BORDER_WIDTH } from './theme/border_width';
import { THEME_BOX_SHADOW } from './theme/box_shadow';
import { THEME_COLORS } from './theme/colors';
import { THEME_CONTENT } from './theme/content';
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

const DARK_MODE_CLASS = '.dark';
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
      // TODO: Container
      gap: EXTENDED_THEME_GAP,
      ...EXTENDED_THEME_SIZE,
    },
  },
  plugins: [
    plugin(function (pluginApi) {
      const darkModeConfig: string = pluginApi.config('darkMode', 'selector');

      let darkModeContext = `${DARK_MODE_CLASS} &`;

      if (darkModeConfig === 'media') {
        darkModeContext = '@media (prefers-color-scheme: dark)';
      }

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
      addAccentuationInputComponent(pluginApi, {
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
      addDropdownComponent(pluginApi, {
        darkModeContext,
        classPrefix: TAILWIND_CLASS_PREFIX,
      });

      /*
      TODO: ajouter les styles par défaut des h1 ?

      addVariant({
      })

      // title text etc...
      addBase({
        h1: {
          fontSize: theme("fontSize.2xl"),
        },
        h2: {
          fontSize: theme("fontSize.xl"),
        },
      }); */

      /* addUtilities({
        ".content-auto": {
          contentVisibility: "auto",
        },
      }); */
    }),
  ],
} as const;

export default TAILWIND_CONFIGURATION;

import { Config } from 'tailwindcss';
import plugin from 'tailwindcss/plugin';
import { addABase } from './bases/a';
import { addBBase } from './bases/b';
import { addBodyBase } from './bases/body';
import { addH1Base } from './bases/h1';
import { addHtmlBase } from './bases/html';
import { addPBase } from './bases/p';
import { addStrongBase } from './bases/strong';
import { addContactSectionComponent } from './components/landing_page/contact_section';
import { addHighlightedSectionComponent } from './components/landing_page/highlighted_section';
import { addProjectSectionComponent } from './components/landing_page/project_section';
import { addSectionContainerComponent } from './components/landing_page/section_container';
import { addProjectCardComponent } from './components/project/project_card';
import { addProjectTagComponent } from './components/project/project_tag';
import { addProjectThumbnailComponent } from './components/project/project_thumbnail';
import { addSearchedProjectTitleInputTextComponent } from './components/project_search_page/searched_project_title_input_text';
import { addAccentuationButtonComponent } from './components/shared/accentuation_button';
import { addAccentuationInputTextComponent } from './components/shared/accentuation_input_text';
import { addAdditionalInfoComponent } from './components/shared/additional_info';
import { addAnchorComponent } from './components/shared/anchor';
import { addBrandComponent } from './components/shared/brand';
import { addDropdownMenuComponent } from './components/shared/dropdown_menu';
import { addErrorInfoComponent } from './components/shared/error_info';
import { addFooterComponent } from './components/shared/footer';
import { addNavBarComponent } from './components/shared/nav_bar';
import { addPaginationComponent } from './components/shared/pagination';
import { addPrimaryButtonComponent } from './components/shared/primary_button';
import { addSecondaryButtonComponent } from './components/shared/secondary_button';
import { addSecondaryCheckboxComponent } from './components/shared/secondary_checkbox';
import { addSecondaryLinkComponent } from './components/shared/secondary_link';
import { addSecondarySelectComponent } from './components/shared/secondary_select';
import { addInfoErrorPageLayout } from './layouts/info_error_page_layout';
import { addNotFoundErrorPageLayout } from './layouts/not_found_error_page_layout';
import { addSecondaryPageLayout } from './layouts/secondary_page_layout';
import { addLandingPage } from './pages/landing_page';
import { addProjectDetailsPage } from './pages/project_details_page';
import { addProjectSearchPage } from './pages/project_search_page';
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
import { THEME_Z_INDEX } from './theme/z_index';
import { addDropdownMenuSearchBarComponent } from './components/shared/dropdown_menu_search_bar';

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
    zIndex: THEME_Z_INDEX,
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
      addH1Base(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addPBase(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addABase(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addHtmlBase(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addBodyBase(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addBBase(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addStrongBase(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });

      addProjectTagComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addPrimaryButtonComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addAccentuationButtonComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addSecondaryButtonComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addProjectThumbnailComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addProjectCardComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addNavBarComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addSecondarySelectComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addAccentuationInputTextComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addSecondaryLinkComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addPaginationComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addBrandComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addSecondaryCheckboxComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addDropdownMenuComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addDropdownMenuSearchBarComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addProjectSectionComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addSectionContainerComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addHighlightedSectionComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addContactSectionComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addAnchorComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addFooterComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addSearchedProjectTitleInputTextComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addAdditionalInfoComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addErrorInfoComponent(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });

      addLandingPage(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addProjectSearchPage(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addProjectDetailsPage(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });

      addSecondaryPageLayout(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addInfoErrorPageLayout(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
      addNotFoundErrorPageLayout(pluginApi, {
        classPrefix: TAILWIND_CLASS_PREFIX,
      });
    }),
  ],
} as const;

export default TAILWIND_CONFIGURATION;

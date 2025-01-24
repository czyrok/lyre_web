import { PluginAPI } from 'tailwindcss/types/config';

export type ComponentBuilderFunction = (
  pluginApi: PluginAPI,
  context: {
    darkModeContext: string;
    classPrefix: string;
  }
) => void;

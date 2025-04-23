import { PluginAPI } from 'tailwindcss/types/config';

export type ComponentBuilderFunction = (
  pluginApi: PluginAPI,
  context: {
    classPrefix: string;
  }
) => void;

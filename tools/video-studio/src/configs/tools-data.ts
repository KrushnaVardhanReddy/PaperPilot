export interface ToolWeekConfig {
  week: number;
  weekStr: string;
  slug: string;
  fullTitle: string;
  toolName: string;
  theme: string;
  toolsFeatured: string;
  mcpTool: string;
  cliCommand: string;
  apiEndpoint: string;
  cliSnippet: string;
  mcpSnippet: string;
  apiSnippet: string;
  cliReceiptLines: string[];
}

import toolsDataJson from '../configs_tools.json' with { type: 'json' };

export const TOOLS_DATA: Record<number, ToolWeekConfig> = (toolsDataJson as ToolWeekConfig[]).reduce(
  (acc, item) => {
    acc[item.week] = item;
    return acc;
  },
  {} as Record<number, ToolWeekConfig>
);

export const ALL_WEEKS = Object.keys(TOOLS_DATA).map(Number).sort((a, b) => a - b);

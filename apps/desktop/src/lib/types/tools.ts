export interface ToolConditionRule {
  supportedExtensions: string[]; // ['pdf'], ['xlsx', 'xls', 'csv'], ['docx'], or ['*']
  minDocuments?: number;         // e.g. merge requires >= 2
  maxDocuments?: number;
  requiredPack?: 'core' | 'office' | 'ocr' | 'dev';
  isBatchable?: boolean;         // Supports batch execution across multiple files
}

export interface ToolDefinition {
  id: string;
  title: string;
  description: string;
  icon: string;
  category: 'quick' | 'pages' | 'edit' | 'optimize' | 'security' | 'convert' | 'ai';
  context: 'pdf' | 'office' | 'ai';
  tags: string[];
  rule?: ToolConditionRule;
}

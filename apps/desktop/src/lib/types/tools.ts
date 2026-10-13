export interface ToolDefinition {
  id: string;
  title: string;
  description: string;
  icon: string;
  category: 'quick' | 'pages' | 'edit' | 'optimize' | 'security' | 'convert' | 'ai';
  context: 'pdf' | 'office' | 'ai';
  tags: string[];
}

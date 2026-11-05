import * as fs from 'fs';
import * as XLSX from 'xlsx';

/**
 * Deep Office & tabular document validator.
 * Validates spreadsheets, CSVs, and text formats.
 */
export class OfficeInspector {
  /**
   * Inspects Excel (.xlsx, .xls) and CSV spreadsheets.
   */
  public static inspectSpreadsheet(filePath: string): {
    sheetNames: string[];
    rowCount: (sheetName?: string) => number;
    getCellValue: (cellCoord: string, sheetName?: string) => any;
    toJson: (sheetName?: string) => any[];
  } {
    if (!fs.existsSync(filePath)) {
      throw new Error(`OfficeInspector Error: File not found at ${filePath}`);
    }

    const workbook = XLSX.readFile(filePath);
    const sheetNames = workbook.SheetNames;

    return {
      sheetNames,
      rowCount: (sheetName?: string) => {
        const targetSheet = sheetName ? workbook.Sheets[sheetName] : workbook.Sheets[sheetNames[0]];
        if (!targetSheet) return 0;
        const json = XLSX.utils.sheet_to_json(targetSheet);
        return json.length;
      },
      getCellValue: (cellCoord: string, sheetName?: string) => {
        const targetSheet = sheetName ? workbook.Sheets[sheetName] : workbook.Sheets[sheetNames[0]];
        if (!targetSheet || !targetSheet[cellCoord]) return undefined;
        return targetSheet[cellCoord].v;
      },
      toJson: (sheetName?: string) => {
        const targetSheet = sheetName ? workbook.Sheets[sheetName] : workbook.Sheets[sheetNames[0]];
        if (!targetSheet) return [];
        return XLSX.utils.sheet_to_json(targetSheet);
      }
    };
  }

  /**
   * Inspects structured text documents (Markdown, HTML, Plain text, JSON).
   */
  public static inspectText(filePath: string): {
    content: string;
    hasString: (str: string) => boolean;
    isJson: boolean;
    parsedJson?: any;
  } {
    if (!fs.existsSync(filePath)) {
      throw new Error(`OfficeInspector Error: File not found at ${filePath}`);
    }

    const content = fs.readFileSync(filePath, 'utf-8');
    let isJson = false;
    let parsedJson: any = undefined;

    try {
      parsedJson = JSON.parse(content);
      isJson = true;
    } catch (e) {}

    return {
      content,
      hasString: (str: string) => content.includes(str),
      isJson,
      parsedJson
    };
  }
}


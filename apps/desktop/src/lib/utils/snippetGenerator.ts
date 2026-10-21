export interface SnippetTarget {
  toolId: string; // e.g. 'pdf_rotate', 'pdf_bates', 'pdf_watermark', 'pdf_compress', 'pdf_encrypt', 'pdf_decrypt', 'pdf_redact', 'pdf_split', 'pdf_merge', etc., or 'document_workflow'
  intent?: string; // e.g. 'Rotate', 'Bates', 'Watermark', 'Compress', 'Encrypt', etc.
  params: Record<string, any>;
  inputFile?: string;
  outputFile?: string;
  inputs?: string[];
}

export interface GeneratedSnippets {
  cli: string;
  curl: string;
  mcpToolCall: string;
  mcpAgentPrompt: string;
  embedJs: string;
}

export function generateSnippets(target: SnippetTarget): GeneratedSnippets {
  const { toolId, intent, params, inputFile, outputFile, inputs } = target;

  const input = inputFile || params.input || 'input.pdf';
  const output = outputFile || params.output || 'output.pdf';

  let cli = '';
  let curl = '';
  let mcpToolCall = '';
  let mcpAgentPrompt = '';
  let embedJs = '';

  const jsonArgs = { ...params };
  if (inputFile) jsonArgs.input = input;
  if (outputFile) jsonArgs.output = output;

  switch (toolId) {
    case 'pdf_rotate': {
      const angle = params.angle || 90;
      const pagesStr = params.pages && params.pages !== 'all' ? ` --pages "${params.pages}"` : '';
      cli = `paperpilot rotate --input "${input}" --angle ${angle}${pagesStr} --output "${output}"`;
      curl = `curl -X POST http://localhost:8080/api/v1/tools/rotate -F "file=@${input}" -F "angle=${angle}"`;

      const mcpArgs: any = { input, angle, output };
      if (params.pages && params.pages !== 'all') mcpArgs.pages = params.pages;

      mcpToolCall = JSON.stringify({ name: "pdf_rotate", arguments: mcpArgs }, null, 2);
      mcpAgentPrompt = `Please rotate "${input}" by ${angle} degrees and save to "${output}".`;
      embedJs = `await window.PaperPilot.executeTool('pdf_rotate', ${JSON.stringify(mcpArgs)});`;
      break;
    }
    case 'pdf_bates': {
      const prefix = params.prefix || '';
      const start = params.start_number || 1;
      const padding = params.padding || 6;
      cli = `paperpilot bates --input "${input}" --prefix "${prefix}" --start ${start} --padding ${padding} --output "${output}"`;
      curl = `curl -X POST http://localhost:8080/api/v1/tools/bates -F "file=@${input}" -F "prefix=${prefix}" -F "start=${start}"`;

      const mcpArgs = { input, prefix, start_number: start, output };
      mcpToolCall = JSON.stringify({ name: "pdf_bates", arguments: mcpArgs }, null, 2);
      mcpAgentPrompt = `Apply Bates numbering to "${input}" starting with prefix "${prefix}" and save to "${output}".`;
      embedJs = `await window.PaperPilot.executeTool('pdf_bates', ${JSON.stringify(mcpArgs)});`;
      break;
    }
    case 'pdf_watermark': {
      const text = params.text || '';
      const opacity = params.opacity || 0.2;
      cli = `paperpilot watermark --input "${input}" --text "${text}" --opacity ${opacity} --output "${output}"`;
      curl = `curl -X POST http://localhost:8080/api/v1/tools/watermark -F "file=@${input}" -F "text=${text}"`;

      const mcpArgs = { input, text, output };
      mcpToolCall = JSON.stringify({ name: "pdf_watermark", arguments: mcpArgs }, null, 2);
      mcpAgentPrompt = `Apply watermark text "${text}" to "${input}" and save to "${output}".`;
      embedJs = `await window.PaperPilot.executeTool('pdf_watermark', ${JSON.stringify(mcpArgs)});`;
      break;
    }
    case 'pdf_sanitize': {
      const rm_metadata = params.remove_metadata !== undefined ? params.remove_metadata : true;
      const rm_attachments = params.remove_attachments !== undefined ? params.remove_attachments : true;
      const rm_scripts = params.remove_scripts !== undefined ? params.remove_scripts : true;
      cli = `paperpilot sanitize --input "${input}" --remove-metadata ${rm_metadata} --remove-attachments ${rm_attachments} --remove-scripts ${rm_scripts} --output "${output}"`;
      curl = `curl -X POST http://localhost:8080/api/v1/pdf/sanitize -H "Content-Type: application/json" -d '{"input": "${input}", "output": "${output}", "remove_metadata": ${rm_metadata}, "remove_attachments": ${rm_attachments}, "remove_scripts": ${rm_scripts}}'`;

      const mcpArgs = { input, output, remove_metadata: rm_metadata, remove_attachments: rm_attachments, remove_scripts: rm_scripts };
      mcpToolCall = JSON.stringify({ name: "pdf_sanitize", arguments: mcpArgs }, null, 2);
      mcpAgentPrompt = `Sanitize "${input}" and save to "${output}".`;
      embedJs = `await window.PaperPilot.executeTool('pdf_sanitize', ${JSON.stringify(mcpArgs)});`;
      break;
    }
    case 'pdf_encrypt': {
      const password = params.password || params.user_password || '';
      cli = `paperpilot encrypt --input "${input}" --password "${password}" --output "${output}"`;
      curl = `curl -X POST http://localhost:8080/api/v1/tools/encrypt -F "file=@${input}" -F "password=${password}"`;

      const mcpArgs = { input, user_password: password, output };
      mcpToolCall = JSON.stringify({ name: "pdf_encrypt", arguments: mcpArgs }, null, 2);
      mcpAgentPrompt = `Encrypt "${input}" with password "${password}" and save to "${output}".`;
      embedJs = `await window.PaperPilot.executeTool('pdf_encrypt', ${JSON.stringify({ input, password, output })});`;
      break;
    }
    case 'pdf_compress': {
      cli = `paperpilot compress --input "${input}" --quality 80 --output "${output}"`;
      curl = `curl -X POST http://localhost:8080/api/v1/tools/compress -F "file=@${input}" -F "quality=80"`;

      const mcpArgs = { input, output };
      mcpToolCall = JSON.stringify({ name: "pdf_compress", arguments: mcpArgs }, null, 2);
      mcpAgentPrompt = `Compress "${input}" and save to "${output}".`;
      embedJs = `await window.PaperPilot.executeTool('pdf_compress', ${JSON.stringify(mcpArgs)});`;
      break;
    }
    default: {
      // Fallback
      const subcommand = toolId.replace('pdf_', '');
      cli = `paperpilot ${subcommand} --input "${input}" --output "${output}"`;
      curl = `curl -X POST http://localhost:8080/api/v1/tools/${subcommand} -F "file=@${input}"`;

      const mcpArgs: any = { ...params };
      if (inputFile) mcpArgs.input = input;
      if (outputFile) mcpArgs.output = output;

      mcpToolCall = JSON.stringify({ name: toolId, arguments: mcpArgs }, null, 2);
      mcpAgentPrompt = `Perform ${intent || subcommand} on "${input}" and save to "${output}".`;
      embedJs = `await window.PaperPilot.executeTool('${toolId}', ${JSON.stringify(mcpArgs)});`;
      break;
    }
  }

  return {
    cli,
    curl,
    mcpToolCall,
    mcpAgentPrompt,
    embedJs
  };
}

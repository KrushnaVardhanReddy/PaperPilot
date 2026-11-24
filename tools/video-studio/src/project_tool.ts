import { makeProject } from '@revideo/core';
import deepdiveData from './configs/tools_deepdive.json' with { type: 'json' };
import { createSingleToolScene, SingleToolData, ToolCaptionsMetadata } from './scenes/single-tool-scene';
import { captionsData_tool_pdf_compress } from './scenes/captions_tool_pdf_compress';

const TARGET_TOOL_NAME = 'pdf_compress';
const toolEntry = (deepdiveData as SingleToolData[]).find(t => t.tool_name === TARGET_TOOL_NAME) || (deepdiveData as SingleToolData[])[0];

const scene = createSingleToolScene(toolEntry, captionsData_tool_pdf_compress as unknown as ToolCaptionsMetadata);

export default makeProject({
  scenes: [scene],
  settings: {
    shared: {
      size: { x: 1920, y: 1080 }, // 16:9 Landscape Full HD
      background: '#070a12',
    },
    rendering: {
      fps: 30,
    },
  },
});

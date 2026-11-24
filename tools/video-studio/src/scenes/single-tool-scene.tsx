import { makeScene2D } from '@revideo/2d';
import { createRef, all, waitFor } from '@revideo/core';
import { Audio, Rect, Txt } from '@revideo/2d';

export interface SingleToolData {
  id: number;
  tool_name: string;
  title: string;
  category: string;
  why_what: string;
  description: string;
  inputs: Array<{
    name: string;
    type: string;
    required: boolean;
    default: any;
    description: string;
  }>;
  outputs: {
    format: string;
    description: string;
    metrics: string;
  };
  interfaces: {
    gui: {
      view: string;
      controls: string[];
      action: string;
    };
    mcp: {
      server: string;
      tool: string;
      arguments: Record<string, any>;
    };
    cli: string;
    rest_api: string;
    wasm: {
      engine: string;
      runtime: string;
      security: string;
    };
  };
  website_url: string;
}

export interface ToolCaptionsMetadata {
  tool_name: string;
  title: string;
  category: string;
  duration: number;
  sections: Array<{
    id: string;
    title: string;
    part: string;
    start: number;
    end: number;
    text: string;
  }>;
  captions: Array<{
    text: string;
    start: number;
    end: number;
    words: Array<{ word: string; start: number; end: number; duration: number }>;
  }>;
}

export function createSingleToolScene(tool: SingleToolData, captionsData: ToolCaptionsMetadata) {
  return makeScene2D(`tool-${tool.tool_name}-deepdive-scene`, function* (view) {
    // Primary container references
    const mainStageRef = createRef<Rect>();

    // Header / Top Navigation Bar
    const navBarRef = createRef<Rect>();
    const stageBadgeTextRef = createRef<Txt>();
    const stageTitleRef = createRef<Txt>();

    // 5 Main Stages (Cards)
    const part1CardRef = createRef<Rect>(); // What is the Tool & Why (15s)
    const part2CardRef = createRef<Rect>(); // Input Parameters & What They Signify (20s)
    const part3CardRef = createRef<Rect>(); // Output Files & Metrics (15s)
    const part4CardRef = createRef<Rect>(); // All 5 Interfaces (25s)
    const part5CardRef = createRef<Rect>(); // Clean Website Call-to-Action (10s)

    // Interface sub-elements inside Part 4
    const interfaceTitleRef = createRef<Txt>();
    const ifaceGuiRef = createRef<Rect>();
    const ifaceMcpRef = createRef<Rect>();
    const ifaceCliRef = createRef<Rect>();
    const ifaceApiRef = createRef<Rect>();
    const ifaceWasmRef = createRef<Rect>();

    // Bottom Subtitle/Caption Banner
    const captionBarRef = createRef<Rect>();
    const captionTextRef = createRef<Txt>();

    // Add root scene hierarchy (1920x1080 Landscape Full HD)
    view.add(
      <Rect
        ref={mainStageRef}
        width={1920}
        height={1080}
        fill={'#070a12'}
        clip
      >
        {/* Dynamic Ambient Background Glow Elements */}
        <Rect
          width={1100}
          height={1100}
          radius={550}
          x={-550}
          y={-300}
          fill={'#1e3a8a'}
          opacity={0.16}
        />
        <Rect
          width={900}
          height={900}
          radius={450}
          x={600}
          y={300}
          fill={'#065f46'}
          opacity={0.14}
        />
        <Rect
          width={800}
          height={800}
          radius={400}
          x={300}
          y={-400}
          fill={'#4f46e5'}
          opacity={0.10}
        />

        {/* Voiceover Audio Stream (Tool #1) */}
        <Audio
          src={`/tool_${tool.tool_name}_voiceover.mp3`}
          play={true}
        />

        {/* Persistent Top Header / Branding Bar */}
        <Rect
          ref={navBarRef}
          y={-465}
          width={1800}
          height={76}
          fill={'#0d1424cc'}
          stroke={'#1e293b'}
          lineWidth={2}
          radius={18}
          layout
          direction={'row'}
          alignItems={'center'}
          justifyContent={'space-between'}
          paddingLeft={30}
          paddingRight={30}
        >
          <Rect layout direction={'row'} alignItems={'center'} gap={16}>
            <Rect
              fill={'#2563eb'}
              radius={8}
              paddingLeft={14}
              paddingRight={14}
              paddingTop={6}
              paddingBottom={6}
            >
              <Txt text={'PAPERPILOT DEEP DIVE'} fill={'#ffffff'} fontSize={16} fontWeight={800} letterSpacing={1} />
            </Rect>
            <Rect
              fill={'#1e293b'}
              stroke={'#3b82f6'}
              lineWidth={1}
              radius={8}
              paddingLeft={12}
              paddingRight={12}
              paddingTop={4}
              paddingBottom={4}
            >
              <Txt
                ref={stageBadgeTextRef}
                text={'PART 1 OF 5'}
                fill={'#38bdf8'}
                fontSize={15}
                fontWeight={700}
              />
            </Rect>
            <Txt
              ref={stageTitleRef}
              text={`Tool: ${tool.tool_name} — ${tool.title}`}
              fill={'#f1f5f9'}
              fontSize={20}
              fontWeight={700}
            />
          </Rect>

          <Rect layout direction={'row'} alignItems={'center'} gap={18}>
            <Txt text={tool.category} fill={'#38bdf8'} fontSize={16} fontWeight={700} />
            <Txt text={'•'} fill={'#475569'} fontSize={16} />
            <Txt text={'Pure Rust'} fill={'#f59e0b'} fontSize={16} fontWeight={700} />
            <Txt text={'•'} fill={'#475569'} fontSize={16} />
            <Txt text={'Zero Cloud Egress'} fill={'#10b981'} fontSize={16} fontWeight={700} />
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* PART 1: WHAT IS THE TOOL & WHY (15s) */}
        {/* ========================================================= */}
        <Rect
          ref={part1CardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#1e3a8a'}
          lineWidth={2}
          radius={24}
          layout
          direction={'row'}
          alignItems={'center'}
          justifyContent={'space-between'}
          padding={50}
          gap={40}
          opacity={1}
          shadowColor={'#000000a0'}
          shadowBlur={40}
        >
          {/* Left Column: Mission & Core Problem */}
          <Rect layout direction={'column'} width={920} gap={22}>
            <Rect
              fill={'#1d4ed8'}
              radius={12}
              paddingLeft={18}
              paddingRight={18}
              paddingTop={6}
              paddingBottom={6}
              alignSelf={'start'}
            >
              <Txt text={'PURE RUST • DETERMINISTIC'} fill={'#ffffff'} fontSize={18} fontWeight={800} letterSpacing={2} />
            </Rect>
            <Txt
              text={tool.title}
              fill={'#ffffff'}
              fontSize={60}
              fontWeight={900}
              lineHeight={68}
            />
            <Txt
              text={`tool: ${tool.tool_name}  •  ${tool.category}`}
              fill={'#38bdf8'}
              fontSize={26}
              fontWeight={700}
              fontFamily={'monospace'}
            />
            <Txt
              text={tool.why_what}
              fill={'#94a3b8'}
              fontSize={22}
              fontWeight={500}
              lineHeight={34}
            />
            <Rect layout direction={'row'} gap={20} marginTop={10}>
              <Rect
                fill={'#10b98120'}
                stroke={'#10b981'}
                lineWidth={1.5}
                radius={10}
                paddingLeft={18}
                paddingRight={18}
                paddingTop={10}
                paddingBottom={10}
              >
                <Txt text={'🔒 100% Offline Local'} fill={'#34d399'} fontSize={18} fontWeight={700} />
              </Rect>
              <Rect
                fill={'#6366f120'}
                stroke={'#6366f1'}
                lineWidth={1.5}
                radius={10}
                paddingLeft={18}
                paddingRight={18}
                paddingTop={10}
                paddingBottom={10}
              >
                <Txt text={'⚡ Sub-15ms Latency'} fill={'#818cf8'} fontSize={18} fontWeight={700} />
              </Rect>
              <Rect
                fill={'#f59e0b20'}
                stroke={'#f59e0b'}
                lineWidth={1.5}
                radius={10}
                paddingLeft={18}
                paddingRight={18}
                paddingTop={10}
                paddingBottom={10}
              >
                <Txt text={'💸 Zero SaaS Subscriptions'} fill={'#fbbf24'} fontSize={18} fontWeight={700} />
              </Rect>
            </Rect>
          </Rect>

          {/* Right Column: Architectural Pillars */}
          <Rect layout direction={'column'} width={660} gap={18}>
            <Rect
              fill={'#131c31'}
              stroke={'#1e293b'}
              lineWidth={1.5}
              radius={18}
              padding={22}
              layout
              direction={'column'}
              gap={8}
            >
              <Txt text={'🦀 Lopdf & Native AST Engine'} fill={'#f97316'} fontSize={22} fontWeight={800} />
              <Txt text={'Operates on PDF cross-reference tables and streams directly. No external Ghostscript or heavy C++ bindings.'} fill={'#cbd5e1'} fontSize={16} fontWeight={500} />
            </Rect>

            <Rect
              fill={'#131c31'}
              stroke={'#1e293b'}
              lineWidth={1.5}
              radius={18}
              padding={22}
              layout
              direction={'column'}
              gap={8}
            >
              <Txt text={'🛡️ Air-Gapped Data Privacy'} fill={'#10b981'} fontSize={22} fontWeight={800} />
              <Txt text={'Guaranteed 0 bytes cloud egress. Process confidential client records, tax filings, and legal filings with zero leak risk.'} fill={'#cbd5e1'} fontSize={16} fontWeight={500} />
            </Rect>

            <Rect
              fill={'#131c31'}
              stroke={'#1e293b'}
              lineWidth={1.5}
              radius={18}
              padding={22}
              layout
              direction={'column'}
              gap={8}
            >
              <Txt text={'🌐 5 Universal Interfaces'} fill={'#38bdf8'} fontSize={22} fontWeight={800} />
              <Txt text={'Desktop GUI, Claude AI Agent via MCP, Terminal CLI, Local REST Daemon, and Client-Side WebAssembly.'} fill={'#cbd5e1'} fontSize={16} fontWeight={500} />
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* PART 2: INPUT PARAMETERS & WHAT THEY SIGNIFY (20s) */}
        {/* ========================================================= */}
        <Rect
          ref={part2CardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#6366f1'}
          lineWidth={2}
          radius={24}
          layout
          direction={'column'}
          padding={40}
          gap={24}
          opacity={0}
          shadowColor={'#000000a0'}
          shadowBlur={40}
        >
          <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'} width={1660}>
            <Rect layout direction={'row'} alignItems={'center'} gap={16}>
              <Rect fill={'#4f46e5'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
                <Txt text={'SCHEMA SPECIFICATION'} fill={'#ffffff'} fontSize={16} fontWeight={800} />
              </Rect>
              <Txt text={`Input Parameters for ${tool.tool_name}`} fill={'#ffffff'} fontSize={28} fontWeight={800} />
            </Rect>
            <Txt text={'Rigorous Memory Validation & Type Safety'} fill={'#a5b4fc'} fontSize={18} fontWeight={600} />
          </Rect>

          {/* Parameters Grid */}
          <Rect layout direction={'row'} gap={24} width={1660}>
            {tool.inputs.map((param, idx) => (
              <Rect
                key={String(idx)}
                width={param.name === 'input' ? 520 : 520}
                height={480}
                fill={'#10172a'}
                stroke={'#334155'}
                lineWidth={1.5}
                radius={18}
                padding={26}
                layout
                direction={'column'}
                gap={16}
              >
                <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'}>
                  <Txt text={param.name} fill={'#38bdf8'} fontSize={26} fontWeight={900} fontFamily={'monospace'} />
                  <Rect
                    fill={param.required ? '#ef444420' : '#10b98120'}
                    stroke={param.required ? '#ef4444' : '#10b981'}
                    lineWidth={1}
                    radius={6}
                    paddingLeft={10}
                    paddingRight={10}
                    paddingTop={4}
                    paddingBottom={4}
                  >
                    <Txt text={param.required ? 'REQUIRED' : 'OPTIONAL'} fill={param.required ? '#f87171' : '#34d399'} fontSize={13} fontWeight={800} />
                  </Rect>
                </Rect>

                <Rect fill={'#1e293b'} radius={8} paddingLeft={12} paddingRight={12} paddingTop={6} paddingBottom={6} alignSelf={'start'}>
                  <Txt text={`type: ${param.type}`} fill={'#fbbf24'} fontSize={15} fontWeight={700} fontFamily={'monospace'} />
                </Rect>

                <Txt
                  text={param.description}
                  fill={'#cbd5e1'}
                  fontSize={18}
                  fontWeight={500}
                  lineHeight={28}
                />

                <Rect
                  fill={'#0b0f19'}
                  stroke={'#1e293b'}
                  lineWidth={1}
                  radius={12}
                  padding={16}
                  layout
                  direction={'column'}
                  gap={6}
                  marginTop={10}
                >
                  <Txt text={'Validation Behavior:'} fill={'#94a3b8'} fontSize={14} fontWeight={700} />
                  <Txt
                    text={param.name === 'input' ? '✔ Validates file exists & parses PDF trailer' : '✔ Matches preset "low"|"medium"|"high" or 1..100'}
                    fill={'#4ade80'}
                    fontSize={14}
                    fontFamily={'monospace'}
                  />
                  <Txt
                    text={param.name === 'input' ? '✔ Checks read permissions without memory copying' : '✔ Applies adaptive JPEG/Flate stream re-encoding'}
                    fill={'#4ade80'}
                    fontSize={14}
                    fontFamily={'monospace'}
                  />
                </Rect>
              </Rect>
            ))}

            {/* Parameter Execution Guarantees Card */}
            <Rect
              width={560}
              height={480}
              fill={'#0a1816'}
              stroke={'#059669'}
              lineWidth={2}
              radius={18}
              padding={26}
              layout
              direction={'column'}
              gap={18}
            >
              <Txt text={'Memory-Safe Execution Guarantees'} fill={'#34d399'} fontSize={24} fontWeight={800} />
              
              <Rect layout direction={'column'} gap={12}>
                <Rect fill={'#0f2620'} radius={12} padding={16} layout direction={'column'} gap={4}>
                  <Txt text={'🔒 Zero Out-of-Bounds Memory Safety'} fill={'#6ee7b7'} fontSize={17} fontWeight={700} />
                  <Txt text={'Pure Rust memory borrow-checker eliminates buffer overflows and segfault vulnerabilities.'} fill={'#94a3b8'} fontSize={14} />
                </Rect>

                <Rect fill={'#0f2620'} radius={12} padding={16} layout direction={'column'} gap={4}>
                  <Txt text={'⚡ Multithreaded Rayon Engine'} fill={'#6ee7b7'} fontSize={17} fontWeight={700} />
                  <Txt text={'Large stream decompression tasks run across all available CPU cores automatically.'} fill={'#94a3b8'} fontSize={14} />
                </Rect>

                <Rect fill={'#0f2620'} radius={12} padding={16} layout direction={'column'} gap={4}>
                  <Txt text={'📄 PDF Specification Conformance'} fill={'#6ee7b7'} fontSize={17} fontWeight={700} />
                  <Txt text={'Strictly adheres to ISO 32000-1 / ISO 32000-2 document structure specifications.'} fill={'#94a3b8'} fontSize={14} />
                </Rect>
              </Rect>
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* PART 3: OUTPUT FILES & METRICS (15s) */}
        {/* ========================================================= */}
        <Rect
          ref={part3CardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#10b981'}
          lineWidth={2}
          radius={24}
          layout
          direction={'column'}
          padding={40}
          gap={24}
          opacity={0}
          shadowColor={'#000000a0'}
          shadowBlur={40}
        >
          <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'} width={1660}>
            <Rect layout direction={'row'} alignItems={'center'} gap={16}>
              <Rect fill={'#059669'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
                <Txt text={'VERIFIED OUTPUTS'} fill={'#ffffff'} fontSize={16} fontWeight={800} />
              </Rect>
              <Txt text={`Output Artifacts & Benchmark Metrics`} fill={'#ffffff'} fontSize={28} fontWeight={800} />
            </Rect>
            <Txt text={'Cryptographic Verification • Instant Disk Commit'} fill={'#34d399'} fontSize={18} fontWeight={600} />
          </Rect>

          <Rect layout direction={'row'} gap={28} width={1660}>
            {/* Output File Representation */}
            <Rect
              width={780}
              height={480}
              fill={'#0b121e'}
              stroke={'#1e293b'}
              lineWidth={1.5}
              radius={18}
              padding={30}
              layout
              direction={'column'}
              gap={18}
            >
              <Txt text={'Generated Output File'} fill={'#f1f5f9'} fontSize={24} fontWeight={800} />
              <Rect fill={'#131d2e'} stroke={'#2563eb'} lineWidth={1.5} radius={14} padding={20} layout direction={'row'} alignItems={'center'} gap={20}>
                <Rect width={60} height={70} fill={'#ef444420'} stroke={'#ef4444'} lineWidth={2} radius={8} alignItems={'center'} justifyContent={'center'}>
                  <Txt text={'PDF'} fill={'#ef4444'} fontSize={20} fontWeight={900} />
                </Rect>
                <Rect layout direction={'column'} gap={4}>
                  <Txt text={'compressed_optimized.pdf'} fill={'#ffffff'} fontSize={22} fontWeight={700} />
                  <Txt text={tool.outputs.description} fill={'#94a3b8'} fontSize={15} />
                  <Txt text={'Format: PDF / Stream-Compressed (ISO-Compliant)'} fill={'#38bdf8'} fontSize={14} fontWeight={600} />
                </Rect>
              </Rect>

              <Rect fill={'#082f24'} stroke={'#059669'} lineWidth={1.5} radius={14} padding={20} layout direction={'column'} gap={8}>
                <Txt text={'✔ Size Reduction: 84.2 MB ➔ 16.8 MB (-80.0%)'} fill={'#4ade80'} fontSize={20} fontWeight={800} />
                <Txt text={'✔ Redundant streams and orphan XObjects purged'} fill={'#cbd5e1'} fontSize={15} />
                <Txt text={'✔ Cross-reference table linearized for instant web streaming'} fill={'#cbd5e1'} fontSize={15} />
              </Rect>
            </Rect>

            {/* Benchmark Metrics Counter Card */}
            <Rect
              width={850}
              height={480}
              fill={'#0a1816'}
              stroke={'#10b981'}
              lineWidth={2}
              radius={18}
              padding={30}
              layout
              direction={'column'}
              justifyContent={'space-between'}
            >
              <Txt text={'Live Engine Performance Telemetry'} fill={'#34d399'} fontSize={24} fontWeight={800} />

              <Rect layout direction={'row'} justifyContent={'space-between'} gap={16}>
                <Rect width={250} fill={'#0f2620'} stroke={'#164e40'} lineWidth={1} radius={14} padding={20} layout direction={'column'} alignItems={'center'}>
                  <Txt text={'12.4 ms'} fill={'#38bdf8'} fontSize={36} fontWeight={900} />
                  <Txt text={'Execution Time'} fill={'#cbd5e1'} fontSize={16} fontWeight={600} marginTop={4} />
                  <Txt text={'Rayon 16-Core SIMD'} fill={'#64748b'} fontSize={13} marginTop={4} />
                </Rect>

                <Rect width={250} fill={'#0f2620'} stroke={'#164e40'} lineWidth={1} radius={14} padding={20} layout direction={'column'} alignItems={'center'}>
                  <Txt text={'0 Bytes'} fill={'#4ade80'} fontSize={36} fontWeight={900} />
                  <Txt text={'Cloud Egress'} fill={'#cbd5e1'} fontSize={16} fontWeight={600} marginTop={4} />
                  <Txt text={'100% Offline Local'} fill={'#64748b'} fontSize={13} marginTop={4} />
                </Rect>

                <Rect width={250} fill={'#0f2620'} stroke={'#164e40'} lineWidth={1} radius={14} padding={20} layout direction={'column'} alignItems={'center'}>
                  <Txt text={'18.2 MB'} fill={'#fbbf24'} fontSize={36} fontWeight={900} />
                  <Txt text={'Peak RAM Usage'} fill={'#cbd5e1'} fontSize={16} fontWeight={600} marginTop={4} />
                  <Txt text={'Zero Memory Leaks'} fill={'#64748b'} fontSize={13} marginTop={4} />
                </Rect>
              </Rect>

              <Rect fill={'#064e3b'} radius={12} padding={16} layout direction={'row'} alignItems={'center'} gap={12}>
                <Txt text={'⚡'} fill={'#facc15'} fontSize={24} />
                <Txt text={'PaperPilot executes 15x faster than Python PyMuPDF and 40x faster than cloud REST APIs.'} fill={'#d1fae5'} fontSize={16} fontWeight={600} />
              </Rect>
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* PART 4: ALL 5 DEPLOYMENT INTERFACES (25s) */}
        {/* ========================================================= */}
        <Rect
          ref={part4CardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#3b82f6'}
          lineWidth={2}
          radius={24}
          layout
          direction={'column'}
          padding={34}
          gap={18}
          opacity={0}
          shadowColor={'#000000a0'}
          shadowBlur={40}
        >
          <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'} width={1672}>
            <Rect layout direction={'row'} alignItems={'center'} gap={16}>
              <Rect fill={'#2563eb'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
                <Txt text={'MULTI-INTERFACE ENGINE'} fill={'#ffffff'} fontSize={16} fontWeight={800} />
              </Rect>
              <Txt ref={interfaceTitleRef} text={'Universal Deployment Across 5 Interfaces'} fill={'#ffffff'} fontSize={28} fontWeight={800} />
            </Rect>
            <Txt text={'Desktop GUI • MCP Agent • CLI • REST API • Web/WASM'} fill={'#38bdf8'} fontSize={18} fontWeight={600} />
          </Rect>

          {/* Interface Tabs View Container */}
          <Rect width={1672} height={540} layout direction={'row'} gap={20}>
            {/* 1. GUI Card */}
            <Rect
              ref={ifaceGuiRef}
              width={318}
              height={540}
              fill={'#0f172a'}
              stroke={'#2563eb'}
              lineWidth={1.5}
              radius={16}
              padding={20}
              layout
              direction={'column'}
              justifyContent={'space-between'}
            >
              <Rect layout direction={'column'} gap={10}>
                <Rect fill={'#2563eb'} radius={6} paddingLeft={10} paddingRight={10} paddingTop={4} paddingBottom={4} alignSelf={'start'}>
                  <Txt text={'1. GUI'} fill={'#ffffff'} fontSize={13} fontWeight={800} />
                </Rect>
                <Txt text={'Desktop Dock'} fill={'#f1f5f9'} fontSize={20} fontWeight={800} />
                <Txt text={'Visual drag & drop with live side-by-side canvas inspector.'} fill={'#94a3b8'} fontSize={14} />
                <Rect fill={'#1e293b'} radius={10} padding={12} layout direction={'column'} gap={6}>
                  <Txt text={'✔ Slider controls'} fill={'#38bdf8'} fontSize={13} />
                  <Txt text={'✔ Real-time preview'} fill={'#38bdf8'} fontSize={13} />
                  <Txt text={'✔ Instant file export'} fill={'#38bdf8'} fontSize={13} />
                </Rect>
              </Rect>
              <Rect fill={'#1d4ed8'} radius={8} padding={10} alignItems={'center'}>
                <Txt text={'Open Desktop'} fill={'#ffffff'} fontSize={14} fontWeight={700} />
              </Rect>
            </Rect>

            {/* 2. MCP Card */}
            <Rect
              ref={ifaceMcpRef}
              width={318}
              height={540}
              fill={'#0a0f1d'}
              stroke={'#8b5cf6'}
              lineWidth={1.5}
              radius={16}
              padding={20}
              layout
              direction={'column'}
              justifyContent={'space-between'}
            >
              <Rect layout direction={'column'} gap={10}>
                <Rect fill={'#7c3aed'} radius={6} paddingLeft={10} paddingRight={10} paddingTop={4} paddingBottom={4} alignSelf={'start'}>
                  <Txt text={'2. MCP'} fill={'#ffffff'} fontSize={13} fontWeight={800} />
                </Rect>
                <Txt text={'AI Agent (Claude)'} fill={'#f1f5f9'} fontSize={20} fontWeight={800} />
                <Txt text={'Direct JSON-RPC tool invocation without leaving localhost.'} fill={'#94a3b8'} fontSize={14} />
                <Rect fill={'#161226'} stroke={'#4c1d95'} lineWidth={1} radius={10} padding={10} layout direction={'column'} gap={4}>
                  <Txt text={`"tool": "${tool.tool_name}"`} fill={'#c084fc'} fontSize={12} fontFamily={'monospace'} />
                  <Txt text={`"arguments": {`} fill={'#c084fc'} fontSize={12} fontFamily={'monospace'} />
                  <Txt text={`  "input": "file.pdf"`} fill={'#c084fc'} fontSize={12} fontFamily={'monospace'} />
                  <Txt text={`}`} fill={'#c084fc'} fontSize={12} fontFamily={'monospace'} />
                </Rect>
              </Rect>
              <Rect fill={'#6d28d9'} radius={8} padding={10} alignItems={'center'}>
                <Txt text={'MCP Connected'} fill={'#ffffff'} fontSize={14} fontWeight={700} />
              </Rect>
            </Rect>

            {/* 3. CLI Card */}
            <Rect
              ref={ifaceCliRef}
              width={318}
              height={540}
              fill={'#0a0e17'}
              stroke={'#10b981'}
              lineWidth={1.5}
              radius={16}
              padding={20}
              layout
              direction={'column'}
              justifyContent={'space-between'}
            >
              <Rect layout direction={'column'} gap={10}>
                <Rect fill={'#059669'} radius={6} paddingLeft={10} paddingRight={10} paddingTop={4} paddingBottom={4} alignSelf={'start'}>
                  <Txt text={'3. CLI'} fill={'#ffffff'} fontSize={13} fontWeight={800} />
                </Rect>
                <Txt text={'Terminal CLI'} fill={'#f1f5f9'} fontSize={20} fontWeight={800} />
                <Txt text={'Scriptable, zero-dependency command line for dev & CI/CD.'} fill={'#94a3b8'} fontSize={14} />
                <Rect fill={'#13241d'} stroke={'#064e3b'} lineWidth={1} radius={10} padding={10} layout direction={'column'} gap={4}>
                  <Txt text={'$ paperpilot \\'} fill={'#34d399'} fontSize={12} fontFamily={'monospace'} />
                  <Txt text={`  ${tool.tool_name.replace('pdf_', '')} \\`} fill={'#34d399'} fontSize={12} fontFamily={'monospace'} />
                  <Txt text={'  -i doc.pdf -o out.pdf'} fill={'#34d399'} fontSize={12} fontFamily={'monospace'} />
                </Rect>
              </Rect>
              <Rect fill={'#047857'} radius={8} padding={10} alignItems={'center'}>
                <Txt text={'Copy CLI Command'} fill={'#ffffff'} fontSize={14} fontWeight={700} />
              </Rect>
            </Rect>

            {/* 4. REST API Card */}
            <Rect
              ref={ifaceApiRef}
              width={318}
              height={540}
              fill={'#0d131f'}
              stroke={'#f59e0b'}
              lineWidth={1.5}
              radius={16}
              padding={20}
              layout
              direction={'column'}
              justifyContent={'space-between'}
            >
              <Rect layout direction={'column'} gap={10}>
                <Rect fill={'#d97706'} radius={6} paddingLeft={10} paddingRight={10} paddingTop={4} paddingBottom={4} alignSelf={'start'}>
                  <Txt text={'4. REST'} fill={'#ffffff'} fontSize={13} fontWeight={800} />
                </Rect>
                <Txt text={'Local REST API'} fill={'#f1f5f9'} fontSize={20} fontWeight={800} />
                <Txt text={'High-throughput local HTTP daemon at localhost:8080.'} fill={'#94a3b8'} fontSize={14} />
                <Rect fill={'#221d13'} stroke={'#78350f'} lineWidth={1} radius={10} padding={10} layout direction={'column'} gap={4}>
                  <Txt text={`POST /api/v1/${tool.tool_name}`} fill={'#fbbf24'} fontSize={12} fontFamily={'monospace'} />
                  <Txt text={'Host: localhost:8080'} fill={'#94a3b8'} fontSize={11} fontFamily={'monospace'} />
                  <Txt text={'Latency: 3.8ms'} fill={'#34d399'} fontSize={12} fontWeight={700} />
                </Rect>
              </Rect>
              <Rect fill={'#b45309'} radius={8} padding={10} alignItems={'center'}>
                <Txt text={'Test Endpoint'} fill={'#ffffff'} fontSize={14} fontWeight={700} />
              </Rect>
            </Rect>

            {/* 5. Web/WASM Card */}
            <Rect
              ref={ifaceWasmRef}
              width={318}
              height={540}
              fill={'#0a1324'}
              stroke={'#06b6d4'}
              lineWidth={1.5}
              radius={16}
              padding={20}
              layout
              direction={'column'}
              justifyContent={'space-between'}
            >
              <Rect layout direction={'column'} gap={10}>
                <Rect fill={'#0891b2'} radius={6} paddingLeft={10} paddingRight={10} paddingTop={4} paddingBottom={4} alignSelf={'start'}>
                  <Txt text={'5. WASM'} fill={'#ffffff'} fontSize={13} fontWeight={800} />
                </Rect>
                <Txt text={'Web / WebAssembly'} fill={'#f1f5f9'} fontSize={20} fontWeight={800} />
                <Txt text={'100% In-Browser execution on usepaperpilot.com.'} fill={'#94a3b8'} fontSize={14} />
                <Rect fill={'#102238'} stroke={'#164e63'} lineWidth={1} radius={10} padding={10} layout direction={'column'} gap={4}>
                  <Txt text={'usepaperpilot.com'} fill={'#22d3ee'} fontSize={12} fontFamily={'monospace'} />
                  <Txt text={'Zero Network Upload'} fill={'#4ade80'} fontSize={12} fontWeight={700} />
                  <Txt text={'Client Sandbox Active'} fill={'#38bdf8'} fontSize={12} />
                </Rect>
              </Rect>
              <Rect fill={'#0e7490'} radius={8} padding={10} alignItems={'center'}>
                <Txt text={'Launch In Browser'} fill={'#ffffff'} fontSize={14} fontWeight={700} />
              </Rect>
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* PART 5: CLEAN WEBSITE CALL-TO-ACTION (10s) */}
        {/* ========================================================= */}
        <Rect
          ref={part5CardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#2563eb'}
          lineWidth={3}
          radius={24}
          layout
          direction={'column'}
          alignItems={'center'}
          justifyContent={'center'}
          padding={50}
          gap={26}
          opacity={0}
          shadowColor={'#2563eb40'}
          shadowBlur={60}
        >
          <Rect
            fill={'#1d4ed8'}
            radius={12}
            paddingLeft={22}
            paddingRight={22}
            paddingTop={8}
            paddingBottom={8}
          >
            <Txt text={'GET STARTED WITH PAPERPILOT'} fill={'#ffffff'} fontSize={20} fontWeight={800} letterSpacing={2} />
          </Rect>

          <Txt
            text={'Explore All 100 Native Tools Online'}
            fill={'#ffffff'}
            fontSize={56}
            fontWeight={900}
            textAlign={'center'}
          />

          <Txt
            text={'Local-First Document Engineering • 100% Offline • Zero Subscription Fees'}
            fill={'#38bdf8'}
            fontSize={26}
            fontWeight={600}
          />

          {/* Primary CTA Box */}
          <Rect
            fill={'#131d33'}
            stroke={'#3b82f6'}
            lineWidth={2}
            radius={18}
            paddingLeft={40}
            paddingRight={40}
            paddingTop={18}
            paddingBottom={18}
            marginTop={10}
            layout
            direction={'row'}
            alignItems={'center'}
            gap={20}
          >
            <Txt text={'🌐'} fontSize={32} />
            <Txt
              text={'https://usepaperpilot.com'}
              fill={'#60a5fa'}
              fontSize={34}
              fontWeight={900}
              fontFamily={'monospace'}
            />
          </Rect>

          <Rect layout direction={'row'} gap={24} marginTop={12}>
            <Rect fill={'#10b98120'} stroke={'#10b981'} lineWidth={1.5} radius={10} paddingLeft={20} paddingRight={20} paddingTop={10} paddingBottom={10}>
              <Txt text={'📦 curl -fsSL https://usepaperpilot.com/install.sh | bash'} fill={'#34d399'} fontSize={18} fontFamily={'monospace'} fontWeight={700} />
            </Rect>
          </Rect>
        </Rect>

        {/* Dynamic Caption Bar Container */}
        <Rect
          ref={captionBarRef}
          y={440}
          width={1500}
          height={84}
          fill={'#090d16ee'}
          stroke={'#2563eb'}
          lineWidth={2}
          radius={18}
          layout
          direction={'row'}
          alignItems={'center'}
          justifyContent={'center'}
          paddingLeft={40}
          paddingRight={40}
          shadowColor={'#2563eb30'}
          shadowBlur={20}
        >
          <Txt
            ref={captionTextRef}
            text={`Welcome to ${tool.title}...`}
            fill={'#ffffff'}
            fontSize={28}
            fontWeight={800}
            textAlign={'center'}
          />
        </Rect>
      </Rect>
    );

    // Dynamic section transition timing derived from sections in captionsData
    const captions = captionsData.captions;
    const sections = captionsData.sections;

    // Retrieve part start times from metadata
    const t_part2 = sections.find(s => s.id === 'part2_inputs')?.start ?? 22.0;
    const t_part3 = sections.find(s => s.id === 'part3_outputs')?.start ?? 51.0;
    const t_part4 = sections.find(s => s.id === 'part4_interfaces')?.start ?? 72.0;
    const t_part5 = sections.find(s => s.id === 'part5_cta')?.start ?? 103.0;

    let activePart = 'part1_what_why';
    let currentTime = 0;

    for (let i = 0; i < captions.length; i++) {
      const item = captions[i];
      const waitBefore = Math.max(0, item.start - currentTime);
      if (waitBefore > 0) {
        yield* waitFor(waitBefore);
        currentTime += waitBefore;
      }

      captionTextRef().text(item.text);

      if (item.start >= t_part2 && activePart === 'part1_what_why') {
        activePart = 'part2_inputs';
        stageBadgeTextRef().text('PART 2 OF 5');
        stageTitleRef().text(`Input Parameters & Schema: ${tool.tool_name}`);
        yield* all(part1CardRef().opacity(0, 0.4), part2CardRef().opacity(1, 0.4));
        currentTime += 0.4;
      } else if (item.start >= t_part3 && activePart === 'part2_inputs') {
        activePart = 'part3_outputs';
        stageBadgeTextRef().text('PART 3 OF 5');
        stageTitleRef().text(`Outputs & Verified Metrics: ${tool.tool_name}`);
        yield* all(part2CardRef().opacity(0, 0.4), part3CardRef().opacity(1, 0.4));
        currentTime += 0.4;
      } else if (item.start >= t_part4 && activePart === 'part3_outputs') {
        activePart = 'part4_interfaces';
        stageBadgeTextRef().text('PART 4 OF 5');
        stageTitleRef().text(`5 Universal Interfaces: ${tool.tool_name}`);
        yield* all(part3CardRef().opacity(0, 0.4), part4CardRef().opacity(1, 0.4));
        currentTime += 0.4;
      } else if (item.start >= t_part5 && activePart === 'part4_interfaces') {
        activePart = 'part5_cta';
        stageBadgeTextRef().text('PART 5 OF 5');
        stageTitleRef().text('Get Started: https://usepaperpilot.com');
        yield* all(part4CardRef().opacity(0, 0.4), part5CardRef().opacity(1, 0.4));
        currentTime += 0.4;
      }

      const captionDuration = Math.max(0.1, item.end - currentTime);
      yield* waitFor(captionDuration);
      currentTime += captionDuration;
    }

    yield* waitFor(2.0);
  });
}

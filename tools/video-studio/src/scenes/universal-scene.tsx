import { makeScene2D } from '@revideo/2d';
import { createRef, all, waitFor } from '@revideo/core';
import { Audio, Rect, Txt } from '@revideo/2d';
import { ToolWeekConfig } from '../configs/tools-data';

export interface CaptionsMetadata {
  week: number;
  weekStr: string;
  duration: number;
  sections: Array<{ id: string; title: string; text: string }>;
  captions: Array<{
    text: string;
    start: number;
    end: number;
    words: Array<{ word: string; start: number; end: number; duration: number }>;
  }>;
}

export function createUniversalToolScene(tool: ToolWeekConfig, captionsData: CaptionsMetadata) {
  return makeScene2D(`week${tool.weekStr}-${tool.slug}-scene`, function* (view) {
    // Primary container references
    const mainStageRef = createRef<Rect>();

    // Header / Top Navigation Bar
    const navBarRef = createRef<Rect>();
    const stageBadgeTextRef = createRef<Txt>();
    const stageTitleRef = createRef<Txt>();

    // Center Content Stage (Cards for each section)
    const introCardRef = createRef<Rect>();
    const problemCardRef = createRef<Rect>();
    const toolCardRef = createRef<Rect>();
    const guiCardRef = createRef<Rect>();
    const agentCardRef = createRef<Rect>();
    const cliCardRef = createRef<Rect>();
    const apiCardRef = createRef<Rect>();
    const wasmCardRef = createRef<Rect>();
    const outroCardRef = createRef<Rect>();

    // Bottom Subtitle/Caption Banner
    const captionBarRef = createRef<Rect>();
    const captionTextRef = createRef<Txt>();

    // Add root scene hierarchy (1920x1080)
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
          width={1000}
          height={1000}
          radius={500}
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

        {/* Voiceover Audio Stream */}
        <Audio
          src={`../assets/week${tool.weekStr}_voiceover.mp3`}
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
          shadowColor={'#00000080'}
          shadowBlur={20}
        >
          <Rect layout direction={'row'} alignItems={'center'} gap={16}>
            <Rect
              fill={'#2563eb'}
              radius={10}
              paddingLeft={14}
              paddingRight={14}
              paddingTop={6}
              paddingBottom={6}
            >
              <Txt
                text={'PAPERPILOT'}
                fill={'#ffffff'}
                fontSize={18}
                fontWeight={900}
                letterSpacing={2}
              />
            </Rect>
            <Txt
              text={'Local-First Document & PDF Toolkit'}
              fill={'#94a3b8'}
              fontSize={18}
              fontWeight={600}
            />
          </Rect>

          <Rect layout direction={'row'} alignItems={'center'} gap={14}>
            <Rect
              fill={'#1e293b'}
              stroke={'#3b82f6'}
              lineWidth={1}
              radius={8}
              paddingLeft={14}
              paddingRight={14}
              paddingTop={5}
              paddingBottom={5}
            >
              <Txt
                ref={stageBadgeTextRef}
                text={'PART 1 OF 5'}
                fill={'#38bdf8'}
                fontSize={15}
                fontWeight={800}
                letterSpacing={1.5}
              />
            </Rect>
            <Txt
              ref={stageTitleRef}
              text={'Introduction & Core Architecture'}
              fill={'#f1f5f9'}
              fontSize={20}
              fontWeight={700}
            />
          </Rect>

          <Rect layout direction={'row'} alignItems={'center'} gap={18}>
            <Txt text={'100 Native Tools'} fill={'#38bdf8'} fontSize={16} fontWeight={700} />
            <Txt text={'•'} fill={'#475569'} fontSize={16} />
            <Txt text={'Zero Cloud Subscriptions'} fill={'#10b981'} fontSize={16} fontWeight={700} />
            <Txt text={'•'} fill={'#475569'} fontSize={16} />
            <Txt text={'Pure Rust'} fill={'#f59e0b'} fontSize={16} fontWeight={700} />
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* SECTION 1: INTRO CARD */}
        {/* ========================================================= */}
        <Rect
          ref={introCardRef}
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
          {/* Left Column */}
          <Rect layout direction={'column'} width={920} gap={24}>
            <Rect
              fill={'#1d4ed8'}
              radius={12}
              paddingLeft={18}
              paddingRight={18}
              paddingTop={6}
              paddingBottom={6}
              alignSelf={'start'}
            >
              <Txt text={'PURE RUST • LOCAL FIRST'} fill={'#ffffff'} fontSize={18} fontWeight={800} letterSpacing={2} />
            </Rect>
            <Txt
              text={'PaperPilot'}
              fill={'#ffffff'}
              fontSize={68}
              fontWeight={900}
              lineHeight={76}
            />
            <Txt
              text={'The Ultimate Document & PDF Engineering Suite'}
              fill={'#38bdf8'}
              fontSize={30}
              fontWeight={700}
            />
            <Txt
              text={'Files never leave your machine. Zero cloud subscriptions. One hundred native high-speed tools running directly on bare metal.'}
              fill={'#94a3b8'}
              fontSize={23}
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
                <Txt text={'🔒 100% Offline Privacy'} fill={'#34d399'} fontSize={18} fontWeight={700} />
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
                <Txt text={'⚡ Native Rust Speed'} fill={'#818cf8'} fontSize={18} fontWeight={700} />
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
                <Txt text={'📦 100 Tools in 1 Binary'} fill={'#fbbf24'} fontSize={18} fontWeight={700} />
              </Rect>
            </Rect>
          </Rect>

          {/* Right Column */}
          <Rect layout direction={'column'} width={660} gap={20}>
            <Rect
              fill={'#131c31'}
              stroke={'#1e293b'}
              lineWidth={1.5}
              radius={18}
              padding={24}
              layout
              direction={'column'}
              gap={8}
            >
              <Txt text={'🦀 Pure Rust Engine'} fill={'#f97316'} fontSize={24} fontWeight={800} />
              <Txt text={'Memory-safe, multi-threaded Rayon parallelism with zero garbage collection overhead.'} fill={'#cbd5e1'} fontSize={17} fontWeight={500} />
            </Rect>

            <Rect
              fill={'#131c31'}
              stroke={'#1e293b'}
              lineWidth={1.5}
              radius={18}
              padding={24}
              layout
              direction={'column'}
              gap={8}
            >
              <Txt text={'🛡️ Zero Cloud Egress'} fill={'#10b981'} fontSize={24} fontWeight={800} />
              <Txt text={'Your confidential financial reports, legal contracts, and medical records never touch remote servers.'} fill={'#cbd5e1'} fontSize={17} fontWeight={500} />
            </Rect>

            <Rect
              fill={'#131c31'}
              stroke={'#1e293b'}
              lineWidth={1.5}
              radius={18}
              padding={24}
              layout
              direction={'column'}
              gap={8}
            >
              <Txt text={'🌐 5 Seamless Interfaces'} fill={'#38bdf8'} fontSize={24} fontWeight={800} />
              <Txt text={'Desktop GUI, Claude AI Agent via MCP, Terminal CLI, Local REST API, and WebAssembly.'} fill={'#cbd5e1'} fontSize={17} fontWeight={500} />
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* SECTION 2: PROBLEM VS SOLUTION */}
        {/* ========================================================= */}
        <Rect
          ref={problemCardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#1e3a8a'}
          lineWidth={2}
          radius={24}
          layout
          direction={'row'}
          alignItems={'stretch'}
          justifyContent={'space-between'}
          padding={44}
          gap={36}
          opacity={0}
          shadowColor={'#000000a0'}
          shadowBlur={40}
        >
          {/* Left Card: The Problem */}
          <Rect
            fill={'#181014'}
            stroke={'#b91c1c'}
            lineWidth={2}
            radius={20}
            width={810}
            padding={36}
            layout
            direction={'column'}
            gap={20}
          >
            <Rect
              fill={'#ef444420'}
              stroke={'#ef4444'}
              lineWidth={1.5}
              radius={10}
              paddingLeft={16}
              paddingRight={16}
              paddingTop={6}
              paddingBottom={6}
              alignSelf={'start'}
            >
              <Txt text={'THE PROBLEM'} fill={'#f87171'} fontSize={18} fontWeight={900} letterSpacing={2} />
            </Rect>
            <Txt text={`Document Workflow Bottlenecks: ${tool.theme}`} fill={'#ffffff'} fontSize={30} fontWeight={800} />

            <Rect layout direction={'column'} gap={16}>
              <Rect fill={'#241217'} stroke={'#3f1b24'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={4}>
                <Txt text={'❌ Expensive SaaS Subscriptions & Quotas'} fill={'#fca5a5'} fontSize={20} fontWeight={700} />
                <Txt text={'Commercial document tools charge $20 to $50 per seat monthly, imposing strict rate limits and file size caps.'} fill={'#94a3b8'} fontSize={16} />
              </Rect>

              <Rect fill={'#241217'} stroke={'#3f1b24'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={4}>
                <Txt text={'❌ High Cloud Egress & Compliance Risks'} fill={'#fca5a5'} fontSize={20} fontWeight={700} />
                <Txt text={'Uploading proprietary legal exhibits, invoices, or medical files to remote cloud APIs violates GDPR, HIPAA, and SOC2.'} fill={'#94a3b8'} fontSize={16} />
              </Rect>

              <Rect fill={'#241217'} stroke={'#3f1b24'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={4}>
                <Txt text={'❌ Unreliable Black-Box Processing'} fill={'#fca5a5'} fontSize={20} fontWeight={700} />
                <Txt text={'Cloud APIs often silently alter document formatting, drop metadata, or hallucinate structured table schemas.'} fill={'#94a3b8'} fontSize={16} />
              </Rect>
            </Rect>
          </Rect>

          {/* Right Card: The Solution */}
          <Rect
            fill={'#0a1816'}
            stroke={'#059669'}
            lineWidth={2}
            radius={20}
            width={810}
            padding={36}
            layout
            direction={'column'}
            gap={20}
          >
            <Rect
              fill={'#10b98120'}
              stroke={'#10b981'}
              lineWidth={1.5}
              radius={10}
              paddingLeft={16}
              paddingRight={16}
              paddingTop={6}
              paddingBottom={6}
              alignSelf={'start'}
            >
              <Txt text={'THE PAPERPILOT SOLUTION'} fill={'#34d399'} fontSize={18} fontWeight={900} letterSpacing={2} />
            </Rect>
            <Txt text={'Deterministic High-Speed Native Engine'} fill={'#ffffff'} fontSize={30} fontWeight={800} />

            <Rect layout direction={'column'} gap={16}>
              <Rect fill={'#0f2620'} stroke={'#164e40'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={4}>
                <Txt text={`✅ Native ${tool.toolsFeatured}`} fill={'#6ee7b7'} fontSize={20} fontWeight={700} />
                <Txt text={'Engineered directly in pure Rust with strict AST preservation, cryptographic guarantees, and zero dependencies.'} fill={'#cbd5e1'} fontSize={16} />
              </Rect>

              <Rect fill={'#0f2620'} stroke={'#164e40'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={4}>
                <Txt text={'✅ 100% Air-Gapped Local Execution'} fill={'#6ee7b7'} fontSize={20} fontWeight={700} />
                <Txt text={'Runs entirely on localhost. Zero network calls, zero telemetry, and zero document egress guaranteed.'} fill={'#cbd5e1'} fontSize={16} />
              </Rect>

              {/* Performance Stats Metrics Callout */}
              <Rect
                fill={'#064e3b'}
                stroke={'#10b981'}
                lineWidth={1.5}
                radius={14}
                padding={16}
                layout
                direction={'row'}
                alignItems={'center'}
                justifyContent={'space-around'}
              >
                <Rect layout direction={'column'} alignItems={'center'}>
                  <Txt text={'Sub-15ms'} fill={'#38bdf8'} fontSize={28} fontWeight={900} />
                  <Txt text={'Execution Time'} fill={'#cbd5e1'} fontSize={15} fontWeight={600} />
                </Rect>
                <Rect width={2} height={44} fill={'#047857'} />
                <Rect layout direction={'column'} alignItems={'center'}>
                  <Txt text={'100%'} fill={'#4ade80'} fontSize={28} fontWeight={900} />
                  <Txt text={'Offline / Private'} fill={'#cbd5e1'} fontSize={15} fontWeight={600} />
                </Rect>
                <Rect width={2} height={44} fill={'#047857'} />
                <Rect layout direction={'column'} alignItems={'center'}>
                  <Txt text={'0 Bytes'} fill={'#fbbf24'} fontSize={28} fontWeight={900} />
                  <Txt text={'Data Egress'} fill={'#cbd5e1'} fontSize={15} fontWeight={600} />
                </Rect>
              </Rect>
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* SECTION 3: TOOL ANNOUNCEMENT */}
        {/* ========================================================= */}
        <Rect
          ref={toolCardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#3b82f6'}
          lineWidth={3}
          radius={24}
          layout
          direction={'column'}
          alignItems={'center'}
          justifyContent={'center'}
          padding={60}
          gap={26}
          opacity={0}
          shadowColor={'#3b82f640'}
          shadowBlur={50}
        >
          <Rect
            fill={'#3b82f6'}
            radius={14}
            paddingLeft={24}
            paddingRight={24}
            paddingTop={8}
            paddingBottom={8}
          >
            <Txt text={`FEATURED SHOWCASE • WEEK ${tool.weekStr} OF 26`} fill={'#ffffff'} fontSize={22} fontWeight={900} letterSpacing={2} />
          </Rect>

          <Txt
            text={tool.toolName}
            fill={'#ffffff'}
            fontSize={58}
            fontWeight={900}
            textAlign={'center'}
          />

          <Txt
            text={`${tool.theme} • 5 Flexible Deployment Interfaces`}
            fill={'#38bdf8'}
            fontSize={28}
            fontWeight={700}
          />

          <Rect
            fill={'#1e293b'}
            stroke={'#475569'}
            lineWidth={2}
            radius={16}
            paddingLeft={34}
            paddingRight={34}
            paddingTop={14}
            paddingBottom={14}
            marginTop={10}
          >
            <Txt
              text={`paperpilot ${tool.toolsFeatured.split(',')[0].trim()}  •  ${tool.mcpTool}  •  ${tool.apiEndpoint}`}
              fill={'#4ade80'}
              fontSize={26}
              fontWeight={800}
              fontFamily={'monospace'}
            />
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* SECTION 4.1: INTERFACE 1 - DESKTOP GUI */}
        {/* ========================================================= */}
        <Rect
          ref={guiCardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#2563eb'}
          lineWidth={2}
          radius={24}
          layout
          direction={'column'}
          padding={34}
          gap={20}
          opacity={0}
          shadowColor={'#000000a0'}
          shadowBlur={40}
        >
          <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'} width={1672}>
            <Rect layout direction={'row'} alignItems={'center'} gap={16}>
              <Rect fill={'#2563eb'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
                <Txt text={'INTERFACE 1 OF 5'} fill={'#ffffff'} fontSize={16} fontWeight={800} />
              </Rect>
              <Txt text={`🖥️ Desktop GUI: Visual Dock & Real-Time Inspector`} fill={'#ffffff'} fontSize={28} fontWeight={800} />
            </Rect>
            <Txt text={'Tauri v2 + Native OS Performance'} fill={'#94a3b8'} fontSize={18} fontWeight={600} />
          </Rect>

          <Rect
            width={1672}
            height={540}
            fill={'#0b1120'}
            stroke={'#334155'}
            lineWidth={2}
            radius={18}
            clip
            layout
            direction={'column'}
          >
            {/* Window Chrome Header */}
            <Rect width={1672} height={44} fill={'#1e293b'} layout direction={'row'} alignItems={'center'} paddingLeft={20} gap={10}>
              <Rect width={14} height={14} radius={7} fill={'#ef4444'} />
              <Rect width={14} height={14} radius={7} fill={'#f59e0b'} />
              <Rect width={14} height={14} radius={7} fill={'#10b981'} />
              <Txt text={`PaperPilot Desktop — ${tool.toolName}`} fill={'#94a3b8'} fontSize={16} fontWeight={600} marginLeft={20} />
            </Rect>

            {/* Window Body: Input -> Controls -> Output */}
            <Rect width={1672} height={496} layout direction={'row'} padding={28} gap={28}>
              {/* Input Zone */}
              <Rect
                width={480}
                height={440}
                fill={'#131c31'}
                stroke={'#3b82f6'}
                lineWidth={2}
                radius={16}
                layout
                direction={'column'}
                alignItems={'center'}
                justifyContent={'center'}
                padding={24}
                gap={14}
              >
                <Rect width={70} height={80} fill={'#ef444420'} stroke={'#ef4444'} lineWidth={2} radius={10} layout direction={'column'} alignItems={'center'} justifyContent={'center'}>
                  <Txt text={'PDF'} fill={'#ef4444'} fontSize={22} fontWeight={900} />
                </Rect>
                <Txt text={'enterprise_document.pdf'} fill={'#ffffff'} fontSize={20} fontWeight={700} />
                <Txt text={'Original Input: 42 Pages'} fill={'#38bdf8'} fontSize={17} fontWeight={700} />
                <Rect fill={'#1e293b'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
                  <Txt text={'Verified Local File Descriptor'} fill={'#94a3b8'} fontSize={14} />
                </Rect>
                <Txt text={'📥 Drag & Drop Active'} fill={'#4ade80'} fontSize={15} fontWeight={700} marginTop={6} />
              </Rect>

              {/* Controls */}
              <Rect
                width={540}
                height={440}
                fill={'#131c31'}
                stroke={'#1e293b'}
                lineWidth={1.5}
                radius={16}
                layout
                direction={'column'}
                padding={24}
                gap={18}
              >
                <Txt text={'Operational Configuration'} fill={'#f1f5f9'} fontSize={22} fontWeight={800} />
                <Rect layout direction={'column'} gap={10}>
                  <Txt text={`Tool: ${tool.toolsFeatured}`} fill={'#cbd5e1'} fontSize={16} fontWeight={600} />
                  <Rect fill={'#0f172a'} radius={10} padding={14} layout direction={'column'} gap={6}>
                    <Txt text={`✔ Mode: Native deterministic ${tool.theme}`} fill={'#4ade80'} fontSize={14} />
                    <Txt text={'✔ Cryptographic integrity validation active'} fill={'#4ade80'} fontSize={14} />
                    <Txt text={'✔ Multi-threaded Rayon execution (16 cores)'} fill={'#4ade80'} fontSize={14} />
                  </Rect>
                </Rect>
                <Rect fill={'#10b981'} radius={10} padding={14} alignItems={'center'} justifyContent={'center'} marginTop={18}>
                  <Txt text={'⚡ Execute Operation (Instant)'} fill={'#ffffff'} fontSize={18} fontWeight={800} />
                </Rect>
              </Rect>

              {/* Output */}
              <Rect
                width={540}
                height={440}
                fill={'#131c31'}
                stroke={'#10b981'}
                lineWidth={2}
                radius={16}
                layout
                direction={'column'}
                padding={24}
                gap={16}
              >
                <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'}>
                  <Txt text={'Operation Receipt'} fill={'#f1f5f9'} fontSize={22} fontWeight={800} />
                  <Rect fill={'#10b98120'} stroke={'#10b981'} lineWidth={1} radius={6} paddingLeft={10} paddingRight={10} paddingTop={4} paddingBottom={4}>
                    <Txt text={'STATUS: SUCCESS'} fill={'#34d399'} fontSize={14} fontWeight={800} />
                  </Rect>
                </Rect>
                <Rect fill={'#082f24'} stroke={'#059669'} lineWidth={1} radius={12} padding={16} layout direction={'column'} gap={6}>
                  <Txt text={'output_verified.pdf'} fill={'#ffffff'} fontSize={18} fontWeight={700} />
                  <Txt text={'Execution Time: 12.4ms'} fill={'#4ade80'} fontSize={22} fontWeight={900} />
                  <Txt text={'0 Bytes sent to cloud (100% Offline)'} fill={'#94a3b8'} fontSize={15} />
                </Rect>
                <Rect fill={'#2563eb'} radius={10} padding={12} alignItems={'center'} justifyContent={'center'} marginTop={14}>
                  <Txt text={'💾 Open Output File'} fill={'#ffffff'} fontSize={17} fontWeight={800} />
                </Rect>
              </Rect>
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* SECTION 4.2: INTERFACE 2 - AI AGENT / MCP */}
        {/* ========================================================= */}
        <Rect
          ref={agentCardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#8b5cf6'}
          lineWidth={2}
          radius={24}
          layout
          direction={'column'}
          padding={34}
          gap={20}
          opacity={0}
          shadowColor={'#000000a0'}
          shadowBlur={40}
        >
          <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'} width={1672}>
            <Rect layout direction={'row'} alignItems={'center'} gap={16}>
              <Rect fill={'#7c3aed'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
                <Txt text={'INTERFACE 2 OF 5'} fill={'#ffffff'} fontSize={16} fontWeight={800} />
              </Rect>
              <Txt text={'🤖 AI Agent / MCP: Local Tool Invocations'} fill={'#ffffff'} fontSize={28} fontWeight={800} />
            </Rect>
            <Txt text={'Model Context Protocol (JSON-RPC 2.0)'} fill={'#c084fc'} fontSize={18} fontWeight={600} />
          </Rect>

          <Rect width={1672} height={540} layout direction={'row'} gap={24}>
            {/* Claude Prompt */}
            <Rect
              width={824}
              height={540}
              fill={'#0f172a'}
              stroke={'#334155'}
              lineWidth={1.5}
              radius={18}
              padding={24}
              layout
              direction={'column'}
              gap={16}
            >
              <Txt text={'Claude Desktop / AI Agent Prompt:'} fill={'#94a3b8'} fontSize={18} fontWeight={700} />
              <Rect fill={'#1e293b'} stroke={'#475569'} lineWidth={1} radius={12} padding={18}>
                <Txt
                  text={`"Execute ${tool.toolName} on document.pdf and return verified results."`}
                  fill={'#f1f5f9'}
                  fontSize={20}
                  fontWeight={600}
                />
              </Rect>
              <Rect fill={'#064e3b'} radius={10} padding={14} layout direction={'row'} alignItems={'center'} gap={10}>
                <Txt text={'✓'} fill={'#34d399'} fontSize={22} fontWeight={900} />
                <Txt text={`Agent dispatched ${tool.mcpTool} in 14ms locally.`} fill={'#34d399'} fontSize={17} fontWeight={700} />
              </Rect>
            </Rect>

            {/* MCP Payload */}
            <Rect
              width={824}
              height={540}
              fill={'#0a0f1d'}
              stroke={'#8b5cf6'}
              lineWidth={2}
              radius={18}
              padding={24}
              layout
              direction={'column'}
              gap={14}
            >
              <Txt text={`⚡ MCP Tool Call: ${tool.mcpTool}`} fill={'#a855f7'} fontSize={20} fontWeight={800} fontFamily={'monospace'} />
              <Rect fill={'#0f172a'} stroke={'#1e293b'} lineWidth={1} radius={12} padding={16} layout direction={'column'} gap={6}>
                <Txt text={'// Tool Arguments (Input):'} fill={'#64748b'} fontSize={14} fontFamily={'monospace'} />
                <Txt
                  text={tool.mcpSnippet.slice(0, 300)}
                  fill={'#38bdf8'}
                  fontSize={14}
                  fontFamily={'monospace'}
                  lineHeight={20}
                />
              </Rect>
              <Rect fill={'#0f172a'} stroke={'#10b981'} lineWidth={1} radius={12} padding={16} layout direction={'column'} gap={6}>
                <Txt text={'// Tool Result (Output):'} fill={'#10b981'} fontSize={14} fontFamily={'monospace'} />
                <Txt
                  text={`{\n  "status": "success",\n  "tool": "${tool.mcpTool}",\n  "offline": true,\n  "elapsed_ms": 11.2\n}`}
                  fill={'#4ade80'}
                  fontSize={14}
                  fontFamily={'monospace'}
                  lineHeight={20}
                />
              </Rect>
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* SECTION 4.3: INTERFACE 3 - TERMINAL CLI */}
        {/* ========================================================= */}
        <Rect
          ref={cliCardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#10b981'}
          lineWidth={2}
          radius={24}
          layout
          direction={'column'}
          padding={34}
          gap={20}
          opacity={0}
          shadowColor={'#000000a0'}
          shadowBlur={40}
        >
          <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'} width={1672}>
            <Rect layout direction={'row'} alignItems={'center'} gap={16}>
              <Rect fill={'#059669'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
                <Txt text={'INTERFACE 3 OF 5'} fill={'#ffffff'} fontSize={16} fontWeight={800} />
              </Rect>
              <Txt text={'💻 Terminal CLI: Native Command-Line Speed'} fill={'#ffffff'} fontSize={28} fontWeight={800} />
            </Rect>
            <Txt text={'Scriptable, CI/CD Ready & Zero Dependencies'} fill={'#34d399'} fontSize={18} fontWeight={600} />
          </Rect>

          <Rect
            width={1672}
            height={540}
            fill={'#0a0e17'}
            stroke={'#1e293b'}
            lineWidth={2}
            radius={18}
            clip
            layout
            direction={'column'}
          >
            <Rect width={1672} height={44} fill={'#161f30'} layout direction={'row'} alignItems={'center'} paddingLeft={20} gap={10}>
              <Rect width={14} height={14} radius={7} fill={'#ef4444'} />
              <Rect width={14} height={14} radius={7} fill={'#f59e0b'} />
              <Rect width={14} height={14} radius={7} fill={'#10b981'} />
              <Txt text={'bash — krushna@workstation: ~/documents'} fill={'#94a3b8'} fontSize={16} fontWeight={600} marginLeft={20} />
            </Rect>

            <Rect width={1672} height={496} padding={30} layout direction={'column'} gap={14}>
              <Rect layout direction={'row'} alignItems={'center'} gap={14}>
                <Txt text={'$'} fill={'#10b981'} fontSize={24} fontWeight={900} fontFamily={'monospace'} />
                <Txt
                  text={tool.cliCommand}
                  fill={'#38bdf8'}
                  fontSize={22}
                  fontWeight={700}
                  fontFamily={'monospace'}
                />
              </Rect>

              <Rect
                fill={'#0f172a'}
                stroke={'#334155'}
                lineWidth={1.5}
                radius={14}
                padding={22}
                layout
                direction={'column'}
                gap={6}
              >
                {tool.cliReceiptLines.map((line, idx) => (
                  <Txt
                    key={String(idx)}
                    text={line}
                    fill={line.includes('✔') ? '#4ade80' : line.includes('──') ? '#475569' : '#cbd5e1'}
                    fontSize={16}
                    fontWeight={line.includes('✔') ? 800 : 600}
                    fontFamily={'monospace'}
                  />
                ))}
              </Rect>
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* SECTION 4.4: INTERFACE 4 - LOCAL REST API */}
        {/* ========================================================= */}
        <Rect
          ref={apiCardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#f59e0b'}
          lineWidth={2}
          radius={24}
          layout
          direction={'column'}
          padding={34}
          gap={20}
          opacity={0}
          shadowColor={'#000000a0'}
          shadowBlur={40}
        >
          <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'} width={1672}>
            <Rect layout direction={'row'} alignItems={'center'} gap={16}>
              <Rect fill={'#d97706'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
                <Txt text={'INTERFACE 4 OF 5'} fill={'#ffffff'} fontSize={16} fontWeight={800} />
              </Rect>
              <Txt text={'🌐 Local REST API: High-Throughput HTTP Microservice'} fill={'#ffffff'} fontSize={28} fontWeight={800} />
            </Rect>
            <Txt text={'Axum Native Rust Server • Sub-5ms Overhead'} fill={'#fbbf24'} fontSize={18} fontWeight={600} />
          </Rect>

          <Rect width={1672} height={540} layout direction={'row'} gap={24}>
            {/* Left: Request */}
            <Rect
              width={824}
              height={540}
              fill={'#0d131f'}
              stroke={'#f59e0b'}
              lineWidth={1.5}
              radius={18}
              padding={24}
              layout
              direction={'column'}
              gap={14}
            >
              <Txt text={'HTTP Request (Local Daemon):'} fill={'#f59e0b'} fontSize={18} fontWeight={800} />
              <Rect fill={'#1e2538'} radius={10} padding={16} layout direction={'column'} gap={4}>
                <Txt text={`POST ${tool.apiEndpoint} HTTP/1.1`} fill={'#fbbf24'} fontSize={16} fontFamily={'monospace'} fontWeight={700} />
                <Txt text={'Host: 127.0.0.1:7823'} fill={'#94a3b8'} fontSize={14} fontFamily={'monospace'} />
                <Txt text={'Content-Type: application/json'} fill={'#94a3b8'} fontSize={14} fontFamily={'monospace'} />
              </Rect>
              <Rect fill={'#131a2a'} radius={10} padding={16} layout direction={'column'} gap={4}>
                <Txt text={'{\n  "input": "document.pdf",\n  "verify": true\n}'} fill={'#38bdf8'} fontSize={14} fontFamily={'monospace'} />
              </Rect>
            </Rect>

            {/* Right: Response */}
            <Rect
              width={824}
              height={540}
              fill={'#0a1816'}
              stroke={'#10b981'}
              lineWidth={2}
              radius={18}
              padding={24}
              layout
              direction={'column'}
              gap={14}
            >
              <Txt text={'HTTP 200 OK — JSON Response:'} fill={'#34d399'} fontSize={18} fontWeight={800} />
              <Rect fill={'#0f2620'} radius={10} padding={18} layout direction={'column'} gap={6}>
                <Txt text={`{\n  "status": "success",\n  "tool": "${tool.toolsFeatured.split(',')[0].trim()}",\n  "offline": true,\n  "cloud_bytes_egress": 0,\n  "latency_ms": 4.2\n}`} fill={'#4ade80'} fontSize={15} fontFamily={'monospace'} lineHeight={22} />
              </Rect>
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* SECTION 4.5: INTERFACE 5 - WEB / WASM */}
        {/* ========================================================= */}
        <Rect
          ref={wasmCardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#0c1322f2'}
          stroke={'#06b6d4'}
          lineWidth={2}
          radius={24}
          layout
          direction={'column'}
          padding={34}
          gap={20}
          opacity={0}
          shadowColor={'#000000a0'}
          shadowBlur={40}
        >
          <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'} width={1672}>
            <Rect layout direction={'row'} alignItems={'center'} gap={16}>
              <Rect fill={'#0891b2'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
                <Txt text={'INTERFACE 5 OF 5'} fill={'#ffffff'} fontSize={16} fontWeight={800} />
              </Rect>
              <Txt text={'⚡ Web / WASM: 100% In-Browser Zero-Upload Processing'} fill={'#ffffff'} fontSize={28} fontWeight={800} />
            </Rect>
            <Txt text={'usepaperpilot.com • Client-Side WebAssembly Sandbox'} fill={'#22d3ee'} fontSize={18} fontWeight={600} />
          </Rect>

          <Rect width={1672} height={540} fill={'#0a1324'} stroke={'#164e63'} lineWidth={2} radius={18} clip layout direction={'column'}>
            <Rect width={1672} height={44} fill={'#15283f'} layout direction={'row'} alignItems={'center'} paddingLeft={20} gap={10}>
              <Rect width={14} height={14} radius={7} fill={'#ef4444'} />
              <Rect width={14} height={14} radius={7} fill={'#f59e0b'} />
              <Rect width={14} height={14} radius={7} fill={'#10b981'} />
              <Txt text={'https://usepaperpilot.com/tools/' + tool.slug} fill={'#38bdf8'} fontSize={15} fontWeight={600} marginLeft={20} />
            </Rect>

            <Rect width={1672} height={496} layout direction={'row'} padding={30} gap={30}>
              <Rect width={800} height={436} fill={'#101e38'} stroke={'#0284c7'} lineWidth={1.5} radius={16} layout direction={'column'} alignItems={'center'} justifyContent={'center'} padding={30} gap={16}>
                <Txt text={'Client-Side Document Processing'} fill={'#ffffff'} fontSize={26} fontWeight={800} />
                <Txt text={'No files are ever uploaded. Processing runs entirely in your local browser sandbox.'} fill={'#94a3b8'} fontSize={17} textAlign={'center'} />
                <Rect fill={'#0891b2'} radius={10} paddingLeft={20} paddingRight={20} paddingTop={10} paddingBottom={10}>
                  <Txt text={'Select Local Document'} fill={'#ffffff'} fontSize={16} fontWeight={700} />
                </Rect>
              </Rect>

              <Rect width={800} height={436} fill={'#0f172a'} stroke={'#10b981'} lineWidth={2} radius={16} layout direction={'column'} padding={26} gap={16}>
                <Txt text={'WebAssembly Execution Pipeline'} fill={'#ffffff'} fontSize={24} fontWeight={800} />
                <Rect layout direction={'column'} gap={12}>
                  <Rect fill={'#172238'} radius={10} padding={14} layout direction={'row'} alignItems={'center'} justifyContent={'space-between'}>
                    <Txt text={'1. WASM Memory Sandbox'} fill={'#94a3b8'} fontSize={16} />
                    <Txt text={'64 MB Allocated'} fill={'#38bdf8'} fontSize={16} fontWeight={700} />
                  </Rect>
                  <Rect fill={'#172238'} radius={10} padding={14} layout direction={'row'} alignItems={'center'} justifyContent={'space-between'}>
                    <Txt text={'2. Network Traffic Inspection'} fill={'#94a3b8'} fontSize={16} />
                    <Txt text={'0 Bytes Sent (Verified)'} fill={'#4ade80'} fontSize={16} fontWeight={700} />
                  </Rect>
                  <Rect fill={'#172238'} radius={10} padding={14} layout direction={'row'} alignItems={'center'} justifyContent={'space-between'}>
                    <Txt text={'3. Rust WASM SIMD Optimization'} fill={'#94a3b8'} fontSize={16} />
                    <Txt text={'Sub-50ms Execution'} fill={'#fbbf24'} fontSize={16} fontWeight={700} />
                  </Rect>
                </Rect>
                <Rect fill={'#10b98120'} stroke={'#10b981'} lineWidth={1} radius={10} padding={14} alignItems={'center'}>
                  <Txt text={'🔒 Client-Side Guarantee: Air-Gapped and Offline Safe'} fill={'#34d399'} fontSize={16} fontWeight={800} />
                </Rect>
              </Rect>
            </Rect>
          </Rect>
        </Rect>

        {/* ========================================================= */}
        {/* SECTION 5: OUTRO & INSTALLATION */}
        {/* ========================================================= */}
        <Rect
          ref={outroCardRef}
          y={-40}
          width={1740}
          height={680}
          fill={'#090d16fa'}
          stroke={'#38bdf8'}
          lineWidth={3}
          radius={24}
          layout
          direction={'column'}
          alignItems={'center'}
          justifyContent={'center'}
          padding={50}
          gap={24}
          opacity={0}
          shadowColor={'#38bdf860'}
          shadowBlur={50}
        >
          <Rect fill={'#10b981'} radius={14} paddingLeft={24} paddingRight={24} paddingTop={8} paddingBottom={8}>
            <Txt text={'100% OFFLINE • OPEN SOURCE • ZERO SUBSCRIPTIONS'} fill={'#ffffff'} fontSize={20} fontWeight={900} letterSpacing={2} />
          </Rect>

          <Txt text={'Claim Complete Document Sovereignty'} fill={'#ffffff'} fontSize={56} fontWeight={900} />
          <Txt text={'One Native Rust Binary. 100 Tools. 5 Interfaces.'} fill={'#38bdf8'} fontSize={28} fontWeight={700} />

          <Rect width={1300} fill={'#131c31'} stroke={'#38bdf8'} lineWidth={2} radius={16} padding={22} layout direction={'column'} alignItems={'center'} gap={10}>
            <Txt text={'5-SECOND FLASH INSTALL:'} fill={'#94a3b8'} fontSize={16} fontWeight={800} letterSpacing={1.5} />
            <Txt text={'curl -fsSL https://usepaperpilot.com/install.sh | bash'} fill={'#4ade80'} fontSize={32} fontWeight={900} fontFamily={'monospace'} />
          </Rect>

          <Rect layout direction={'row'} gap={30} marginTop={10}>
            <Rect fill={'#0369a1'} stroke={'#38bdf8'} lineWidth={2} radius={16} paddingLeft={40} paddingRight={40} paddingTop={16} paddingBottom={16} layout direction={'row'} alignItems={'center'} gap={16}>
              <Txt text={'🌐 Explore All Tools:'} fill={'#ffffff'} fontSize={26} fontWeight={800} />
              <Txt text={'https://usepaperpilot.com'} fill={'#38bdf8'} fontSize={26} fontWeight={800} />
            </Rect>
          </Rect>
        </Rect>

        {/* BOTTOM CAPTION BAR */}
        <Rect
          ref={captionBarRef}
          y={465}
          width={1800}
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
            text={'Welcome to PaperPilot...'}
            fill={'#ffffff'}
            fontSize={30}
            fontWeight={800}
            textAlign={'center'}
          />
        </Rect>
      </Rect>
    );

    // Dynamic section transition timing derived from sections or fallback times
    const captions = captionsData.captions;
    const totalDuration = captionsData.duration;
    
    // Calculate proportional section switch times based on section audio timestamps
    // Sections: [intro, problem_solution, tool_focus, interface_gui, interface_agent, interface_cli, interface_api, interface_wasm, outro]
    const secCount = captionsData.sections.length;
    // We compute approximate thresholds:
    const t_problem = totalDuration * (13.5 / 146.0);
    const t_tool = totalDuration * (45.8 / 146.0);
    const t_gui = totalDuration * (52.8 / 146.0);
    const t_agent = totalDuration * (69.0 / 146.0);
    const t_cli = totalDuration * (86.8 / 146.0);
    const t_api = totalDuration * (100.0 / 146.0);
    const t_wasm = totalDuration * (114.7 / 146.0);
    const t_outro = totalDuration * (128.4 / 146.0);

    let activeSection = 'intro';
    let currentTime = 0;

    for (let i = 0; i < captions.length; i++) {
      const item = captions[i];
      const waitBefore = Math.max(0, item.start - currentTime);
      if (waitBefore > 0) {
        yield* waitFor(waitBefore);
        currentTime += waitBefore;
      }

      captionTextRef().text(item.text);

      if (item.start >= t_problem && activeSection === 'intro') {
        activeSection = 'problem_solution';
        stageBadgeTextRef().text('PART 2 OF 5');
        stageTitleRef().text(`The Problem & Solution: ${tool.theme}`);
        yield* all(introCardRef().opacity(0, 0.4), problemCardRef().opacity(1, 0.4));
        currentTime += 0.4;
      } else if (item.start >= t_tool && activeSection === 'problem_solution') {
        activeSection = 'tool_focus';
        stageBadgeTextRef().text('PART 3 OF 5');
        stageTitleRef().text(`Tool ${tool.weekStr}: ${tool.toolName}`);
        yield* all(problemCardRef().opacity(0, 0.4), toolCardRef().opacity(1, 0.4));
        currentTime += 0.4;
      } else if (item.start >= t_gui && activeSection === 'tool_focus') {
        activeSection = 'interface_gui';
        stageBadgeTextRef().text('PART 4 OF 5 (1/5)');
        stageTitleRef().text('Interface 1: Desktop GUI');
        yield* all(toolCardRef().opacity(0, 0.4), guiCardRef().opacity(1, 0.4));
        currentTime += 0.4;
      } else if (item.start >= t_agent && activeSection === 'interface_gui') {
        activeSection = 'interface_agent';
        stageBadgeTextRef().text('PART 4 OF 5 (2/5)');
        stageTitleRef().text('Interface 2: AI Agent (Claude / MCP)');
        yield* all(guiCardRef().opacity(0, 0.4), agentCardRef().opacity(1, 0.4));
        currentTime += 0.4;
      } else if (item.start >= t_cli && activeSection === 'interface_agent') {
        activeSection = 'interface_cli';
        stageBadgeTextRef().text('PART 4 OF 5 (3/5)');
        stageTitleRef().text('Interface 3: Terminal CLI');
        yield* all(agentCardRef().opacity(0, 0.4), cliCardRef().opacity(1, 0.4));
        currentTime += 0.4;
      } else if (item.start >= t_api && activeSection === 'interface_cli') {
        activeSection = 'interface_api';
        stageBadgeTextRef().text('PART 4 OF 5 (4/5)');
        stageTitleRef().text('Interface 4: Local REST API');
        yield* all(cliCardRef().opacity(0, 0.4), apiCardRef().opacity(1, 0.4));
        currentTime += 0.4;
      } else if (item.start >= t_wasm && activeSection === 'interface_api') {
        activeSection = 'interface_wasm';
        stageBadgeTextRef().text('PART 4 OF 5 (5/5)');
        stageTitleRef().text('Interface 5: Web / WASM');
        yield* all(apiCardRef().opacity(0, 0.4), wasmCardRef().opacity(1, 0.4));
        currentTime += 0.4;
      } else if (item.start >= t_outro && activeSection === 'interface_wasm') {
        activeSection = 'outro';
        stageBadgeTextRef().text('PART 5 OF 5');
        stageTitleRef().text('Outro & Installation');
        yield* all(wasmCardRef().opacity(0, 0.4), outroCardRef().opacity(1, 0.4));
        currentTime += 0.4;
      }

      const captionDuration = Math.max(0.1, item.end - currentTime);
      yield* waitFor(captionDuration);
      currentTime += captionDuration;
    }

    yield* waitFor(2.0);
  });
}

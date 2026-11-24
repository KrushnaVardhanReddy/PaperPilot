import { makeScene2D } from '@revideo/2d';
import { createRef, all, waitFor, easeInOutCubic, easeOutBack, linear } from '@revideo/core';
import { Audio, Rect, Txt, Layout } from '@revideo/2d';
import { captionsData } from './captions';

export default makeScene2D('week01-compression-scene', function* (view) {
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
        src={'../assets/week01_voiceover.mp3'}
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
      {/* SECTION 1: INTRO CARD (0.0s - 13.5s) */}
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
        {/* Left Column: Big typography & highlights */}
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

        {/* Right Column: Architectural Feature Pillars */}
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
      {/* SECTION 2: PROBLEM VS SOLUTION (13.5s - 45.9s) */}
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
        {/* Left Card: The Problem (Red accented) */}
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
          <Txt text={'Broken PDF Workflows & Security Hazards'} fill={'#ffffff'} fontSize={32} fontWeight={800} />

          <Rect layout direction={'column'} gap={16}>
            <Rect fill={'#241217'} stroke={'#3f1b24'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={4}>
              <Txt text={'❌ Massive PDF File Bloat'} fill={'#fca5a5'} fontSize={20} fontWeight={700} />
              <Txt text={'High-res scans, uncompressed object streams, and embedded fonts bloat documents to 50MB+, breaking email and portal caps.'} fill={'#94a3b8'} fontSize={16} />
            </Rect>

            <Rect fill={'#241217'} stroke={'#3f1b24'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={4}>
              <Txt text={'❌ Severe Data Leak Risks'} fill={'#fca5a5'} fontSize={20} fontWeight={700} />
              <Txt text={'Third-party web compressors store your unencrypted PDF files on external cloud servers, violating GDPR, HIPAA, and SOC2.'} fill={'#94a3b8'} fontSize={16} />
            </Rect>

            <Rect fill={'#241217'} stroke={'#3f1b24'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={4}>
              <Txt text={'❌ Expensive Monthly Subscriptions'} fill={'#fca5a5'} fontSize={20} fontWeight={700} />
              <Txt text={'SaaS tools charge $15 to $40 per user every month just to shrink file sizes with restrictive daily quotas.'} fill={'#94a3b8'} fontSize={16} />
            </Rect>
          </Rect>
        </Rect>

        {/* Right Card: The Solution (Green/Blue accented) */}
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
          <Txt text={'Offline High-Speed Native Rust Engine'} fill={'#ffffff'} fontSize={32} fontWeight={800} />

          <Rect layout direction={'column'} gap={16}>
            <Rect fill={'#0f2620'} stroke={'#164e40'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={4}>
              <Txt text={'✅ Content Stream Deduplication'} fill={'#6ee7b7'} fontSize={20} fontWeight={700} />
              <Txt text={'Identifies identical XObjects, streams, and resources, unifying references and wiping duplicate data.'} fill={'#cbd5e1'} fontSize={16} />
            </Rect>

            <Rect fill={'#0f2620'} stroke={'#164e40'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={4}>
              <Txt text={'✅ Font Subsetting & Smart Downsampling'} fill={'#6ee7b7'} fontSize={20} fontWeight={700} />
              <Txt text={'Strips unused font glyphs and re-encodes oversized image payloads losslessly or with tuned perceptual quality.'} fill={'#cbd5e1'} fontSize={16} />
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
                <Txt text={'237 docs/s'} fill={'#38bdf8'} fontSize={28} fontWeight={900} />
                <Txt text={'Throughput'} fill={'#cbd5e1'} fontSize={15} fontWeight={600} />
              </Rect>
              <Rect width={2} height={44} fill={'#047857'} />
              <Rect layout direction={'column'} alignItems={'center'}>
                <Txt text={'Up to ~75%'} fill={'#4ade80'} fontSize={28} fontWeight={900} />
                <Txt text={'Size Reduction'} fill={'#cbd5e1'} fontSize={15} fontWeight={600} />
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
      {/* SECTION 3: TOOL 01 ANNOUNCEMENT (45.9s - 52.8s) */}
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
          <Txt text={'FEATURED SHOWCASE • TOOL 01 OF 100'} fill={'#ffffff'} fontSize={22} fontWeight={900} letterSpacing={2} />
        </Rect>

        <Txt
          text={'Instant PDF Compression'}
          fill={'#ffffff'}
          fontSize={68}
          fontWeight={900}
        />

        <Txt
          text={'Native Rust Optimization Engine • 5 Flexible Deployment Interfaces'}
          fill={'#38bdf8'}
          fontSize={30}
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
            text={'paperpilot compress  •  paperpilot_compress_pdf  •  /v1/compress'}
            fill={'#4ade80'}
            fontSize={28}
            fontWeight={800}
            fontFamily={'monospace'}
          />
        </Rect>
      </Rect>

      {/* ========================================================= */}
      {/* SECTION 4.1: INTERFACE 1 - DESKTOP GUI (52.8s - 69.0s) */}
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
        {/* Sub-Header */}
        <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'} width={1672}>
          <Rect layout direction={'row'} alignItems={'center'} gap={16}>
            <Rect fill={'#2563eb'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
              <Txt text={'INTERFACE 1 OF 5'} fill={'#ffffff'} fontSize={16} fontWeight={800} />
            </Rect>
            <Txt text={'🖥️ Desktop GUI: Drag & Drop Preset Optimization'} fill={'#ffffff'} fontSize={28} fontWeight={800} />
          </Rect>
          <Txt text={'Tauri v2 + React Native OS Performance'} fill={'#94a3b8'} fontSize={18} fontWeight={600} />
        </Rect>

        {/* Mock Desktop Window Frame */}
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
            <Txt text={'PaperPilot Desktop — Compress PDF'} fill={'#94a3b8'} fontSize={16} fontWeight={600} marginLeft={20} />
          </Rect>

          {/* Window Body: Input -> Controls -> Output Preview */}
          <Rect width={1672} height={496} layout direction={'row'} padding={28} gap={28}>
            {/* Input PDF Drop Zone */}
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
              <Txt text={'quarterly-report-2026.pdf'} fill={'#ffffff'} fontSize={20} fontWeight={700} />
              <Txt text={'Original Size: 48.6 MB'} fill={'#f87171'} fontSize={17} fontWeight={700} />
              <Rect fill={'#1e293b'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
                <Txt text={'42 Pages • 300 DPI Images'} fill={'#94a3b8'} fontSize={14} />
              </Rect>
              <Txt text={'📥 Drag & Drop Input Target'} fill={'#38bdf8'} fontSize={15} fontWeight={700} marginTop={6} />
            </Rect>

            {/* Middle: Interactive Slider & Mode Selection */}
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
              <Txt text={'Compression Settings'} fill={'#f1f5f9'} fontSize={22} fontWeight={800} />

              <Rect layout direction={'column'} gap={8}>
                <Txt text={'Optimization Preset:'} fill={'#cbd5e1'} fontSize={16} fontWeight={600} />
                <Rect layout direction={'row'} gap={10}>
                  <Rect fill={'#1e293b'} radius={8} padding={10} width={150} alignItems={'center'}><Txt text={'Light'} fill={'#94a3b8'} fontSize={15} /></Rect>
                  <Rect fill={'#2563eb'} radius={8} padding={10} width={160} alignItems={'center'}><Txt text={'Medium (Balanced)'} fill={'#ffffff'} fontSize={15} fontWeight={700} /></Rect>
                  <Rect fill={'#1e293b'} radius={8} padding={10} width={150} alignItems={'center'}><Txt text={'Maximum'} fill={'#94a3b8'} fontSize={15} /></Rect>
                </Rect>
              </Rect>

              {/* Slider visualization */}
              <Rect layout direction={'column'} gap={8} marginTop={6}>
                <Rect layout direction={'row'} justifyContent={'space-between'}>
                  <Txt text={'Quality Target'} fill={'#cbd5e1'} fontSize={16} />
                  <Txt text={'75% (Recommended)'} fill={'#38bdf8'} fontSize={16} fontWeight={700} />
                </Rect>
                <Rect width={492} height={12} fill={'#1e293b'} radius={6} layout direction={'row'} alignItems={'center'}>
                  <Rect width={370} height={12} fill={'#3b82f6'} radius={6} />
                  <Rect width={24} height={24} radius={12} fill={'#ffffff'} shadowColor={'#000000'} shadowBlur={6} />
                </Rect>
              </Rect>

              <Rect fill={'#0f172a'} radius={10} padding={14} layout direction={'column'} gap={6}>
                <Txt text={'✔ Content stream deduplication active'} fill={'#4ade80'} fontSize={14} />
                <Txt text={'✔ Font subsetting active (4 fonts subset)'} fill={'#4ade80'} fontSize={14} />
                <Txt text={'✔ Image re-sampling: 150 DPI target'} fill={'#4ade80'} fontSize={14} />
              </Rect>

              <Rect fill={'#10b981'} radius={10} padding={12} alignItems={'center'} justifyContent={'center'}>
                <Txt text={'⚡ Instant Compress (One Click)'} fill={'#ffffff'} fontSize={18} fontWeight={800} />
              </Rect>
            </Rect>

            {/* Right: Output Preview & Real-time Stats */}
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
                <Txt text={'Output Result'} fill={'#f1f5f9'} fontSize={22} fontWeight={800} />
                <Rect fill={'#10b98120'} stroke={'#10b981'} lineWidth={1} radius={6} paddingLeft={10} paddingRight={10} paddingTop={4} paddingBottom={4}>
                  <Txt text={'SAVED 74.5%'} fill={'#34d399'} fontSize={14} fontWeight={800} />
                </Rect>
              </Rect>

              <Rect fill={'#082f24'} stroke={'#059669'} lineWidth={1} radius={12} padding={16} layout direction={'column'} gap={6}>
                <Txt text={'quarterly-report-2026_compressed.pdf'} fill={'#ffffff'} fontSize={18} fontWeight={700} />
                <Txt text={'12.4 MB (Reduced from 48.6 MB)'} fill={'#4ade80'} fontSize={22} fontWeight={900} />
                <Txt text={'Processing Time: 86 milliseconds'} fill={'#94a3b8'} fontSize={15} />
              </Rect>

              {/* Before vs After comparison bar */}
              <Rect layout direction={'column'} gap={8}>
                <Txt text={'Size Comparison:'} fill={'#94a3b8'} fontSize={15} />
                <Rect layout direction={'column'} gap={6}>
                  <Rect width={492} height={20} fill={'#ef4444'} radius={6} layout direction={'row'} alignItems={'center'} paddingLeft={10}>
                    <Txt text={'Before: 48.6 MB (100%)'} fill={'#ffffff'} fontSize={13} fontWeight={700} />
                  </Rect>
                  <Rect width={126} height={20} fill={'#10b981'} radius={6} layout direction={'row'} alignItems={'center'} paddingLeft={10}>
                    <Txt text={'After: 12.4 MB (25.5%)'} fill={'#ffffff'} fontSize={13} fontWeight={700} />
                  </Rect>
                </Rect>
              </Rect>

              <Rect fill={'#2563eb'} radius={10} padding={12} alignItems={'center'} justifyContent={'center'} marginTop={8}>
                <Txt text={'💾 Open Output in System Folder'} fill={'#ffffff'} fontSize={17} fontWeight={800} />
              </Rect>
            </Rect>
          </Rect>
        </Rect>
      </Rect>

      {/* ========================================================= */}
      {/* SECTION 4.2: INTERFACE 2 - AI AGENT / MCP (69.0s - 86.8s) */}
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
            <Rect fill={'#8b5cf6'} radius={8} paddingLeft={14} paddingRight={14} paddingTop={6} paddingBottom={6}>
              <Txt text={'INTERFACE 2 OF 5'} fill={'#ffffff'} fontSize={16} fontWeight={800} />
            </Rect>
            <Txt text={'🤖 AI Agent: Autonomous MCP Protocol Server'} fill={'#ffffff'} fontSize={28} fontWeight={800} />
          </Rect>
          <Txt text={'Compatible with Claude Desktop, Cursor, Gemini & Cline'} fill={'#c084fc'} fontSize={18} fontWeight={600} />
        </Rect>

        {/* 2-Column Split: User Prompt / Claude Chat vs Native MCP Tool Execution */}
        <Rect width={1672} height={540} layout direction={'row'} gap={24}>
          {/* Left: Chat Interaction */}
          <Rect
            width={824}
            height={540}
            fill={'#10172a'}
            stroke={'#334155'}
            lineWidth={2}
            radius={18}
            padding={24}
            layout
            direction={'column'}
            gap={18}
          >
            <Rect layout direction={'row'} alignItems={'center'} gap={12}>
              <Rect width={36} height={36} radius={18} fill={'#c2410c'} alignItems={'center'} justifyContent={'center'}>
                <Txt text={'👤'} fontSize={18} />
              </Rect>
              <Txt text={'User Prompt to Claude Desktop'} fill={'#f1f5f9'} fontSize={20} fontWeight={700} />
            </Rect>

            <Rect fill={'#1e293b'} stroke={'#475569'} lineWidth={1} radius={14} padding={18}>
              <Txt
                text={'"Compress quarterly-report.pdf under 10MB so I can upload it to our board portal."'}
                fill={'#ffffff'}
                fontSize={19}
                fontWeight={600}
                lineHeight={28}
              />
            </Rect>

            <Rect layout direction={'row'} alignItems={'center'} gap={12} marginTop={8}>
              <Rect width={36} height={36} radius={18} fill={'#d97706'} alignItems={'center'} justifyContent={'center'}>
                <Txt text={'🤖'} fontSize={18} />
              </Rect>
              <Txt text={'Claude Autonomous Reasoning'} fill={'#fbbf24'} fontSize={20} fontWeight={700} />
            </Rect>

            <Rect fill={'#172033'} stroke={'#8b5cf6'} lineWidth={1} radius={14} padding={18} layout direction={'column'} gap={10}>
              <Txt
                text={'I will inspect the file size (48.6 MB) and invoke the PaperPilot MCP tool to apply balanced compression under 10MB target.'}
                fill={'#cbd5e1'}
                fontSize={16}
                lineHeight={24}
              />
              <Rect fill={'#064e3b'} radius={8} padding={12} layout direction={'row'} alignItems={'center'} gap={10}>
                <Txt text={'✓'} fill={'#34d399'} fontSize={20} fontWeight={900} />
                <Txt text={'Compression complete: 8.9 MB output generated in 72ms.'} fill={'#34d399'} fontSize={16} fontWeight={700} />
              </Rect>
            </Rect>
          </Rect>

          {/* Right: MCP Tool Call Protocol Inspection */}
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
            <Rect layout direction={'row'} justifyContent={'space-between'} alignItems={'center'}>
              <Txt text={'⚡ MCP Tool Call: paperpilot_compress_pdf'} fill={'#a855f7'} fontSize={20} fontWeight={800} fontFamily={'monospace'} />
              <Rect fill={'#8b5cf620'} radius={6} paddingLeft={10} paddingRight={10} paddingTop={4} paddingBottom={4}>
                <Txt text={'JSON-RPC 2.0'} fill={'#c084fc'} fontSize={13} fontWeight={700} />
              </Rect>
            </Rect>

            {/* Input Payload */}
            <Rect fill={'#0f172a'} stroke={'#1e293b'} lineWidth={1} radius={12} padding={16} layout direction={'column'} gap={6}>
              <Txt text={'// Tool Arguments (Input):'} fill={'#64748b'} fontSize={14} fontFamily={'monospace'} />
              <Txt
                text={'{\n  "path": "/data/quarterly-report.pdf",\n  "target_size_mb": 10.0,\n  "quality": "balanced",\n  "subset_fonts": true\n}'}
                fill={'#38bdf8'}
                fontSize={15}
                fontFamily={'monospace'}
                lineHeight={22}
              />
            </Rect>

            {/* Output Payload */}
            <Rect fill={'#0f172a'} stroke={'#10b981'} lineWidth={1} radius={12} padding={16} layout direction={'column'} gap={6}>
              <Txt text={'// Tool Execution Result (Output):'} fill={'#10b981'} fontSize={14} fontFamily={'monospace'} />
              <Txt
                text={'{\n  "status": "success",\n  "output_path": "/data/quarterly-report_compressed.pdf",\n  "original_bytes": 50960793,\n  "compressed_bytes": 9332326,\n  "savings_percent": 81.69,\n  "elapsed_ms": 72.4\n}'}
                fill={'#4ade80'}
                fontSize={15}
                fontFamily={'monospace'}
                lineHeight={22}
              />
            </Rect>
          </Rect>
        </Rect>
      </Rect>

      {/* ========================================================= */}
      {/* SECTION 4.3: INTERFACE 3 - TERMINAL CLI (86.8s - 100.0s) */}
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

        {/* Mock Terminal Window */}
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
          {/* Terminal Title Bar */}
          <Rect width={1672} height={44} fill={'#161f30'} layout direction={'row'} alignItems={'center'} paddingLeft={20} gap={10}>
            <Rect width={14} height={14} radius={7} fill={'#ef4444'} />
            <Rect width={14} height={14} radius={7} fill={'#f59e0b'} />
            <Rect width={14} height={14} radius={7} fill={'#10b981'} />
            <Txt text={'bash — krushna@workstation: ~/documents'} fill={'#94a3b8'} fontSize={16} fontWeight={600} marginLeft={20} />
          </Rect>

          {/* Terminal Console Output */}
          <Rect width={1672} height={496} padding={30} layout direction={'column'} gap={14}>
            {/* Command Line Input */}
            <Rect layout direction={'row'} alignItems={'center'} gap={14}>
              <Txt text={'$'} fill={'#10b981'} fontSize={24} fontWeight={900} fontFamily={'monospace'} />
              <Txt
                text={'paperpilot compress input.pdf -o output.pdf --quality medium'}
                fill={'#38bdf8'}
                fontSize={24}
                fontWeight={700}
                fontFamily={'monospace'}
              />
            </Rect>

            {/* Terminal Formatted Receipt Output */}
            <Rect
              fill={'#0f172a'}
              stroke={'#334155'}
              lineWidth={1.5}
              radius={14}
              padding={24}
              layout
              direction={'column'}
              gap={8}
            >
              <Txt text={'[PaperPilot v1.0.0] High-Speed Compression Engine'} fill={'#f1f5f9'} fontSize={18} fontWeight={700} fontFamily={'monospace'} />
              <Txt text={'───────────────────────────────────────────────────────────────────────'} fill={'#475569'} fontSize={16} fontFamily={'monospace'} />
              <Txt text={'[1/3] Parsing document xref & content streams ... [OK] 12ms'} fill={'#94a3b8'} fontSize={16} fontFamily={'monospace'} />
              <Txt text={'[2/3] Deduplicating streams & subsetting 4 embedded fonts ... [OK] 34ms'} fill={'#94a3b8'} fontSize={16} fontFamily={'monospace'} />
              <Txt text={'[3/3] Resampling images (300 DPI -> 150 DPI balanced) ... [OK] 42ms'} fill={'#94a3b8'} fontSize={16} fontFamily={'monospace'} />
              <Txt text={'───────────────────────────────────────────────────────────────────────'} fill={'#475569'} fontSize={16} fontFamily={'monospace'} />
              <Txt text={'✔ Compression Complete:'} fill={'#4ade80'} fontSize={20} fontWeight={800} fontFamily={'monospace'} />
              <Txt text={'   Original Size   : 48,602,112 bytes (48.6 MB)'} fill={'#f87171'} fontSize={17} fontFamily={'monospace'} />
              <Txt text={'   Compressed Size : 12,410,880 bytes (12.4 MB)'} fill={'#4ade80'} fontSize={17} fontWeight={700} fontFamily={'monospace'} />
              <Txt text={'   Reclaimed Space : 36,191,232 bytes (-74.46%)'} fill={'#38bdf8'} fontSize={17} fontWeight={700} fontFamily={'monospace'} />
              <Txt text={'   Throughput      : 237 docs/sec across 16 Rayon CPU worker threads'} fill={'#fbbf24'} fontSize={17} fontFamily={'monospace'} />
              <Txt text={'   Total Wall Time : 88 milliseconds'} fill={'#e2e8f0'} fontSize={17} fontFamily={'monospace'} />
            </Rect>
          </Rect>
        </Rect>
      </Rect>

      {/* ========================================================= */}
      {/* SECTION 4.4: INTERFACE 4 - LOCAL REST API (100.0s - 114.7s) */}
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

        {/* 2-Column Split: HTTP Request (Input) vs JSON Receipt (Output) */}
        <Rect width={1672} height={540} layout direction={'row'} gap={24}>
          {/* Left: HTTP POST Request */}
          <Rect
            width={824}
            height={540}
            fill={'#10172a'}
            stroke={'#f59e0b'}
            lineWidth={1.5}
            radius={18}
            padding={24}
            layout
            direction={'column'}
            gap={14}
          >
            <Rect layout direction={'row'} alignItems={'center'} gap={12}>
              <Rect fill={'#f59e0b'} radius={6} paddingLeft={12} paddingRight={12} paddingTop={4} paddingBottom={4}>
                <Txt text={'POST'} fill={'#000000'} fontSize={16} fontWeight={900} fontFamily={'monospace'} />
              </Rect>
              <Txt text={'http://localhost:7823/v1/compress'} fill={'#ffffff'} fontSize={20} fontWeight={700} fontFamily={'monospace'} />
            </Rect>

            <Txt text={'// Multipart / JSON Request Headers:'} fill={'#94a3b8'} fontSize={14} fontFamily={'monospace'} />
            <Rect fill={'#0a0f1d'} radius={10} padding={16}>
              <Txt
                text={'Host: 127.0.0.1:7823\nContent-Type: multipart/form-data; boundary=X-BOUNDARY\nX-PaperPilot-Engine: native-rust\nX-PaperPilot-Threads: 16'}
                fill={'#cbd5e1'}
                fontSize={15}
                fontFamily={'monospace'}
                lineHeight={22}
              />
            </Rect>

            <Txt text={'// Request Body Form Parameters (Input):'} fill={'#94a3b8'} fontSize={14} fontFamily={'monospace'} />
            <Rect fill={'#0a0f1d'} radius={10} padding={16}>
              <Txt
                text={'--X-BOUNDARY\nContent-Disposition: form-data; name="file"; filename="input.pdf"\n[48.6 MB PDF Binary Stream]\n--X-BOUNDARY\nContent-Disposition: form-data; name="quality"\nmedium\n--X-BOUNDARY--'}
                fill={'#38bdf8'}
                fontSize={15}
                fontFamily={'monospace'}
                lineHeight={22}
              />
            </Rect>
          </Rect>

          {/* Right: HTTP 200 JSON Response */}
          <Rect
            width={824}
            height={540}
            fill={'#0a0f1d'}
            stroke={'#10b981'}
            lineWidth={1.5}
            radius={18}
            padding={24}
            layout
            direction={'column'}
            gap={14}
          >
            <Rect layout direction={'row'} alignItems={'center'} gap={12}>
              <Rect fill={'#10b981'} radius={6} paddingLeft={12} paddingRight={12} paddingTop={4} paddingBottom={4}>
                <Txt text={'200 OK'} fill={'#ffffff'} fontSize={16} fontWeight={900} fontFamily={'monospace'} />
              </Rect>
              <Txt text={'JSON Compression Receipt (Output)'} fill={'#4ade80'} fontSize={20} fontWeight={700} />
            </Rect>

            <Rect fill={'#0f172a'} radius={10} padding={18} layout direction={'column'} gap={6}>
              <Txt
                text={'{\n  "success": true,\n  "status": "completed",\n  "input_file": "input.pdf",\n  "original_size_bytes": 50960793,\n  "compressed_size_bytes": 12410880,\n  "bytes_saved": 38549913,\n  "reduction_ratio": 0.7565,\n  "reduction_percentage": "75.65%",\n  "optimizations_applied": [\n    "stream_deduplication",\n    "font_glyph_subsetting",\n    "bicubic_image_downsample"\n  ],\n  "execution_time_ms": 84.1,\n  "engine": "paperpilot-core-rust 1.0"\n}'}
                fill={'#4ade80'}
                fontSize={15}
                fontFamily={'monospace'}
                lineHeight={22}
              />
            </Rect>
          </Rect>
        </Rect>
      </Rect>

      {/* ========================================================= */}
      {/* SECTION 4.5: INTERFACE 5 - WEB / WASM (114.7s - 128.4s) */}
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
            <Txt text={'⚡ Web / WASM: 100% In-Browser WebAssembly Memory'} fill={'#ffffff'} fontSize={28} fontWeight={800} />
          </Rect>
          <Txt text={'usepaperpilot.com • Zero Uploads • 100% Client-Side'} fill={'#22d3ee'} fontSize={18} fontWeight={600} />
        </Rect>

        {/* Browser Mockup */}
        <Rect
          width={1672}
          height={540}
          fill={'#080e1a'}
          stroke={'#1e293b'}
          lineWidth={2}
          radius={18}
          clip
          layout
          direction={'column'}
        >
          {/* Browser Address Bar */}
          <Rect width={1672} height={48} fill={'#131c31'} layout direction={'row'} alignItems={'center'} paddingLeft={20} paddingRight={20} gap={14}>
            <Rect layout direction={'row'} gap={8}>
              <Rect width={12} height={12} radius={6} fill={'#ef4444'} />
              <Rect width={12} height={12} radius={6} fill={'#f59e0b'} />
              <Rect width={12} height={12} radius={6} fill={'#10b981'} />
            </Rect>
            <Rect fill={'#0c1322'} stroke={'#334155'} lineWidth={1} radius={8} width={1100} height={32} layout direction={'row'} alignItems={'center'} paddingLeft={14} gap={8}>
              <Txt text={'🔒'} fontSize={14} />
              <Txt text={'https://usepaperpilot.com/tools/compress'} fill={'#ffffff'} fontSize={15} fontWeight={600} fontFamily={'monospace'} />
            </Rect>
            <Rect fill={'#10b98120'} stroke={'#10b981'} lineWidth={1} radius={6} paddingLeft={10} paddingRight={10} paddingTop={3} paddingBottom={3}>
              <Txt text={'WASM V8 / SpiderMonkey Active'} fill={'#34d399'} fontSize={13} fontWeight={700} />
            </Rect>
          </Rect>

          {/* Web App Canvas */}
          <Rect width={1672} height={492} layout direction={'row'} padding={28} gap={28}>
            {/* Drop Target */}
            <Rect
              width={800}
              height={436}
              fill={'#0f172a'}
              stroke={'#0891b2'}
              lineWidth={2}
              radius={16}
              layout
              direction={'column'}
              alignItems={'center'}
              justifyContent={'center'}
              padding={30}
              gap={16}
            >
              <Rect width={80} height={80} radius={40} fill={'#0891b220'} stroke={'#0891b2'} lineWidth={2} alignItems={'center'} justifyContent={'center'}>
                <Txt text={'☁️⃠'} fontSize={36} />
              </Rect>
              <Txt text={'Drag & Drop PDF Here'} fill={'#ffffff'} fontSize={26} fontWeight={800} />
              <Txt text={'No files are ever uploaded. Processing runs entirely in your local browser sandbox.'} fill={'#94a3b8'} fontSize={17} textAlign={'center'} />
              <Rect fill={'#0891b2'} radius={10} paddingLeft={20} paddingRight={20} paddingTop={10} paddingBottom={10}>
                <Txt text={'Select Local PDF Document'} fill={'#ffffff'} fontSize={16} fontWeight={700} />
              </Rect>
            </Rect>

            {/* In-Browser WASM Execution Details */}
            <Rect
              width={800}
              height={436}
              fill={'#0f172a'}
              stroke={'#10b981'}
              lineWidth={2}
              radius={16}
              layout
              direction={'column'}
              padding={26}
              gap={16}
            >
              <Txt text={'WebAssembly Execution Pipeline'} fill={'#ffffff'} fontSize={24} fontWeight={800} />

              <Rect layout direction={'column'} gap={12}>
                <Rect fill={'#172238'} radius={10} padding={14} layout direction={'row'} alignItems={'center'} justifyContent={'space-between'}>
                  <Txt text={'1. WASM Memory Allocation'} fill={'#94a3b8'} fontSize={16} />
                  <Txt text={'64 MB SharedBuffer'} fill={'#38bdf8'} fontSize={16} fontWeight={700} />
                </Rect>
                <Rect fill={'#172238'} radius={10} padding={14} layout direction={'row'} alignItems={'center'} justifyContent={'space-between'}>
                  <Txt text={'2. Network Traffic Inspection'} fill={'#94a3b8'} fontSize={16} />
                  <Txt text={'0 Bytes Sent (Verified)'} fill={'#4ade80'} fontSize={16} fontWeight={700} />
                </Rect>
                <Rect fill={'#172238'} radius={10} padding={14} layout direction={'row'} alignItems={'center'} justifyContent={'space-between'}>
                  <Txt text={'3. Rust WASM SIMD Optimization'} fill={'#94a3b8'} fontSize={16} />
                  <Txt text={'210ms Execution'} fill={'#fbbf24'} fontSize={16} fontWeight={700} />
                </Rect>
                <Rect fill={'#172238'} radius={10} padding={14} layout direction={'row'} alignItems={'center'} justifyContent={'space-between'}>
                  <Txt text={'4. Final In-Memory Download'} fill={'#94a3b8'} fontSize={16} />
                  <Txt text={'Blob URL Instant Save'} fill={'#34d399'} fontSize={16} fontWeight={700} />
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
      {/* SECTION 5: OUTRO & INSTALLATION (128.4s - End) */}
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
        <Rect
          fill={'#10b981'}
          radius={14}
          paddingLeft={24}
          paddingRight={24}
          paddingTop={8}
          paddingBottom={8}
        >
          <Txt text={'100% OFFLINE • OPEN SOURCE • ZERO SUBSCRIPTIONS'} fill={'#ffffff'} fontSize={20} fontWeight={900} letterSpacing={2} />
        </Rect>

        <Txt
          text={'Claim Complete Document Sovereignty'}
          fill={'#ffffff'}
          fontSize={56}
          fontWeight={900}
        />

        <Txt
          text={'One Native Rust Binary. 100 Tools. 5 Interfaces.'}
          fill={'#38bdf8'}
          fontSize={28}
          fontWeight={700}
        />

        {/* 5-Second Flash Install Command Box */}
        <Rect
          width={1300}
          fill={'#131c31'}
          stroke={'#38bdf8'}
          lineWidth={2}
          radius={16}
          padding={22}
          layout
          direction={'column'}
          alignItems={'center'}
          gap={10}
        >
          <Txt text={'5-SECOND FLASH INSTALL:'} fill={'#94a3b8'} fontSize={16} fontWeight={800} letterSpacing={1.5} />
          <Txt
            text={'curl -fsSL https://usepaperpilot.com/install.sh | bash'}
            fill={'#4ade80'}
            fontSize={32}
            fontWeight={900}
            fontFamily={'monospace'}
          />
        </Rect>

        {/* Links: GitHub & Website */}
        <Rect layout direction={'row'} gap={50} marginTop={10}>
          <Rect
            fill={'#1e293b'}
            stroke={'#64748b'}
            lineWidth={1.5}
            radius={12}
            paddingLeft={24}
            paddingRight={24}
            paddingTop={12}
            paddingBottom={12}
            layout
            direction={'row'}
            alignItems={'center'}
            gap={12}
          >
            <Txt text={'⭐ Star us on GitHub:'} fill={'#fbbf24'} fontSize={20} fontWeight={800} />
            <Txt text={'github.com/KrushnaVardhanReddy/PaperPilot'} fill={'#ffffff'} fontSize={20} fontWeight={700} />
          </Rect>

          <Rect
            fill={'#1e293b'}
            stroke={'#64748b'}
            lineWidth={1.5}
            radius={12}
            paddingLeft={24}
            paddingRight={24}
            paddingTop={12}
            paddingBottom={12}
            layout
            direction={'row'}
            alignItems={'center'}
            gap={12}
          >
            <Txt text={'🌐 Website:'} fill={'#38bdf8'} fontSize={20} fontWeight={800} />
            <Txt text={'https://usepaperpilot.com'} fill={'#ffffff'} fontSize={20} fontWeight={700} />
          </Rect>
        </Rect>
      </Rect>

      {/* ========================================================= */}
      {/* BOTTOM PERSISTENT CAPTIONS BAR */}
      {/* ========================================================= */}
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

  // Section timing landmarks from captions alignment
  // intro: 0.10s
  // problem_solution: 13.48s
  // tool_focus: 45.87s
  // interface_gui: 52.84s
  // interface_agent: 68.97s
  // interface_cli: 86.80s
  // interface_api: 100.01s
  // interface_wasm: 114.69s
  // outro: 128.44s
  // Total: 146.43s

  const captions = captionsData.captions;
  let activeSection = 'intro';
  let currentTime = 0;

  for (let i = 0; i < captions.length; i++) {
    const item = captions[i];
    const waitBefore = Math.max(0, item.start - currentTime);
    if (waitBefore > 0) {
      yield* waitFor(waitBefore);
      currentTime += waitBefore;
    }

    // Update caption text
    captionTextRef().text(item.text);

    // Dynamic Section Stage Transitions based on voiceover progression
    if (item.start >= 13.4 && activeSection === 'intro') {
      activeSection = 'problem_solution';
      stageBadgeTextRef().text('PART 2 OF 5');
      stageTitleRef().text('The Problem & The High-Speed Engine Solution');
      yield* all(
        introCardRef().opacity(0, 0.4),
        problemCardRef().opacity(1, 0.4),
      );
      currentTime += 0.4;
    } else if (item.start >= 45.8 && activeSection === 'problem_solution') {
      activeSection = 'tool_focus';
      stageBadgeTextRef().text('PART 3 OF 5');
      stageTitleRef().text('Tool 01: Instant PDF Compression');
      yield* all(
        problemCardRef().opacity(0, 0.4),
        toolCardRef().opacity(1, 0.4),
      );
      currentTime += 0.4;
    } else if (item.start >= 52.8 && activeSection === 'tool_focus') {
      activeSection = 'interface_gui';
      stageBadgeTextRef().text('PART 4 OF 5 (1/5)');
      stageTitleRef().text('Interface 1: Desktop GUI');
      yield* all(
        toolCardRef().opacity(0, 0.4),
        guiCardRef().opacity(1, 0.4),
      );
      currentTime += 0.4;
    } else if (item.start >= 68.9 && activeSection === 'interface_gui') {
      activeSection = 'interface_agent';
      stageBadgeTextRef().text('PART 4 OF 5 (2/5)');
      stageTitleRef().text('Interface 2: AI Agent (Claude / MCP)');
      yield* all(
        guiCardRef().opacity(0, 0.4),
        agentCardRef().opacity(1, 0.4),
      );
      currentTime += 0.4;
    } else if (item.start >= 86.7 && activeSection === 'interface_agent') {
      activeSection = 'interface_cli';
      stageBadgeTextRef().text('PART 4 OF 5 (3/5)');
      stageTitleRef().text('Interface 3: Terminal CLI');
      yield* all(
        agentCardRef().opacity(0, 0.4),
        cliCardRef().opacity(1, 0.4),
      );
      currentTime += 0.4;
    } else if (item.start >= 99.9 && activeSection === 'interface_cli') {
      activeSection = 'interface_api';
      stageBadgeTextRef().text('PART 4 OF 5 (4/5)');
      stageTitleRef().text('Interface 4: Local REST API');
      yield* all(
        cliCardRef().opacity(0, 0.4),
        apiCardRef().opacity(1, 0.4),
      );
      currentTime += 0.4;
    } else if (item.start >= 114.6 && activeSection === 'interface_api') {
      activeSection = 'interface_wasm';
      stageBadgeTextRef().text('PART 4 OF 5 (5/5)');
      stageTitleRef().text('Interface 5: Web / WASM');
      yield* all(
        apiCardRef().opacity(0, 0.4),
        wasmCardRef().opacity(1, 0.4),
      );
      currentTime += 0.4;
    } else if (item.start >= 128.3 && activeSection === 'interface_wasm') {
      activeSection = 'outro';
      stageBadgeTextRef().text('PART 5 OF 5');
      stageTitleRef().text('Outro & Installation');
      yield* all(
        wasmCardRef().opacity(0, 0.4),
        outroCardRef().opacity(1, 0.4),
      );
      currentTime += 0.4;
    }

    const captionDuration = Math.max(0.1, item.end - currentTime);
    yield* waitFor(captionDuration);
    currentTime += captionDuration;
  }

  // Hold final frame for 2.0s
  yield* waitFor(2.0);
});

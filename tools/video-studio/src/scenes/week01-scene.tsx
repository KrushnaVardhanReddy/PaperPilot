import { makeScene2D } from '@revideo/2d';
import { createRef, all, waitFor, easeInOutCubic, easeOutBack } from '@revideo/core';
import { Audio, Video, Rect, Txt, Layout, Img } from '@revideo/2d';

// Captions data loaded directly for word-level sync
import { captionsData } from './captions';

export default makeScene2D('week01-compression-scene', function* (view) {
  // Primary container and layout references
  const bgRef = createRef<Rect>();
  const hookBadgeRef = createRef<Rect>();
  const hookTitleRef = createRef<Txt>();
  const hookSubRef = createRef<Txt>();
  const hookCardRef = createRef<Rect>();

  const videoWindowRef = createRef<Rect>();
  const screenVideoRef = createRef<Video>();

  const captionCardRef = createRef<Rect>();
  const captionTextRef = createRef<Txt>();

  const statsCardRef = createRef<Rect>();
  const statValRef = createRef<Txt>();
  const statSubRef = createRef<Txt>();

  const outroCardRef = createRef<Rect>();
  const outroTitleRef = createRef<Txt>();
  const outroCommandRef = createRef<Txt>();
  const outroBadgeRef = createRef<Rect>();

  // Add all visual elements to view
  view.add(
    <Rect
      ref={bgRef}
      width={1080}
      height={1920}
      fill={'#080c14'}
    >
      {/* Background ambient gradient glow blobs */}
      <Rect
        width={700}
        height={700}
        radius={350}
        x={-200}
        y={-400}
        fill={'#1e3a8a'}
        opacity={0.25}
      />
      <Rect
        width={600}
        height={600}
        radius={300}
        x={250}
        y={500}
        fill={'#065f46'}
        opacity={0.20}
      />

      {/* Voiceover Audio Component */}
      <Audio
        src={'../assets/week01_voiceover.mp3'}
        play={true}
      />

      {/* Top Header & Intro Hook Card */}
      <Rect
        ref={hookCardRef}
        y={-680}
        width={960}
        height={260}
        fill={'#0f172a'}
        stroke={'#1e293b'}
        lineWidth={3}
        radius={28}
        layout
        direction={'column'}
        alignItems={'center'}
        justifyContent={'center'}
        padding={30}
        gap={14}
        shadowColor={'#000000'}
        shadowBlur={30}
        shadowOffsetY={10}
      >
        <Rect
          ref={hookBadgeRef}
          fill={'#2563eb'}
          radius={20}
          paddingLeft={24}
          paddingRight={24}
          paddingTop={8}
          paddingBottom={8}
        >
          <Txt
            text={'WEEK 1 • MULTI-INTERFACE SHOWCASE'}
            fill={'#ffffff'}
            fontSize={24}
            fontWeight={800}
            letterSpacing={2}
          />
        </Rect>

        <Txt
          ref={hookTitleRef}
          text={'Instant PDF Compression'}
          fill={'#ffffff'}
          fontSize={52}
          fontWeight={900}
        />

        <Txt
          ref={hookSubRef}
          text={'Native Rust • 85% Size Reduction • Sub-Second Execution'}
          fill={'#94a3b8'}
          fontSize={26}
          fontWeight={600}
        />
      </Rect>

      {/* Center Screen Recording Window (Mock desktop chrome) */}
      <Rect
        ref={videoWindowRef}
        y={-40}
        width={980}
        height={660}
        fill={'#0b0f19'}
        stroke={'#334155'}
        lineWidth={3}
        radius={24}
        clip
        shadowColor={'#000000'}
        shadowBlur={40}
        shadowOffsetY={15}
      >
        {/* Window Chrome Header Bar */}
        <Rect
          width={980}
          height={52}
          y={-304}
          fill={'#1e293b'}
          layout
          direction={'row'}
          alignItems={'center'}
          paddingLeft={24}
          gap={12}
        >
          <Rect width={16} height={16} radius={8} fill={'#ef4444'} />
          <Rect width={16} height={16} radius={8} fill={'#f59e0b'} />
          <Rect width={16} height={16} radius={8} fill={'#10b981'} />
          <Txt
            text={'PaperPilot Multi-Interface Engine (GUI • MCP • CLI • REST API)'}
            fill={'#94a3b8'}
            fontSize={20}
            fontWeight={600}
            marginLeft={20}
          />
        </Rect>

        {/* Live Screen Action Video */}
        <Video
          ref={screenVideoRef}
          src={'../assets/screen_recording.mp4'}
          y={26}
          width={980}
          play={true}
        />
      </Rect>

      {/* Real-time Dynamic Metrics Overlay Badge */}
      <Rect
        ref={statsCardRef}
        y={360}
        width={940}
        height={130}
        fill={'#0f172a'}
        stroke={'#3b82f6'}
        lineWidth={2}
        radius={20}
        layout
        direction={'row'}
        alignItems={'center'}
        justifyContent={'space-around'}
        padding={20}
        opacity={0}
      >
        <Rect layout direction={'column'} alignItems={'center'}>
          <Txt text={'237 docs/sec'} fill={'#38bdf8'} fontSize={36} fontWeight={800} />
          <Txt text={'Throughput'} fill={'#94a3b8'} fontSize={20} fontWeight={600} />
        </Rect>
        <Rect width={2} height={60} fill={'#334155'} />
        <Rect layout direction={'column'} alignItems={'center'}>
          <Txt text={'-74.4% (-48.2 GB)'} fill={'#4ade80'} fontSize={36} fontWeight={800} />
          <Txt text={'Storage Reclaimed'} fill={'#94a3b8'} fontSize={20} fontWeight={600} />
        </Rect>
        <Rect width={2} height={60} fill={'#334155'} />
        <Rect layout direction={'column'} alignItems={'center'}>
          <Txt text={'$0.00'} fill={'#fbbf24'} fontSize={36} fontWeight={800} />
          <Txt text={'Cloud API Fees'} fill={'#94a3b8'} fontSize={20} fontWeight={600} />
        </Rect>
      </Rect>

      {/* Dynamic Animated Captions Bar */}
      <Rect
        ref={captionCardRef}
        y={560}
        width={940}
        height={160}
        fill={'#0f172ae6'}
        stroke={'#6366f1'}
        lineWidth={3}
        radius={24}
        layout
        direction={'column'}
        alignItems={'center'}
        justifyContent={'center'}
        paddingLeft={30}
        paddingRight={30}
        shadowColor={'#6366f140'}
        shadowBlur={25}
      >
        <Txt
          ref={captionTextRef}
          text={'Tired of PDF tools...'}
          fill={'#ffffff'}
          fontSize={38}
          fontWeight={800}
          textWrap={'balance'}
        />
      </Rect>

      {/* Outro Call To Action Modal Card (Revealed near end) */}
      <Rect
        ref={outroCardRef}
        y={150}
        width={960}
        height={520}
        fill={'#090d16fa'}
        stroke={'#38bdf8'}
        lineWidth={4}
        radius={32}
        layout
        direction={'column'}
        alignItems={'center'}
        justifyContent={'center'}
        padding={40}
        gap={20}
        opacity={0}
        scale={0.8}
        shadowColor={'#38bdf860'}
        shadowBlur={40}
      >
        <Rect
          ref={outroBadgeRef}
          fill={'#10b981'}
          radius={20}
          paddingLeft={28}
          paddingRight={28}
          paddingTop={8}
          paddingBottom={8}
        >
          <Txt
            text={'100% OFFLINE • OPEN SOURCE'}
            fill={'#ffffff'}
            fontSize={24}
            fontWeight={900}
            letterSpacing={2}
          />
        </Rect>

        <Txt
          ref={outroTitleRef}
          text={'Install PaperPilot in Seconds'}
          fill={'#ffffff'}
          fontSize={48}
          fontWeight={900}
        />

        <Rect
          width={880}
          fill={'#1e293b'}
          stroke={'#475569'}
          lineWidth={2}
          radius={16}
          padding={20}
          layout
          direction={'column'}
          alignItems={'center'}
        >
          <Txt
            ref={outroCommandRef}
            text={'curl -fsSL https://paperpilot.dev/install.sh | bash'}
            fill={'#38bdf8'}
            fontSize={26}
            fontWeight={700}
            fontFamily={'monospace'}
          />
        </Rect>

        <Txt
          text={'github.com/KrushnaVardhanReddy/PaperPilot'}
          fill={'#94a3b8'}
          fontSize={24}
          fontWeight={600}
        />
      </Rect>
    </Rect>
  );

  // Initial Entrance Animation
  yield* all(
    hookCardRef().scale(1, 0.6, easeOutBack),
    videoWindowRef().scale(1, 0.6, easeOutBack),
  );

  // Animate through all captions synchronized with word/phrase timestamps
  const captions = captionsData.captions;
  let currentTime = 0.6; // We spent 0.6s on intro entrance

  for (let i = 0; i < captions.length; i++) {
    const item = captions[i];
    const waitBefore = Math.max(0, item.start - currentTime);
    if (waitBefore > 0) {
      yield* waitFor(waitBefore);
      currentTime += waitBefore;
    }

    // Update caption text
    captionTextRef().text(item.text);

    // Zoom and pan video based on story progression
    if (item.start >= 10.0 && statsCardRef().opacity() === 0) {
      // Reveal metrics card when discussing batch compression scale
      yield* all(
        statsCardRef().opacity(1, 0.5),
        videoWindowRef().scale(1.05, 0.5, easeInOutCubic),
      );
      currentTime += 0.5;
    }

    // Outro transition at 33.0s
    if (item.start >= 33.0 && outroCardRef().opacity() === 0) {
      yield* all(
        outroCardRef().opacity(1, 0.6, easeOutBack),
        outroCardRef().scale(1, 0.6, easeOutBack),
        videoWindowRef().opacity(0.3, 0.6),
        captionCardRef().opacity(0.8, 0.6),
      );
      currentTime += 0.6;
    }

    const captionDuration = Math.max(0.1, item.end - currentTime);
    yield* waitFor(captionDuration);
    currentTime += captionDuration;
  }

  // Hold final outro frame for 1.5 seconds
  yield* waitFor(1.5);
});

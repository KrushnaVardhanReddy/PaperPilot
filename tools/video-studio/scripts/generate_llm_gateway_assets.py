import asyncio
import json
import os
import edge_tts

VOICE = "en-US-ChristopherNeural"
RATE = "+6%"

TEXT = (
    "Enterprise LLM pipelines often choke on document parsing before generation even begins. "
    "PaperPilot changes that. With our native Rust ingestion gateway, documents like this quarterly "
    "financial report are parsed in just 3.4 milliseconds, taking under 50 megabytes of memory. "
    "Clean tabular Markdown feeds directly into high-throughput models like Llama 3.3 70B, instantly delivering "
    "accurate financial intelligence with zero cloud egress. Blazing speed, minimal footprint. "
    "Get started at usepaperpilot.com."
)

AUDIO_OUT = "tools/video-studio/public/audio/llm_gateway_showcase.mp3"
CAPTIONS_OUT = "tools/video-studio/public/captions/llm_gateway_showcase.json"
SCENE_CAPTIONS_TS = "tools/video-studio/src/scenes/captions_llm_gateway.ts"

os.makedirs(os.path.dirname(AUDIO_OUT), exist_ok=True)
os.makedirs(os.path.dirname(CAPTIONS_OUT), exist_ok=True)
os.makedirs(os.path.dirname(SCENE_CAPTIONS_TS), exist_ok=True)

async def main():
    comm = edge_tts.Communicate(TEXT, VOICE, rate=RATE, boundary="WordBoundary")
    words = []
    
    with open(AUDIO_OUT, "wb") as f:
        async for chunk in comm.stream():
            if chunk["type"] == "audio":
                f.write(chunk["data"])
            elif chunk["type"] == "WordBoundary":
                start_sec = chunk["offset"] / 10_000_000.0
                dur_sec = chunk["duration"] / 10_000_000.0
                words.append({
                    "word": chunk["text"],
                    "start": round(start_sec, 3),
                    "end": round(start_sec + dur_sec, 3),
                    "duration": round(dur_sec, 3)
                })
                
    with open(CAPTIONS_OUT, "w") as f:
        json.dump({"text": TEXT, "words": words}, f, indent=2)

    # Generate TypeScript file for Revideo scene import
    with open(SCENE_CAPTIONS_TS, "w") as f:
        f.write("export interface WordCaption {\n  word: string;\n  start: number;\n  end: number;\n  duration: number;\n}\n\n")
        f.write(f"export const LLM_GATEWAY_WORDS: WordCaption[] = {json.dumps(words, indent=2)};\n")
        f.write(f"export const LLM_GATEWAY_TOTAL_DURATION: number = {words[-1]['end'] if words else 0.0};\n")

    print(f"Generated audio: {AUDIO_OUT}")
    print(f"Generated captions: {CAPTIONS_OUT} ({len(words)} words)")
    print(f"Total duration: {words[-1]['end'] if words else 0}s")

if __name__ == "__main__":
    asyncio.run(main())

from typing import Optional, List
import subprocess
import httpx
from .models import MergeRequest, CompressRequest, SplitRequest, PaperPilotResponse

class PaperPilot:
    def __init__(self, endpoint_url: Optional[str] = None, api_key: Optional[str] = None):
        self.endpoint_url = endpoint_url
        self.api_key = api_key
        self._is_local = endpoint_url is None

    @property
    def headers(self):
        h = {"Content-Type": "application/json"}
        if self.api_key:
            h["Authorization"] = f"Bearer {self.api_key}"
        return h

    def _exec_local(self, args: List[str]) -> str:
        cmd = ["paperpilot-cli"] + args
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return result.stdout

    async def _exec_local_async(self, args: List[str]) -> str:
        import asyncio
        cmd = ["paperpilot-cli"] + args
        process = await asyncio.create_subprocess_exec(
            *cmd,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE
        )
        stdout, stderr = await process.communicate()
        if process.returncode != 0:
            raise subprocess.CalledProcessError(process.returncode, cmd, output=stdout, stderr=stderr)
        return stdout.decode()

    def _exec_remote(self, endpoint: str, payload: dict) -> PaperPilotResponse:
        if not self.endpoint_url:
            raise ValueError("endpoint_url must be set for remote execution")
        url = f"{self.endpoint_url.rstrip('/')}{endpoint}"
        with httpx.Client() as client:
            response = client.post(url, headers=self.headers, json=payload)
            response.raise_for_status()
            data = response.json()
            return PaperPilotResponse(success=data.get("success", True), stdout=data.get("stdout"))

    async def _exec_remote_async(self, endpoint: str, payload: dict) -> PaperPilotResponse:
        if not self.endpoint_url:
            raise ValueError("endpoint_url must be set for remote execution")
        url = f"{self.endpoint_url.rstrip('/')}{endpoint}"
        async with httpx.AsyncClient() as client:
            response = await client.post(url, headers=self.headers, json=payload)
            response.raise_for_status()
            data = response.json()
            return PaperPilotResponse(success=data.get("success", True), stdout=data.get("stdout"))

    def merge(self, inputs: List[str], output: str) -> PaperPilotResponse:
        if self._is_local:
            args = ["merge"]
            for i in inputs:
                args.extend(["--input", i])
            args.extend(["--output", output])
            return PaperPilotResponse(success=True, stdout=self._exec_local(args))
        req = MergeRequest(inputs=inputs, output=output)
        return self._exec_remote("/api/v1/pdf/merge", req.model_dump())

    async def merge_async(self, inputs: List[str], output: str) -> PaperPilotResponse:
        if self._is_local:
            args = ["merge"]
            for i in inputs:
                args.extend(["--input", i])
            args.extend(["--output", output])
            return PaperPilotResponse(success=True, stdout=await self._exec_local_async(args))
        req = MergeRequest(inputs=inputs, output=output)
        return await self._exec_remote_async("/api/v1/pdf/merge", req.model_dump())

    def compress(self, input: str, quality: str, output: str) -> PaperPilotResponse:
        if self._is_local:
            args = ["compress", "--input", input, "--quality", quality, "--output", output]
            return PaperPilotResponse(success=True, stdout=self._exec_local(args))
        req = CompressRequest(input=input, quality=quality, output=output)
        return self._exec_remote("/api/v1/pdf/compress", req.model_dump(exclude_none=True))

    async def compress_async(self, input: str, quality: str, output: str) -> PaperPilotResponse:
        if self._is_local:
            args = ["compress", "--input", input, "--quality", quality, "--output", output]
            return PaperPilotResponse(success=True, stdout=await self._exec_local_async(args))
        req = CompressRequest(input=input, quality=quality, output=output)
        return await self._exec_remote_async("/api/v1/pdf/compress", req.model_dump(exclude_none=True))

    def bates_stamp(self, input: str, prefix: str, start: int, output: str) -> PaperPilotResponse:
        if self._is_local:
            args = ["bates", "--input", input, "--prefix", prefix, "--start", str(start), "--output", output]
            return PaperPilotResponse(success=True, stdout=self._exec_local(args))
        raise NotImplementedError("bates_stamp is local-only")

    def split(self, input: str, pages: str, output: str) -> PaperPilotResponse:
        if self._is_local:
            args = ["split", "--input", input, "--pages", pages, "--output", output]
            return PaperPilotResponse(success=True, stdout=self._exec_local(args))
        req = SplitRequest(input=input, pages=pages, output=output)
        return self._exec_remote("/api/v1/pdf/split", req.model_dump(exclude_none=True))

    async def split_async(self, input: str, pages: str, output: str) -> PaperPilotResponse:
        if self._is_local:
            args = ["split", "--input", input, "--pages", pages, "--output", output]
            return PaperPilotResponse(success=True, stdout=await self._exec_local_async(args))
        req = SplitRequest(input=input, pages=pages, output=output)
        return await self._exec_remote_async("/api/v1/pdf/split", req.model_dump(exclude_none=True))

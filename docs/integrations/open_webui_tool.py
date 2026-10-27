"""
title: PaperPilot Shared PDF Extraction
description: Extract text and tables from PDFs in an administrator-selected folder.
version: 0.1.0
requirements: httpx
"""

import asyncio
from pathlib import Path

import httpx
from pydantic import BaseModel, Field


class Tools:
    class Valves(BaseModel):
        PAPERPILOT_URL: str = Field(
            default="http://127.0.0.1:7823", description="Trusted PaperPilot Gateway base URL"
        )
        PDF_ROOT: str = Field(
            default="/data/pdfs", description="Absolute shared PDF folder in the Open-WebUI host"
        )

    def __init__(self):
        self.valves = self.Valves()

    async def extract_pdf(self, filename: str) -> str:
        """Extract PDF text and tables from a file relative to the shared PDF folder.

        :param filename: Relative PDF filename, for example invoices/january.pdf.
        :return: Page-ordered text with detected Markdown tables.
        """
        root = Path(self.valves.PDF_ROOT).resolve(strict=True)

        def read_pdf() -> bytes:
            relative = Path(filename)
            if relative.is_absolute() or ".." in relative.parts:
                raise ValueError("Use a relative filename inside the shared PDF folder")
            path = (root / relative).resolve(strict=True)
            if not path.is_relative_to(root) or path.suffix.lower() != ".pdf":
                raise ValueError("Only PDFs inside the shared PDF folder are allowed")
            limit = 100 * 1024 * 1024
            with path.open("rb") as source:
                data = source.read(limit + 1)
            if len(data) > limit:
                raise ValueError("PDF exceeds 100 MiB")
            if not data.startswith(b"%PDF-"):
                raise ValueError("File does not contain PDF bytes")
            return data

        data = await asyncio.to_thread(read_pdf)
        async with httpx.AsyncClient(timeout=120.0, follow_redirects=False) as client:
            response = await client.put(
                self.valves.PAPERPILOT_URL.rstrip("/") + "/tika/text",
                content=data,
                headers={"Content-Type": "application/pdf"},
            )
            response.raise_for_status()
            return response.json()["X-TIKA:content"]

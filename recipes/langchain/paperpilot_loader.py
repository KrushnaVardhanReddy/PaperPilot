from typing import Iterator, List, Optional
import requests
from langchain_core.document_loaders import BaseLoader
from langchain_core.documents import Document

class PaperPilotLoader(BaseLoader):
    """Load and semantically chunk PDFs using local PaperPilot Gateway."""

    def __init__(
        self,
        file_path: str,
        api_base: str = "http://127.0.0.1:7823",
        strip_boilerplate: bool = True
    ):
        self.file_path = file_path
        self.api_base = api_base
        self.strip_boilerplate = strip_boilerplate

    def lazy_load(self) -> Iterator[Document]:
        url = f"{self.api_base}/api/v1/pdf/tools/pdf_rag_chunks"
        payload = {
            "input": self.file_path,
            "chunk_size": 512
        }
        res = requests.post(url, json=payload, timeout=30)
        res.raise_for_status()
        data = res.json()
        chunks = data.get("chunks", []) if isinstance(data, dict) else data

        for chunk in chunks:
            text = chunk.get("text", "")
            metadata = {
                "source": self.file_path,
                "page": chunk.get("page", 1),
                "tokens": chunk.get("tokens", 0)
            }
            yield Document(page_content=text, metadata=metadata)

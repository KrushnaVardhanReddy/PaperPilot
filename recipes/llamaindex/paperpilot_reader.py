from typing import List
import requests
from llama_index.core.readers.base import BaseReader
from llama_index.core.schema import Document

class PaperPilotReader(BaseReader):
    """Load and semantically chunk PDFs using local PaperPilot Gateway."""

    def __init__(
        self,
        api_base: str = "http://127.0.0.1:7823",
        strip_boilerplate: bool = True
    ):
        self.api_base = api_base
        self.strip_boilerplate = strip_boilerplate

    def load_data(self, file_path: str, extra_info: dict = None) -> List[Document]:
        url = f"{self.api_base}/api/v1/pdf/tools/pdf_rag_chunks"
        payload = {
            "input": file_path,
            "chunk_size": 512
        }
        res = requests.post(url, json=payload, timeout=30)
        res.raise_for_status()
        data = res.json()
        chunks = data.get("chunks", []) if isinstance(data, dict) else data

        documents = []
        for chunk in chunks:
            text = chunk.get("text", "")
            metadata = {
                "source": file_path,
                "page": chunk.get("page", 1),
                "tokens": chunk.get("tokens", 0)
            }
            if extra_info:
                metadata.update(extra_info)
            documents.append(Document(text=text, metadata=metadata))

        return documents

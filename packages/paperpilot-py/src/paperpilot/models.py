from pydantic import BaseModel
from typing import List, Optional

class MergeRequest(BaseModel):
    inputs: List[str]
    output: str

class CompressRequest(BaseModel):
    input: str
    output: str
    quality: Optional[str] = None

class SplitRequest(BaseModel):
    input: str
    pages: Optional[str] = None
    output: Optional[str] = None

class PaperPilotResponse(BaseModel):
    success: bool
    stdout: Optional[str] = None

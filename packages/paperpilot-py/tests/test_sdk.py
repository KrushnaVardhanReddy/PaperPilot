import pytest
from unittest.mock import patch, MagicMock
from paperpilot import PaperPilot

def test_sync_local_merge():
    client = PaperPilot()
    with patch('subprocess.run') as mock_run:
        mock_run.return_value = MagicMock(stdout="success")
        client.merge(["a.pdf", "b.pdf"], "out.pdf")
        mock_run.assert_called_once_with(["paperpilot-cli", "merge", "--input", "a.pdf", "--input", "b.pdf", "--output", "out.pdf"], capture_output=True, text=True, check=True)

def test_sync_local_compress():
    client = PaperPilot()
    with patch('subprocess.run') as mock_run:
        mock_run.return_value = MagicMock(stdout="success")
        client.compress("in.pdf", "medium", "out.pdf")
        mock_run.assert_called_once_with(["paperpilot-cli", "compress", "--input", "in.pdf", "--quality", "medium", "--output", "out.pdf"], capture_output=True, text=True, check=True)

def test_sync_remote_merge():
    client = PaperPilot(endpoint_url="http://localhost:7823")
    with patch('httpx.Client.post') as mock_post:
        mock_post.return_value = MagicMock(json=lambda: {"success": True})
        client.merge(["a.pdf", "b.pdf"], "out.pdf")
        mock_post.assert_called_once_with("http://localhost:7823/api/v1/pdf/merge", headers={"Content-Type": "application/json"}, json={"inputs": ["a.pdf", "b.pdf"], "output": "out.pdf"})

def test_sync_remote_compress():
    client = PaperPilot(endpoint_url="http://localhost:7823")
    with patch('httpx.Client.post') as mock_post:
        mock_post.return_value = MagicMock(json=lambda: {"success": True})
        client.compress("in.pdf", "medium", "out.pdf")
        mock_post.assert_called_once_with("http://localhost:7823/api/v1/pdf/compress", headers={"Content-Type": "application/json"}, json={"input": "in.pdf", "output": "out.pdf", "quality": "medium"})

def test_sync_local_bates_stamp():
    client = PaperPilot()
    with patch('subprocess.run') as mock_run:
        mock_run.return_value = MagicMock(stdout="success")
        client.bates_stamp("in.pdf", "DOC-", 1, "out.pdf")
        mock_run.assert_called_once_with(["paperpilot-cli", "bates", "--input", "in.pdf", "--prefix", "DOC-", "--start", "1", "--output", "out.pdf"], capture_output=True, text=True, check=True)

def test_sync_local_split():
    client = PaperPilot()
    with patch('subprocess.run') as mock_run:
        mock_run.return_value = MagicMock(stdout="success")
        client.split("in.pdf", "1-3", "out_dir")
        mock_run.assert_called_once_with(["paperpilot-cli", "split", "--input", "in.pdf", "--pages", "1-3", "--output", "out_dir"], capture_output=True, text=True, check=True)

def test_sync_remote_split():
    client = PaperPilot(endpoint_url="http://localhost:7823")
    with patch('httpx.Client.post') as mock_post:
        mock_post.return_value = MagicMock(json=lambda: {"success": True})
        client.split("in.pdf", "1-3", "out_dir")
        mock_post.assert_called_once_with("http://localhost:7823/api/v1/pdf/split", headers={"Content-Type": "application/json"}, json={"input": "in.pdf", "pages": "1-3", "output": "out_dir"})

def test_sync_remote_bates_stamp_raises():
    client = PaperPilot(endpoint_url="http://localhost:7823")
    with pytest.raises(NotImplementedError, match="bates_stamp is local-only"):
        client.bates_stamp("in.pdf", "DOC-", 1, "out.pdf")

@pytest.mark.asyncio
async def test_async_remote_merge():
    client = PaperPilot(endpoint_url="http://localhost:7823")
    with patch('httpx.AsyncClient.post') as mock_post:
        mock_post.return_value = MagicMock(json=lambda: {"success": True})
        await client.merge_async(["a.pdf", "b.pdf"], "out.pdf")
        mock_post.assert_called_once_with("http://localhost:7823/api/v1/pdf/merge", headers={"Content-Type": "application/json"}, json={"inputs": ["a.pdf", "b.pdf"], "output": "out.pdf"})

@pytest.mark.asyncio
async def test_async_remote_compress():
    client = PaperPilot(endpoint_url="http://localhost:7823")
    with patch('httpx.AsyncClient.post') as mock_post:
        mock_post.return_value = MagicMock(json=lambda: {"success": True})
        await client.compress_async("in.pdf", "medium", "out.pdf")
        mock_post.assert_called_once_with("http://localhost:7823/api/v1/pdf/compress", headers={"Content-Type": "application/json"}, json={"input": "in.pdf", "output": "out.pdf", "quality": "medium"})

@pytest.mark.asyncio
async def test_async_local_merge():
    client = PaperPilot()
    with patch('asyncio.create_subprocess_exec') as mock_exec:
        mock_proc = MagicMock()
        async def mock_communicate():
            return (b"success", b"")
        mock_proc.communicate = mock_communicate
        mock_proc.returncode = 0
        mock_exec.return_value = mock_proc

        await client.merge_async(["a.pdf", "b.pdf"], "out.pdf")

        import asyncio
        mock_exec.assert_called_once_with("paperpilot-cli", "merge", "--input", "a.pdf", "--input", "b.pdf", "--output", "out.pdf", stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE)

@pytest.mark.asyncio
async def test_async_local_compress():
    client = PaperPilot()
    with patch('asyncio.create_subprocess_exec') as mock_exec:
        mock_proc = MagicMock()
        async def mock_communicate():
            return (b"success", b"")
        mock_proc.communicate = mock_communicate
        mock_proc.returncode = 0
        mock_exec.return_value = mock_proc

        await client.compress_async("in.pdf", "medium", "out.pdf")

        import asyncio
        mock_exec.assert_called_once_with("paperpilot-cli", "compress", "--input", "in.pdf", "--quality", "medium", "--output", "out.pdf", stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE)

@pytest.mark.asyncio
async def test_async_remote_split():
    client = PaperPilot(endpoint_url="http://localhost:7823")
    with patch('httpx.AsyncClient.post') as mock_post:
        mock_post.return_value = MagicMock(json=lambda: {"success": True})
        await client.split_async("in.pdf", "1-3", "out_dir")
        mock_post.assert_called_once_with("http://localhost:7823/api/v1/pdf/split", headers={"Content-Type": "application/json"}, json={"input": "in.pdf", "pages": "1-3", "output": "out_dir"})

@pytest.mark.asyncio
async def test_async_local_split():
    client = PaperPilot()
    with patch('asyncio.create_subprocess_exec') as mock_exec:
        mock_proc = MagicMock()
        async def mock_communicate():
            return (b"success", b"")
        mock_proc.communicate = mock_communicate
        mock_proc.returncode = 0
        mock_exec.return_value = mock_proc

        await client.split_async("in.pdf", "1-3", "out_dir")

        import asyncio
        mock_exec.assert_called_once_with("paperpilot-cli", "split", "--input", "in.pdf", "--pages", "1-3", "--output", "out_dir", stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE)

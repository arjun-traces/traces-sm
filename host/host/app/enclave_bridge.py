import httpx
from app.config import settings

class EnclaveClient:
    def __init__(self):
        self.base_url = settings.ENCLAVE_URL
        self.verify = settings.ENCLAVE_TLS_VERIFY
        self.client = httpx.AsyncClient(verify=self.verify, base_url=self.base_url, timeout=10.0)

    async def get(self, path: str, headers: dict = None):
        return await self.client.get(path, headers=headers)
        
    async def post(self, path: str, json: dict = None, headers: dict = None):
        return await self.client.post(path, json=json, headers=headers)

    async def put(self, path: str, json: dict = None, headers: dict = None):
        return await self.client.put(path, json=json, headers=headers)
        
    async def delete(self, path: str, headers: dict = None):
        return await self.client.delete(path, headers=headers)

    async def close(self):
        await self.client.aclose()

enclave_client = EnclaveClient()

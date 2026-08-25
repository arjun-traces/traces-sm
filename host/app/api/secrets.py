from fastapi import APIRouter, Depends, Request, HTTPException
from app.auth import get_current_user
from app.enclave_bridge import enclave_client
from pydantic import BaseModel

router = APIRouter()

class SecretCreate(BaseModel):
    name: str
    value: str
    secret_type: str = "generic"
    ttl: int = 0
    tags: dict = {}

@router.post("")
async def create_secret(secret: SecretCreate, req: Request, user: dict = Depends(get_current_user)):
    # Proxy to enclave
    response = await enclave_client.post("/v1/secrets", json=secret.model_dump())
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

@router.get("/{name}")
async def get_secret(name: str, req: Request, user: dict = Depends(get_current_user)):
    response = await enclave_client.get(f"/v1/secrets/{name}")
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

@router.put("/{name}")
async def update_secret(name: str, value: dict, req: Request, user: dict = Depends(get_current_user)):
    response = await enclave_client.put(f"/v1/secrets/{name}", json=value)
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

@router.delete("/{name}")
async def delete_secret(name: str, req: Request, user: dict = Depends(get_current_user)):
    response = await enclave_client.delete(f"/v1/secrets/{name}")
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

@router.get("")
async def list_secrets(req: Request, user: dict = Depends(get_current_user)):
    response = await enclave_client.get("/v1/secrets")
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

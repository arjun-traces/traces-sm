from fastapi import APIRouter, Depends, HTTPException, Request
from app.auth import get_current_user
from app.enclave_bridge import enclave_client
from pydantic import BaseModel

router = APIRouter()

class KeyGenerate(BaseModel):
    name: str
    algorithm: str
    tags: dict = {}

@router.post("/generate")
async def generate_key(key: KeyGenerate, user: dict = Depends(get_current_user)):
    response = await enclave_client.post("/v1/keys/generate", json=key.model_dump())
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

@router.get("/{name}/public")
async def get_public_key(name: str, format: str = "pem", user: dict = Depends(get_current_user)):
    response = await enclave_client.get(f"/v1/keys/{name}/public?format={format}")
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

@router.post("/{name}/sign")
async def sign_message(name: str, payload: dict, user: dict = Depends(get_current_user)):
    response = await enclave_client.post(f"/v1/keys/{name}/sign", json=payload)
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

@router.post("/{name}/verify")
async def verify_signature(name: str, payload: dict, user: dict = Depends(get_current_user)):
    response = await enclave_client.post(f"/v1/keys/{name}/verify", json=payload)
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

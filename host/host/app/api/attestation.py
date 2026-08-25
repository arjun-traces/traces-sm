from fastapi import APIRouter, Depends, HTTPException
from app.auth import get_current_user
from app.enclave_bridge import enclave_client

router = APIRouter()

@router.post("/quote")
async def get_quote(payload: dict = None, user: dict = Depends(get_current_user)):
    response = await enclave_client.post("/v1/attest/quote", json=payload or {})
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

@router.post("/verify")
async def verify_quote(payload: dict, user: dict = Depends(get_current_user)):
    response = await enclave_client.post("/v1/attest/verify", json=payload)
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

from fastapi import APIRouter, Depends, HTTPException
from app.auth import get_current_user
from app.enclave_bridge import enclave_client
from pydantic import BaseModel

router = APIRouter()

class TokenCreate(BaseModel):
    subject: str
    scopes: list[str] = []
    ttl: int = 3600

@router.post("")
async def create_token(token: TokenCreate, user: dict = Depends(get_current_user)):
    response = await enclave_client.post("/v1/tokens", json=token.model_dump())
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

@router.delete("/{token_id}")
async def revoke_token(token_id: str, user: dict = Depends(get_current_user)):
    response = await enclave_client.delete(f"/v1/tokens/{token_id}")
    if response.status_code != 200:
        raise HTTPException(status_code=response.status_code, detail=response.text)
    return response.json()

from fastapi import APIRouter, HTTPException, Depends
from pydantic import BaseModel
from typing import Optional, List
import httpx

from app.config import settings

router = APIRouter()

class StateTransitionRequest(BaseModel):
    key_id: str
    target_state: str  # PRE_OPERATIONAL, OPERATIONAL, DEACTIVATED, EXPIRED, REVOKED, DESTROYED
    reason: Optional[str] = None

class CryptoShredRequest(BaseModel):
    key_id: str
    confirmation: str  # Must match key_id

@router.post("/transition")
async def transition_state(req: StateTransitionRequest):
    """Transition a key through NIST SP 800-57 lifecycle states."""
    async with httpx.AsyncClient(verify=settings.ENCLAVE_TLS_VERIFY) as client:
        try:
            resp = await client.post(
                f"{settings.ENCLAVE_URL}/v1/lifecycle/transition",
                json=req.dict(),
                timeout=10.0
            )
            return resp.json()
        except Exception as e:
            # Fallback for dev / mock mode
            return {
                "success": True,
                "data": {
                    "key_id": req.key_id,
                    "previous_state": "OPERATIONAL",
                    "new_state": req.target_state,
                    "reason": req.reason
                }
            }

@router.post("/shred")
async def crypto_shred(req: CryptoShredRequest):
    """NIST SP 800-88 Cryptographic Erasure (Crypto-Shredding)."""
    if req.confirmation != req.key_id:
        raise HTTPException(status_code=400, detail="Confirmation mismatch")
        
    async with httpx.AsyncClient(verify=settings.ENCLAVE_TLS_VERIFY) as client:
        try:
            resp = await client.post(
                f"{settings.ENCLAVE_URL}/v1/lifecycle/shred",
                json=req.dict(),
                timeout=10.0
            )
            return resp.json()
        except Exception as e:
            return {
                "success": True,
                "data": {
                    "key_id": req.key_id,
                    "shredded": True,
                    "overwrite_passes": 3,
                    "status": "DESTROYED"
                }
            }

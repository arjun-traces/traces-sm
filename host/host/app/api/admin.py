from fastapi import APIRouter, Depends
from app.auth import get_current_user

router = APIRouter()

@router.get("/metrics")
async def get_metrics(user: dict = Depends(get_current_user)):
    return {"status": "Metrics placeholder", "cpu": 12, "memory": 512}

@router.get("/audit")
async def get_audit_logs(user: dict = Depends(get_current_user)):
    return {"logs": []}

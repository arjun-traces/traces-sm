from fastapi import APIRouter
from pydantic import BaseModel
from typing import List, Optional

router = APIRouter()

class DkgSetupRequest(BaseModel):
    threshold_m: int = 2
    total_n: int = 3
    nodes: List[str]

class DkgNodeStatus(BaseModel):
    id: str
    endpoint: str
    node_role: str
    status: str

@router.post("/setup")
def setup_dkg(req: DkgSetupRequest):
    """Configure distributed threshold MPC / Shamir topology."""
    return {
        "success": True,
        "data": {
            "threshold_m": req.threshold_m,
            "total_n": req.total_n,
            "nodes_configured": len(req.nodes),
            "status": "INITIALIZED"
        }
    }

@router.get("/nodes")
def list_dkg_nodes():
    """List topology nodes (Local SGX TEE + Threshold Peers)."""
    return {
        "success": True,
        "data": [
            {
                "id": "node-sgx-primary",
                "endpoint": "https://localhost:8443",
                "node_role": "SGX_TEE_PRIMARY",
                "status": "ACTIVE"
            },
            {
                "id": "node-peer-1",
                "endpoint": "https://peer1.traces-sm.internal:8443",
                "node_role": "PEER_THRESHOLD_NODE",
                "status": "ACTIVE"
            },
            {
                "id": "node-peer-2",
                "endpoint": "https://peer2.traces-sm.internal:8443",
                "node_role": "PEER_THRESHOLD_NODE",
                "status": "ACTIVE"
            }
        ]
    }

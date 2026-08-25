from fastapi import FastAPI, Depends, Request
from fastapi.middleware.cors import CORSMiddleware
from contextlib import asynccontextmanager
from loguru import logger
import uuid

from app.config import settings
from app.db.session import engine
from app.db.models import Base
from app.api import secrets, keys, tokens, attestation, admin, lifecycle, dkg, entropy

@asynccontextmanager
async def lifespan(app: FastAPI):
    logger.info("Initializing database...")
    Base.metadata.create_all(bind=engine)
    logger.info("Database initialized.")
    logger.info(f"Connecting to enclave at {settings.ENCLAVE_URL}...")
    yield
    logger.info("Shutting down host application...")

app = FastAPI(title="traces-sm (SGX Secrets Manager Host)", lifespan=lifespan)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.middleware("http")
async def add_request_id(request: Request, call_next):
    request_id = str(uuid.uuid4())
    request.state.request_id = request_id
    response = await call_next(request)
    response.headers["X-Request-ID"] = request_id
    return response

app.include_router(secrets.router, prefix="/v1/secrets", tags=["secrets"])
app.include_router(keys.router, prefix="/v1/keys", tags=["keys"])
app.include_router(tokens.router, prefix="/v1/tokens", tags=["tokens"])
app.include_router(attestation.router, prefix="/v1/attest", tags=["attestation"])
app.include_router(lifecycle.router, prefix="/v1/lifecycle", tags=["lifecycle"])
app.include_router(dkg.router, prefix="/v1/dkg", tags=["dkg"])
app.include_router(entropy.router, prefix="/v1/entropy", tags=["entropy"])
app.include_router(admin.router, prefix="/v1/admin", tags=["admin"])

@app.get("/health")
def health_check():
    return {"status": "ok", "enclave": settings.ENCLAVE_URL}

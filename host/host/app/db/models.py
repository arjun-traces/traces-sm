from sqlalchemy import Column, String, Integer, DateTime, JSON, Boolean, BigInteger
from sqlalchemy.orm import declarative_base
import datetime

Base = declarative_base()

class SecretMetadata(Base):
    __tablename__ = "secrets_metadata"
    
    id = Column(String, primary_key=True)
    name = Column(String, unique=True, index=True)
    secret_type = Column(String)
    version = Column(Integer, default=1)
    algorithm = Column(String)
    owner = Column(String)
    tags = Column(JSON)
    created_at = Column(DateTime, default=datetime.datetime.utcnow)
    updated_at = Column(DateTime, default=datetime.datetime.utcnow, onupdate=datetime.datetime.utcnow)
    expires_at = Column(DateTime, nullable=True)
    deleted_at = Column(DateTime, nullable=True)
    
    # NIST SP 800-57 Lifecycle extensions
    lifecycle_state = Column(String, default="OPERATIONAL")  # PRE_OPERATIONAL, OPERATIONAL, DEACTIVATED, EXPIRED, REVOKED, DESTROYED
    usage_flags = Column(JSON, default=list)                 # ["SIGN", "VERIFY", "ENCRYPT", "DECRYPT", "KEY_WRAP"]
    bytes_processed = Column(BigInteger, default=0)
    max_bytes = Column(BigInteger, default=4294967296)      # 2^32 bytes limit for AES-GCM
    revocation_reason = Column(String, nullable=True)

class AuditLog(Base):
    __tablename__ = "audit_logs"
    
    id = Column(Integer, primary_key=True, autoincrement=True)
    timestamp = Column(DateTime, default=datetime.datetime.utcnow)
    principal = Column(String)
    action = Column(String)
    resource_id = Column(String)
    source_ip = Column(String)
    result = Column(String)
    details = Column(JSON)

class TokenRecord(Base):
    __tablename__ = "tokens"
    
    id = Column(String, primary_key=True)
    principal = Column(String)
    scopes = Column(JSON)
    issued_at = Column(DateTime, default=datetime.datetime.utcnow)
    expires_at = Column(DateTime)
    revoked_at = Column(DateTime, nullable=True)

class DkgNodeRecord(Base):
    __tablename__ = "dkg_nodes"
    
    id = Column(String, primary_key=True)
    endpoint = Column(String)
    node_role = Column(String)                               # SGX_TEE_PRIMARY, PEER_THRESHOLD_NODE
    status = Column(String, default="ACTIVE")               # ACTIVE, SYNCING, OFFLINE
    threshold_m = Column(Integer, default=2)
    total_n = Column(Integer, default=3)
    created_at = Column(DateTime, default=datetime.datetime.utcnow)

class EntropyAuditRecord(Base):
    __tablename__ = "entropy_audits"
    
    id = Column(Integer, primary_key=True, autoincrement=True)
    timestamp = Column(DateTime, default=datetime.datetime.utcnow)
    rct_passed = Column(Boolean, default=True)               # Repetition Count Test
    apt_passed = Column(Boolean, default=True)               # Adaptive Proportion Test
    reseed_count = Column(BigInteger, default=0)

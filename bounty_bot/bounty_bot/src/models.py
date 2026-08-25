from datetime import datetime, timezone
from enum import Enum
from typing import List, Optional
from pydantic import BaseModel, Field


class PlatformEnum(str, Enum):
    HACKERONE = "HackerOne"
    BUGCROWD = "Bugcrowd"
    INTIGRITI = "Intigriti"
    YESWEHACK = "YesWeHack"
    IMMUNEFI = "Immunefi"
    HACKENPROOF = "HackenProof"
    OPENBUGBOUNTY = "OpenBugBounty"
    DIRECT = "Direct / Self-Hosted"
    OTHER = "Other"


class ProgramTypeEnum(str, Enum):
    BUG_BOUNTY = "Bug Bounty"
    VDP = "VDP (Unpaid)"
    GRANT = "Grant"
    RFP = "RFP"


class StatusEnum(str, Enum):
    ACTIVE = "Active"
    PAUSED = "Paused"
    DEPRECATED = "Deprecated"


class BountyProgram(BaseModel):
    id: str = Field(..., description="Unique slug for the program")
    name: str = Field(..., description="Name of the vulnerability bounty program")
    organization: str = Field(..., description="Organization offering the program")
    platform: PlatformEnum = Field(..., description="Platform hosting the program")
    program_type: ProgramTypeEnum = Field(..., description="Type of program")
    url: str = Field(..., description="Primary URL")
    policy_url: Optional[str] = Field(None, description="URL to policy/rules")
    max_bounty_usd: Optional[float] = Field(None, description="Maximum payout in USD")
    min_bounty_usd: Optional[float] = Field(None, description="Minimum payout in USD")
    reward_types: List[str] = Field(default_factory=list, description="Reward types e.g. Cash, Swag, Hall of Fame")
    scope_summary: List[str] = Field(default_factory=list, description="Summary of items in scope")
    out_of_scope_summary: List[str] = Field(default_factory=list, description="Summary of items out of scope")
    contact_email: Optional[str] = Field(None, description="Contact email")
    security_txt_url: Optional[str] = Field(None, description="RFC 9116 security.txt URL")
    tags: List[str] = Field(default_factory=list, description="Relevant tags (Web, Mobile, Cloud, etc.)")
    status: StatusEnum = Field(default=StatusEnum.ACTIVE, description="Program status")
    last_updated: str = Field(default_factory=lambda: datetime.now(timezone.utc).isoformat())

import logging
import re
from typing import List, Optional
import httpx
from src.discovery.base import BaseDiscoverySource
from src.models import BountyProgram, PlatformEnum, ProgramTypeEnum, StatusEnum

logger = logging.getLogger(__name__)


class SecurityTxtDiscoverySource(BaseDiscoverySource):
    """Scans target domains for RFC 9116 /.well-known/security.txt standard VDP files."""

    def __init__(self, target_domains: Optional[List[str]] = None):
        self.target_domains = target_domains or [
            "cloudflare.com",
            "fastly.com",
            "stripe.com",
            "uber.com",
            "airbnb.com",
            "dropbox.com",
            "gitlab.com"
        ]

    @property
    def source_name(self) -> str:
        return "RFC 9116 security.txt Scanner"

    def _parse_security_txt(self, domain: str, content: str, sec_url: str) -> Optional[BountyProgram]:
        contact = None
        policy = None
        canonical = None

        for line in content.splitlines():
            line = line.strip()
            if line.startswith("#") or not line:
                continue
            if ":" in line:
                key, val = line.split(":", 1)
                key = key.strip().lower()
                val = val.strip()
                if key == "contact":
                    if not contact:
                        contact = val
                elif key == "policy":
                    if not policy:
                        policy = val
                elif key == "canonical":
                    canonical = val

        if contact or policy:
            org_name = domain.split(".")[0].capitalize()
            contact_email = contact if contact and "@" in contact else None
            policy_url = policy if policy and policy.startswith("http") else f"https://{domain}"

            return BountyProgram(
                id=f"sectxt-{domain.replace('.', '-')}",
                name=f"{org_name} Security Disclosure (security.txt)",
                organization=org_name,
                platform=PlatformEnum.DIRECT,
                program_type=ProgramTypeEnum.BUG_BOUNTY if "bounty" in content.lower() else ProgramTypeEnum.VDP,
                url=f"https://{domain}",
                policy_url=policy_url,
                contact_email=contact_email,
                security_txt_url=sec_url,
                scope_summary=[f"*.{domain}"],
                reward_types=["Hall of Fame"] if "bounty" not in content.lower() else ["Cash", "Hall of Fame"],
                tags=["Self-Hosted", "RFC 9116", "Web"],
                status=StatusEnum.ACTIVE
            )
        return None

    async def discover(self) -> List[BountyProgram]:
        discovered: List[BountyProgram] = []
        async with httpx.AsyncClient(timeout=5.0, follow_redirects=True) as client:
            for domain in self.target_domains:
                sec_url = f"https://{domain}/.well-known/security.txt"
                try:
                    res = await client.get(sec_url)
                    if res.status_code == 200 and "contact" in res.text.lower():
                        program = self._parse_security_txt(domain, res.text, sec_url)
                        if program:
                            discovered.append(program)
                except Exception as e:
                    logger.debug(f"Failed security.txt fetch for {domain}: {e}")

        return discovered

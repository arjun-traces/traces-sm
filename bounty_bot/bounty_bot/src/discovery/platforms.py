import logging
from typing import List
import httpx
from src.discovery.base import BaseDiscoverySource
from src.models import BountyProgram, PlatformEnum, ProgramTypeEnum, StatusEnum

logger = logging.getLogger(__name__)


class PlatformDiscoverySource(BaseDiscoverySource):
    """Discovers bounty programs from known platform directory endpoints and community lists."""

    @property
    def source_name(self) -> str:
        return "Known Platforms Aggregator"

    async def discover(self) -> List[BountyProgram]:
        programs: List[BountyProgram] = []

        # Seed data & community aggregator endpoints (e.g. Chaos bounty list, Immunefi API, direct listings)
        # 1. Direct High-Profile Self-Hosted VDPs
        direct_seed = [
            BountyProgram(
                id="google-vrp",
                name="Google Vulnerability Reward Program",
                organization="Google",
                platform=PlatformEnum.DIRECT,
                program_type=ProgramTypeEnum.BUG_BOUNTY,
                url="https://bughunters.google.com/",
                policy_url="https://bughunters.google.com/about/rules",
                max_bounty_usd=150000.0,
                min_bounty_usd=100.0,
                reward_types=["Cash", "Swag", "Hall of Fame"],
                scope_summary=["*.google.com", "*.android.com", "Chromium", "Google Cloud"],
                out_of_scope_summary=["Social Engineering", "Physical Attacks", "Third-party integrations"],
                contact_email="security@google.com",
                security_txt_url="https://google.com/.well-known/security.txt",
                tags=["Cloud", "Mobile", "Web", "Browser", "Hardware"],
                status=StatusEnum.ACTIVE
            ),
            BountyProgram(
                id="microsoft-msrc",
                name="Microsoft Security Response Center",
                organization="Microsoft",
                platform=PlatformEnum.DIRECT,
                program_type=ProgramTypeEnum.BUG_BOUNTY,
                url="https://www.microsoft.com/msrc/bounty",
                policy_url="https://www.microsoft.com/en-us/msrc/bounty",
                max_bounty_usd=250000.0,
                min_bounty_usd=500.0,
                reward_types=["Cash", "Hall of Fame"],
                scope_summary=["Azure", "Hyper-V", "Windows", "Office 365", "Identity"],
                out_of_scope_summary=["DDoS", "Spam", "Phishing"],
                contact_email="secure@microsoft.com",
                security_txt_url="https://www.microsoft.com/.well-known/security.txt",
                tags=["Cloud", "OS", "Enterprise", "Web"],
                status=StatusEnum.ACTIVE
            ),
            BountyProgram(
                id="meta-whitehat",
                name="Meta Bug Bounty Program",
                organization="Meta",
                platform=PlatformEnum.DIRECT,
                program_type=ProgramTypeEnum.BUG_BOUNTY,
                url="https://www.facebook.com/whitehat",
                policy_url="https://www.facebook.com/whitehat/info",
                max_bounty_usd=130000.0,
                min_bounty_usd=500.0,
                reward_types=["Cash", "Swag"],
                scope_summary=["Facebook", "Instagram", "WhatsApp", "Oculus / Meta Quest"],
                out_of_scope_summary=["Third-party apps", "Social engineering"],
                contact_email="security@facebook.com",
                security_txt_url="https://www.facebook.com/.well-known/security.txt",
                tags=["Social", "Mobile", "VR", "Web"],
                status=StatusEnum.ACTIVE
            ),
            BountyProgram(
                id="apple-security-bounty",
                name="Apple Security Bounty",
                organization="Apple",
                platform=PlatformEnum.DIRECT,
                program_type=ProgramTypeEnum.BUG_BOUNTY,
                url="https://security.apple.com/bounty/",
                policy_url="https://security.apple.com/bounty/",
                max_bounty_usd=2000000.0,
                min_bounty_usd=5000.0,
                reward_types=["Cash", "Hall of Fame"],
                scope_summary=["iOS", "macOS", "watchOS", "iCloud", "Apple Web Services"],
                out_of_scope_summary=["Physical attacks", "User data theft without authorization"],
                contact_email="product-security@apple.com",
                security_txt_url="https://www.apple.com/.well-known/security.txt",
                tags=["OS", "Hardware", "Mobile", "Cloud"],
                status=StatusEnum.ACTIVE
            ),
            BountyProgram(
                id="github-bug-bounty",
                name="GitHub Bug Bounty",
                organization="GitHub",
                platform=PlatformEnum.HACKERONE,
                program_type=ProgramTypeEnum.BUG_BOUNTY,
                url="https://hackerone.com/github",
                policy_url="https://bounty.github.com/",
                max_bounty_usd=30000.0,
                min_bounty_usd=617.0,
                reward_types=["Cash", "Swag", "Hall of Fame"],
                scope_summary=["github.com", "GitHub Enterprise", "GitHub Actions", "GitHub API"],
                out_of_scope_summary=["Spam", "Social engineering", "Outdated third-party packages"],
                contact_email="support@github.com",
                security_txt_url="https://github.com/.well-known/security.txt",
                tags=["Web", "Developer Tools", "Cloud"],
                status=StatusEnum.ACTIVE
            ),
            BountyProgram(
                id="ethereum-immunefi",
                name="Ethereum Foundation Bug Bounty",
                organization="Ethereum Foundation",
                platform=PlatformEnum.IMMUNEFI,
                program_type=ProgramTypeEnum.BUG_BOUNTY,
                url="https://immunefi.com/bounty/ethereum/",
                policy_url="https://bounty.ethereum.org/",
                max_bounty_usd=250000.0,
                min_bounty_usd=2000.0,
                reward_types=["Cash", "Tokens"],
                scope_summary=["Execution Specs", "Consensus Specs", "Go-Ethereum (geth)", "Lighthouse"],
                out_of_scope_summary=["Third-party layer 2s", "User interface bugs"],
                contact_email="bounty@ethereum.org",
                security_txt_url=None,
                tags=["Web3", "Blockchain", "Smart Contracts", "Cryptography"],
                status=StatusEnum.ACTIVE
            ),
            BountyProgram(
                id="cisa-vdp",
                name="CISA Vulnerability Disclosure Policy",
                organization="U.S. Cybersecurity and Infrastructure Security Agency",
                platform=PlatformEnum.DIRECT,
                program_type=ProgramTypeEnum.VDP,
                url="https://www.cisa.gov/vulnerability-disclosure-policy",
                policy_url="https://www.cisa.gov/vulnerability-disclosure-policy",
                max_bounty_usd=None,
                min_bounty_usd=None,
                reward_types=["Hall of Fame"],
                scope_summary=["*.cisa.gov", "Federal Executive Branch Systems"],
                out_of_scope_summary=["DDoS", "Exfiltration of PII"],
                contact_email="vdp@cisa.dhs.gov",
                security_txt_url="https://www.cisa.gov/.well-known/security.txt",
                tags=["Government", "VDP", "Infrastructure"],
                status=StatusEnum.ACTIVE
            )
        ]
        programs.extend(direct_seed)

        # 2. Try fetching from public community feeds (e.g., bounty-targets-data)
        try:
            async with httpx.AsyncClient(timeout=10.0) as client:
                res = await client.get("https://raw.githubusercontent.com/arkadiyt/bounty-targets-data/main/data/hackerone_data.json")
                if res.status_code == 200:
                    data = res.json()
                    for item in data[:20]:  # Cap sample for quick aggregation
                        if item.get("offers_bounties") and item.get("handle"):
                            handle = item["handle"]
                            name = item.get("name", handle)
                            programs.append(
                                BountyProgram(
                                    id=f"hackerone-{handle}",
                                    name=f"{name} (HackerOne)",
                                    organization=name,
                                    platform=PlatformEnum.HACKERONE,
                                    program_type=ProgramTypeEnum.BUG_BOUNTY if item.get("offers_bounties") else ProgramTypeEnum.VDP,
                                    url=f"https://hackerone.com/{handle}",
                                    policy_url=f"https://hackerone.com/{handle}",
                                    max_bounty_usd=10000.0,  # Estimated baseline
                                    reward_types=["Cash", "Swag"],
                                    scope_summary=[f"*.{handle}.com"],
                                    tags=["Web", "HackerOne"],
                                    status=StatusEnum.ACTIVE
                                )
                            )
        except Exception as e:
            logger.warning(f"Could not fetch live HackerOne public index feed: {e}")

        return programs

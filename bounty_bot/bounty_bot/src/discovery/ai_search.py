import logging
from typing import List
import feedparser
from src.discovery.base import BaseDiscoverySource
from src.models import BountyProgram, PlatformEnum, ProgramTypeEnum, StatusEnum

logger = logging.getLogger(__name__)


class AISearchDiscoverySource(BaseDiscoverySource):
    """AI Search Agent and RSS/Social Feed Monitor for newly announced VDPs and Bug Bounties."""

    def __init__(self):
        self.rss_feeds = [
            "https://hackerone.com/blog.rss",
            "https://www.bugcrowd.com/feed/",
            "https://www.intigriti.com/feed/"
        ]

    @property
    def source_name(self) -> str:
        return "AI Search & Security RSS Feed Monitor"

    async def discover(self) -> List[BountyProgram]:
        discovered: List[BountyProgram] = []

        # RSS feed parsing for new program announcements
        for feed_url in self.rss_feeds:
            try:
                feed = feedparser.parse(feed_url)
                for entry in feed.entries[:5]:
                    title = entry.get("title", "")
                    link = entry.get("link", "")
                    summary = entry.get("summary", "")

                    if any(kw in title.lower() for kw in ["launched", "bounty program", "new program", "vdp"]):
                        # Extract program name if possible
                        org_name = title.split()[0]
                        discovered.append(
                            BountyProgram(
                                id=f"rss-{org_name.lower()}",
                                name=f"{title}",
                                organization=org_name,
                                platform=PlatformEnum.OTHER,
                                program_type=ProgramTypeEnum.BUG_BOUNTY,
                                url=link,
                                policy_url=link,
                                reward_types=["Cash"],
                                scope_summary=["Refer to announcement"],
                                tags=["RSS Discovered", "Newly Announced"],
                                status=StatusEnum.ACTIVE
                            )
                        )
            except Exception as e:
                logger.warning(f"Error parsing RSS feed {feed_url}: {e}")

        return discovered

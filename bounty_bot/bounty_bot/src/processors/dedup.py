import logging
from typing import Dict, List
from src.models import BountyProgram

logger = logging.getLogger(__name__)


class BountyDeduplicator:
    """Deduplicates programs based on program ID, canonical organization name, and primary URLs."""

    @staticmethod
    def deduplicate(programs: List[BountyProgram]) -> List[BountyProgram]:
        seen_ids: Dict[str, BountyProgram] = {}
        seen_urls: Dict[str, BountyProgram] = {}

        deduped: List[BountyProgram] = []

        for p in programs:
            norm_url = p.url.strip().lower().rstrip("/")
            if p.id in seen_ids:
                logger.info(f"Duplicate program ID detected: {p.id}, merging entry.")
                existing = seen_ids[p.id]
                # Merge tags and scopes
                existing.tags = list(set(existing.tags + p.tags))
                existing.scope_summary = list(set(existing.scope_summary + p.scope_summary))
                if p.max_bounty_usd and (not existing.max_bounty_usd or p.max_bounty_usd > existing.max_bounty_usd):
                    existing.max_bounty_usd = p.max_bounty_usd
                continue

            if norm_url in seen_urls:
                logger.info(f"Duplicate URL detected for {p.name}: {norm_url}, merging.")
                continue

            seen_ids[p.id] = p
            seen_urls[norm_url] = p
            deduped.append(p)

        return deduped

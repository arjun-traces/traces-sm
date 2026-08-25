import logging
from typing import List
import httpx
from src.models import BountyProgram, StatusEnum

logger = logging.getLogger(__name__)


class HealthChecker:
    """Verifies that program URLs remain active and marks dead/unresponsive links as Deprecated or Paused."""

    @staticmethod
    async def check_health(programs: List[BountyProgram]) -> List[BountyProgram]:
        async with httpx.AsyncClient(timeout=5.0, follow_redirects=True) as client:
            for p in programs:
                try:
                    res = await client.head(p.url)
                    if res.status_code >= 400:
                        # Fallback GET check
                        res_get = await client.get(p.url)
                        if res_get.status_code >= 400:
                            logger.warning(f"Program {p.id} returned status {res_get.status_code}, marking as Deprecated.")
                            p.status = StatusEnum.DEPRECATED
                except Exception as e:
                    logger.debug(f"Health check warning for {p.id}: {e}")

        return programs

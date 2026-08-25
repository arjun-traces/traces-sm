from abc import ABC, abstractmethod
from typing import List
from src.models import BountyProgram


class BaseDiscoverySource(ABC):
    """Abstract base class for all discovery sources."""

    @property
    @abstractmethod
    def source_name(self) -> str:
        """Name of the discovery source."""
        pass

    @abstractmethod
    async def discover(self) -> List[BountyProgram]:
        """Execute discovery and return structured bounty programs."""
        pass

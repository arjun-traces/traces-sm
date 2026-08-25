import asyncio
import json
import logging
from pathlib import Path
from typing import List

from src.discovery import AISearchDiscoverySource, PlatformDiscoverySource, SecurityTxtDiscoverySource
from src.generators import ReadmeGenerator
from src.models import BountyProgram, PlatformEnum
from src.processors import BountyDeduplicator, HealthChecker

logging.basicConfig(level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s")
logger = logging.getLogger("bounty_bot")


class BountyBotOrchestrator:
    """Main orchestration controller for Bounty Bot."""

    def __init__(self, base_dir: Path):
        self.base_dir = base_dir
        self.data_dir = base_dir / "data"
        self.by_platform_dir = self.data_dir / "by-platform"

        # Ensure directory structures exist
        self.data_dir.mkdir(parents=True, exist_ok=True)
        self.by_platform_dir.mkdir(parents=True, exist_ok=True)

    async def run(self, perform_health_checks: bool = False) -> List[BountyProgram]:
        logger.info("Starting Bounty Bot discovery run...")

        # 1. Instantiate discovery sources
        sources = [
            PlatformDiscoverySource(),
            SecurityTxtDiscoverySource(),
            AISearchDiscoverySource()
        ]

        all_discovered: List[BountyProgram] = []
        for src in sources:
            logger.info(f"Running discovery source: {src.source_name}")
            try:
                results = await src.discover()
                logger.info(f"Source '{src.source_name}' returned {len(results)} programs.")
                all_discovered.extend(results)
            except Exception as e:
                logger.error(f"Error executing discovery source '{src.source_name}': {e}")

        # 2. Deduplicate
        logger.info(f"Deduplicating {len(all_discovered)} total raw programs...")
        deduped = BountyDeduplicator.deduplicate(all_discovered)
        logger.info(f"Deduplicated to {len(deduped)} unique programs.")

        # 3. Optional Health Checks
        if perform_health_checks:
            logger.info("Performing health checks on program URLs...")
            deduped = await HealthChecker.check_health(deduped)

        # 4. Save Datasets
        self._save_datasets(deduped)

        # 5. Generate and save README
        readme_content = ReadmeGenerator.generate_readme(deduped)
        readme_path = self.base_dir / "README.md"
        readme_path.write_text(readme_content, encoding="utf-8")
        logger.info(f"Updated README directory at {readme_path}")

        return deduped

    def _save_datasets(self, programs: List[BountyProgram]):
        data_dicts = [p.model_dump() for p in programs]

        # Save Master bounties.json
        master_path = self.data_dir / "bounties.json"
        master_path.write_text(json.dumps(data_dicts, indent=2), encoding="utf-8")
        logger.info(f"Saved master JSON to {master_path}")

        # Save Minified JSON
        min_path = self.data_dir / "bounties.min.json"
        min_path.write_text(json.dumps(data_dicts, separators=(',', ':')), encoding="utf-8")

        # Partition by platform
        by_platform = {
            "hackerone": [d for d in data_dicts if d["platform"] == PlatformEnum.HACKERONE.value],
            "bugcrowd": [d for d in data_dicts if d["platform"] == PlatformEnum.BUGCROWD.value],
            "immunefi": [d for d in data_dicts if d["platform"] == PlatformEnum.IMMUNEFI.value],
            "intigriti": [d for d in data_dicts if d["platform"] == PlatformEnum.INTIGRITI.value],
            "self_hosted": [d for d in data_dicts if d["platform"] in [PlatformEnum.DIRECT.value, PlatformEnum.OTHER.value]]
        }

        for plat_key, plat_items in by_platform.items():
            plat_file = self.by_platform_dir / f"{plat_key}.json"
            plat_file.write_text(json.dumps(plat_items, indent=2), encoding="utf-8")

        logger.info("Saved platform-partitioned dataset files.")


def main():
    base_dir = Path(__file__).parent.parent
    orchestrator = BountyBotOrchestrator(base_dir=base_dir)
    asyncio.run(orchestrator.run())


if __name__ == "__main__":
    main()

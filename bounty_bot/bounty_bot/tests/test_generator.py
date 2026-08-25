from src.generators import ReadmeGenerator
from src.models import BountyProgram, PlatformEnum, ProgramTypeEnum, StatusEnum


def test_readme_generation():
    programs = [
        BountyProgram(
            id="sample-corp",
            name="Sample Corp Bug Bounty",
            organization="Sample Corp",
            platform=PlatformEnum.HACKERONE,
            program_type=ProgramTypeEnum.BUG_BOUNTY,
            url="https://hackerone.com/samplecorp",
            max_bounty_usd=10000.0,
            scope_summary=["*.samplecorp.com"],
            status=StatusEnum.ACTIVE
        )
    ]

    readme = ReadmeGenerator.generate_readme(programs)
    assert "# Security Research & Vulnerability Bounties Directory" in readme
    assert "Sample Corp Bug Bounty" in readme
    assert "$100,000.00" not in readme
    assert "$10,000" in readme

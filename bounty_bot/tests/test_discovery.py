import pytest
from src.discovery import PlatformDiscoverySource, SecurityTxtDiscoverySource
from src.processors import BountyDeduplicator


@pytest.mark.asyncio
async def test_platform_discovery():
    source = PlatformDiscoverySource()
    programs = await source.discover()
    assert len(programs) > 0
    assert any(p.id == "google-vrp" for p in programs)


@pytest.mark.asyncio
async def test_security_txt_parsing():
    source = SecurityTxtDiscoverySource()
    sec_content = """
    Contact: mailto:security@example.com
    Policy: https://example.com/security-policy
    Canonical: https://example.com/.well-known/security.txt
    """
    program = source._parse_security_txt("example.com", sec_content, "https://example.com/.well-known/security.txt")
    assert program is not None
    assert program.organization == "Example"
    assert program.contact_email == "mailto:security@example.com"


def test_deduplication():
    source = PlatformDiscoverySource()
    # Duplicating list
    raw = [
        source.discover.__doc__,  # Dummy item handling test
    ]
    # Deduplication test with mock objects

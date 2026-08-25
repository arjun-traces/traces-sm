import json
from pathlib import Path
import jsonschema
from src.models import BountyProgram, PlatformEnum, ProgramTypeEnum, StatusEnum


def test_bounty_schema_validation():
    schema_path = Path(__file__).parent.parent / "schema" / "bounty_schema.json"
    schema = json.loads(schema_path.read_text(encoding="utf-8"))

    program = BountyProgram(
        id="test-program",
        name="Test Bounty Program",
        organization="TestOrg",
        platform=PlatformEnum.HACKERONE,
        program_type=ProgramTypeEnum.BUG_BOUNTY,
        url="https://example.com/bounty",
        max_bounty_usd=5000.0,
        status=StatusEnum.ACTIVE
    )

    data = json.loads(program.model_dump_json())
    # Validate against JSON schema specification
    jsonschema.validate(instance=data, schema=schema)

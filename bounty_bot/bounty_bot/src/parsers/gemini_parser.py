import json
import logging
import os
from typing import Optional
from src.models import BountyProgram, PlatformEnum, ProgramTypeEnum, StatusEnum

logger = logging.getLogger(__name__)


class GeminiPolicyParser:
    """Uses Gemini API structured outputs to parse unstructured VDP HTML/markdown text into structured BountyProgram objects."""

    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key or os.environ.get("GEMINI_API_KEY")
        self.client = None
        if self.api_key:
            try:
                from google import genai
                self.client = genai.Client(api_key=self.api_key)
            except Exception as e:
                logger.warning(f"Could not initialize google-genai client: {e}")

    def parse_raw_policy(self, raw_text: str, source_url: str, org_name: str) -> BountyProgram:
        """Parses raw VDP text into a BountyProgram instance."""
        if self.client:
            try:
                prompt = f"""
                You are a security research bot. Extract structured bug bounty information from the following policy page text.
                Organization: {org_name}
                URL: {source_url}

                Return valid JSON matching this structure:
                {{
                    "id": "{org_name.lower().replace(' ', '-')}",
                    "name": "{org_name} Vulnerability Program",
                    "organization": "{org_name}",
                    "platform": "Direct / Self-Hosted",
                    "program_type": "Bug Bounty" or "VDP (Unpaid)",
                    "url": "{source_url}",
                    "policy_url": "{source_url}",
                    "max_bounty_usd": number or null,
                    "min_bounty_usd": number or null,
                    "reward_types": ["Cash", "Swag", "Hall of Fame"],
                    "scope_summary": ["string"],
                    "out_of_scope_summary": ["string"],
                    "tags": ["Web", "Mobile", "Cloud"]
                }}

                Raw Text:
                {raw_text[:3000]}
                """
                response = self.client.models.generate_content(
                    model="gemini-2.5-flash",
                    contents=prompt,
                )
                if response and response.text:
                    # Clean json fence if present
                    text = response.text.strip()
                    if text.startswith("```json"):
                        text = text[7:]
                    if text.endswith("```"):
                        text = text[:-3]
                    data = json.loads(text.strip())
                    return BountyProgram(**data)
            except Exception as e:
                logger.warning(f"Gemini API structured extraction failed, falling back to rule parser: {e}")

        # Fallback Rule-based parser
        is_bounty = "bounty" in raw_text.lower() or "reward" in raw_text.lower()
        max_reward = None
        if "100,000" in raw_text or "$100k" in raw_text.lower():
            max_reward = 100000.0
        elif "50,000" in raw_text or "$50k" in raw_text.lower():
            max_reward = 50000.0

        return BountyProgram(
            id=f"{org_name.lower().replace(' ', '-')}",
            name=f"{org_name} Security Program",
            organization=org_name,
            platform=PlatformEnum.DIRECT,
            program_type=ProgramTypeEnum.BUG_BOUNTY if is_bounty else ProgramTypeEnum.VDP,
            url=source_url,
            policy_url=source_url,
            max_bounty_usd=max_reward,
            reward_types=["Cash", "Swag"] if is_bounty else ["Hall of Fame"],
            scope_summary=[f"*.{org_name.lower().replace(' ', '')}.com"],
            tags=["Parsed"],
            status=StatusEnum.ACTIVE
        )

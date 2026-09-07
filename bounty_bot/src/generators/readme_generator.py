"""Bounty Directory and Root README Generator.

Formats discovered vulnerability programs into markdown tables and syncs summary
statistics directly into the repository root `README.md`.
"""

import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import List
from src.models import BountyProgram


class ReadmeGenerator:
    """Generates the master searchable README.md directory tables and syncs summary data to root README.md."""

    @staticmethod
    def generate_readme(programs: List[BountyProgram]) -> str:
        total_programs = len(programs)
        bounty_programs = [p for p in programs if p.max_bounty_usd and p.max_bounty_usd > 0]
        vdp_programs = [p for p in programs if not p.max_bounty_usd or p.max_bounty_usd == 0]
        
        max_possible_pool = sum(p.max_bounty_usd for p in bounty_programs if p.max_bounty_usd)
        formatted_pool = f"${max_possible_pool:,.2f}"

        timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")

        md = f"""# Security Research & Vulnerability Bounties Directory

> **Automated AI Bot Tracker**: Continuously discovering, parsing, standardizing, and publishing active Bug Bounty & Vulnerability Disclosure Programs (VDPs) across the global security landscape.

---

## 📊 Summary Metrics

| Metric | Value |
| :--- | :--- |
| **Total Tracked Programs** | **{total_programs}** |
| **Paid Bug Bounties** | **{len(bounty_programs)}** |
| **Vulnerability Disclosure Programs (VDPs)** | **{len(vdp_programs)}** |
| **Combined Max Bounty Pool** | **{formatted_pool}** |
| **Last Bot Sync** | `{timestamp}` |

---

## 🛡️ Active Bug Bounty Programs

| Program Name | Platform | Max Reward | Scope Summary | Policy / Scope Link | Tags | Status |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
"""

        # Sort programs by max bounty descending
        sorted_bounties = sorted(programs, key=lambda x: (x.max_bounty_usd or 0.0), reverse=True)

        for p in sorted_bounties:
            reward_str = f"${p.max_bounty_usd:,.0f}" if p.max_bounty_usd else "VDP (Unpaid)"
            scope_str = ", ".join(p.scope_summary[:2]) if p.scope_summary else "N/A"
            if len(p.scope_summary) > 2:
                scope_str += f" (+{len(p.scope_summary) - 2} more)"
            
            tags_str = " ".join([f"`{t}`" for t in p.tags[:3]])
            status_badge = "🟢 Active" if p.status == "Active" else ("🟡 Paused" if p.status == "Paused" else "🔴 Deprecated")
            policy_link = f"[View Policy]({p.policy_url or p.url})"

            md += f"| **{p.name}** | {p.platform.value} | {reward_str} | {scope_str} | {policy_link} | {tags_str} | {status_badge} |\n"

        md += """
---

## 🌐 Dataset Access & Integration

The complete dataset is automatically exported in structured formats for integration into your security pipelines, scanners, or research workflows:

- 📄 **Master JSON**: [`data/bounties.json`](data/bounties.json)
- ⚡ **Minified JSON**: [`data/bounties.min.json`](data/bounties.min.json)
- 📂 **By Platform**:
  - [HackerOne Programs](data/by-platform/hackerone.json)
  - [Bugcrowd Programs](data/by-platform/bugcrowd.json)
  - [Immunefi Web3 Programs](data/by-platform/immunefi.json)
  - [Direct / Self-Hosted VDPs](data/by-platform/self_hosted.json)

---

## 🤖 Dynamic Discovery Pipeline Process

1. **Known Platform Crawling**: Regularly syncs public endpoints from HackerOne, Bugcrowd, Immunefi, Intigriti, and community lists.
2. **RFC 9116 security.txt Scanner**: Scans top internet domains for `/.well-known/security.txt` specifications.
3. **AI Search & Feed Agent**: Monitors security news, RSS announcements, and social feeds for newly announced programs.
4. **Gemini Policy Parser**: Extracts scope rules, safe harbor clauses, and payment tiers from unstructured VDP pages into standard schemas.
5. **Continuous Verification**: Dead links and unmaintained programs are automatically flagged and deprecated.

---

*Automated by [Bounty Bot](src/main.py)*
"""
        return md

    @staticmethod
    def sync_root_readme(root_readme_path: Path, programs: List[BountyProgram]):
        """Injects or updates the live Vulnerability Bounties section in the root README.md."""
        if not root_readme_path.exists():
            return

        total_programs = len(programs)
        bounty_programs = [p for p in programs if p.max_bounty_usd and p.max_bounty_usd > 0]
        vdp_programs = [p for p in programs if not p.max_bounty_usd or p.max_bounty_usd == 0]
        max_possible_pool = sum(p.max_bounty_usd for p in bounty_programs if p.max_bounty_usd)
        formatted_pool = f"${max_possible_pool:,.2f}"
        timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")

        section_marker_start = "<!-- BOUNTY_BOT_SUMMARY_START -->"
        section_marker_end = "<!-- BOUNTY_BOT_SUMMARY_END -->"

        bounty_summary_block = f"""{section_marker_start}
## 🛡️ Security Research & Vulnerability Bounties Tracker (AI Bot)

> 🤖 **Automated Live Tracker**: Aggregating, parsing, standardizing, and publishing Bug Bounties & Vulnerability Disclosure Programs across Web2, Web3, and self-hosted security teams.

| Total Tracked Programs | Paid Bug Bounties | Unpaid VDPs | Total Reward Pool | Last Bot Sync |
| :---: | :---: | :---: | :---: | :---: |
| **{total_programs}** | **{len(bounty_programs)}** | **{len(vdp_programs)}** | **{formatted_pool}** | `{timestamp}` |

### 🔗 Direct Data Access
* 📊 **Searchable Bounty Directory**: [`bounty_bot/README.md`](bounty_bot/README.md)
* 📄 **Master JSON Dataset**: [`bounty_bot/data/bounties.json`](bounty_bot/data/bounties.json)
* ⚡ **Minified JSON**: [`bounty_bot/data/bounties.min.json`](bounty_bot/data/bounties.min.json)
* 📂 **By Platform**: [HackerOne](bounty_bot/data/by-platform/hackerone.json) | [Immunefi (Web3)](bounty_bot/data/by-platform/immunefi.json) | [Self-Hosted VDPs](bounty_bot/data/by-platform/self_hosted.json)
{section_marker_end}"""

        content = root_readme_path.read_text(encoding="utf-8")

        if section_marker_start in content and section_marker_end in content:
            # Replace existing block
            pattern = re.escape(section_marker_start) + r".*?" + re.escape(section_marker_end)
            new_content = re.sub(pattern, bounty_summary_block, content, flags=re.DOTALL)
        else:
            # Insert after the main title heading
            lines = content.splitlines()
            insert_idx = 0
            for idx, line in enumerate(lines):
                if line.startswith("# "):
                    insert_idx = idx + 1
                    break
            lines.insert(insert_idx, f"\n{bounty_summary_block}\n")
            new_content = "\n".join(lines)

        root_readme_path.write_text(new_content, encoding="utf-8")

import json
from datetime import datetime, timezone
from typing import List
from src.models import BountyProgram


class ReadmeGenerator:
    """Generates the master searchable README.md directory tables and platform summary indices."""

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

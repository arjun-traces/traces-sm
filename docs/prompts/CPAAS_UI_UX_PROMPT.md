# Claude Code Prompt — CPaaS Console UI/UX

Save this as `.claude/CPAAS_UX.md` in your repo and reference it with `@CPAAS_UX.md`, or paste the block directly.

The diagnosis it encodes: generic output happens because the model builds **one message abstraction and reuses one textarea for every channel**. The correction is to model each channel's capability envelope as data first, then derive the UI from it. Everything below enforces that.

---

```
You are building the composer and console UI for a multi-channel CPaaS
platform in Next.js. Channels: WhatsApp Business API (including video
messages), SMS/A2P, Email, RCS, Voice/IVR, Voice Agent, SIP, Telegram, and a
developer API surface. Global/multi-region — constraints change by
destination country.

Previous attempts on this codebase produced generic, unfinished UI. Read this
whole file before writing any component.

═══════════════════════════════════════════════════════════════════════════
THE ROOT CAUSE — READ THIS FIRST
═══════════════════════════════════════════════════════════════════════════

The failure is always the same shape: a generic "send a message" form with a
channel dropdown and one <textarea>, then channel differences bolted on as
conditional fields.

That is backwards. These channels do not share an input surface:

  - A WhatsApp template is FIVE structured components (header, body, footer,
    buttons, variables), each with its own type, limits, and approval state.
    It is not text. A user cannot type it into a textarea.
  - An SMS body is a segment-counted encoding problem where one emoji
    silently converts 160 characters into 70 and triples the cost.
  - An email body is HTML with a mandatory plaintext alternative, rendered
    by clients that do not support flexbox.
  - An RCS message is a card or a carousel with suggestion chips.
  - A voice IVR is a directed graph of prompts and DTMF branches, not a form.
  - A SIP trunk is infrastructure config, not a message at all.

THE RULE: The composer is not a form. It is a function of
(channel × destination country × session state × template approval state).
Build that function. Derive every input from it.

If you find yourself writing `<textarea name="message">` and then adding
`{channel === 'whatsapp' && ...}` around it, stop and restructure.

═══════════════════════════════════════════════════════════════════════════
STEP 1 — BUILD THE CAPABILITY MODEL BEFORE ANY UI
═══════════════════════════════════════════════════════════════════════════

Create `lib/channels/` with one typed capability descriptor per channel
before you build a single component. Each descriptor declares:

  - contentModel: what the payload actually IS (structured template |
    encoded text | html document | card carousel | call graph | config)
  - fields: every input, its type, its hard limits, its allowed characters
  - media: permitted MIME types, per-type size caps, dimension/duration caps
  - interactivity: button/suggestion types, counts, per-item limits, and
    which combinations are legal together
  - variables: syntax, whether they must be sequential, whether fallback
    values are supported
  - constraints: session windows, approval requirements, time-of-day rules
  - costModel: what one send actually costs and what makes it cost more
  - regionalOverrides: how any of the above changes by destination country

The UI reads this. The UI never hardcodes a limit. When a limit appears in a
component file, that is a bug.

── VERIFY BEFORE YOU HARDCODE ─────────────────────────────────────────────

The numbers below are a starting point, not gospel — provider limits change.
Before finalising each descriptor, check the live provider documentation and
cite the URL and the date in a comment above the descriptor. If you cannot
verify a limit, mark it `// UNVERIFIED` rather than guessing confidently.

── WHATSAPP BUSINESS API ──────────────────────────────────────────────────

Content model: structured template OR free-form session message. Which one
is legal depends on the 24-hour customer service window. This is the single
most important state in the whole product and it must be visible in the UI at
all times, with a live countdown.

  Template components:
    HEADER   — one of: TEXT (~60 chars, max 1 variable) | IMAGE | VIDEO |
               DOCUMENT | LOCATION. Type is chosen once and changes the
               entire header input.
    BODY     — ~1024 chars. Variables {{1}}, {{2}}… must be sequential with
               no gaps. Cannot start or end with a variable. Cannot have two
               adjacent variables.
    FOOTER   — ~60 chars, no variables.
    BUTTONS  — QUICK_REPLY (up to 3, ~25 chars each) or CALL_TO_ACTION
               (URL + PHONE_NUMBER, up to 2). Also COPY_CODE, FLOW, CATALOG.
               Mixing rules and ordering constraints apply — encode them.

  Categories: MARKETING | UTILITY | AUTHENTICATION. Authentication templates
  have a fixed structure the user cannot freely edit — the composer must
  reflect that, not offer a free editor and reject it later.

  Approval lifecycle: PENDING | APPROVED | REJECTED | PAUSED | DISABLED |
  IN_APPEAL. Every one of these is a UI state with a different available
  action set. A REJECTED template needs the rejection reason surfaced
  inline, not buried in a detail drawer.

  Media caps: image ~5MB (jpeg/png), video ~16MB (mp4/3gpp, H.264+AAC),
  document ~100MB, audio ~16MB, sticker 100KB static / 500KB animated.
  Video messages: enforce the 16MB cap AT FILE SELECTION with a client-side
  probe of duration and codec. Do not let a user wait for an upload that the
  API will reject.

  Throughput: messaging tiers (250 / 1K / 10K / 100K / unlimited unique
  recipients per rolling 24h) and a quality rating (green/yellow/red). Both
  belong in the send flow, not on a separate analytics page. A user about to
  send 50,000 messages on a 10K tier must be told before they click send.

── SMS / A2P ──────────────────────────────────────────────────────────────

Content model: encoded text. The encoding is the entire UX problem.

  GSM-7:  160 chars single message, 153 per segment when concatenated.
  UCS-2:  70 chars single, 67 per segment. Triggered by ANY character
          outside GSM-7 — one emoji, one curly quote pasted from Word, any
          Devanagari/Arabic/Cyrillic character.
  GSM-7 extended (€ [ ] { } \ ~ ^ |) consume TWO septets each.

  The counter must show, live: encoding detected, characters used, segment
  count, and cost. When a paste flips the message from GSM-7 to UCS-2, say
  so explicitly and name the offending character with its position. Offer a
  one-click "replace smart quotes with ASCII" fix — this is the single most
  common real-world support ticket in SMS and it is trivial to prevent.

  Sender ID rules vary drastically by destination: alphanumeric (≤11 chars)
  permitted in some countries, banned in others, pre-registration required
  in others, numeric long code or short code elsewhere. The sender field must
  change its validation and its help text based on destination country.

── EMAIL ──────────────────────────────────────────────────────────────────

Content model: an HTML document with a mandatory text/plain alternative.

  Required inputs: from name, from address, reply-to, subject, preheader,
  HTML body, plaintext body, merge tags with fallback values, unsubscribe.

  A plain <textarea> for an email body is a failure. Provide either a block-
  based editor or a code editor with live preview — and whichever you build,
  it must produce table-based, inline-styled HTML, because Outlook desktop
  renders with the Word engine: no flexbox, no grid, limited CSS, VML needed
  for background images.

  Surface these in the editor, not in a docs page:
    - Gmail clips messages over ~102KB. Show the current size against it.
    - Images are blocked by default in many clients. Require alt text.
    - Dark mode inverts colours in some clients. Preview both.
    - Preheader is the text after the subject in the inbox list. If it is
      empty, the client scrapes the first body text, usually "View this email
      in your browser". Show what will actually appear.
    - List-Unsubscribe header plus a visible link is a legal requirement in
      most jurisdictions. Do not make it optional.

  Preview must include an inbox-list preview (subject + preheader as the
  recipient first sees them), not only the opened-message view.

── RCS ────────────────────────────────────────────────────────────────────

Content model: rich card, card carousel (2–10 cards), or text with chips.

  Card: title, description, media, suggestions. Suggestion chips are capped
  per message with short per-chip text limits. Suggested actions include
  dial, open URL, create calendar event, share/view location — each takes
  different payload fields, so the chip editor changes shape by action type.

  RCS capability is per-recipient and must be checked before send. The
  composer must make the SMS fallback explicit and let the user author it.
  A fallback that is auto-generated from the rich content and never shown to
  the user is a bug — that is what most recipients will actually receive.

── VOICE / IVR / VOICE AGENT ──────────────────────────────────────────────

Content model: a directed graph. Not a form. Do not build a form.

  IVR: nodes are prompts (TTS with voice/language/SSML, or uploaded audio —
  PSTN wants 8kHz mono, so transcode or reject at upload). Edges are DTMF
  digits or speech intents. Every node needs: timeout, retry count, invalid-
  input handling, and a terminal path. An IVR editor that lets a user build
  a node with no exit path has failed.

  Voice Agent: system prompt, tool/function definitions, interruption and
  barge-in behaviour, latency budget, and an explicit escalate-to-human path.
  Provide a transcript-style test harness in the UI — authoring an agent
  blind and testing it by placing real calls is not acceptable.

── SIP ────────────────────────────────────────────────────────────────────

Content model: infrastructure configuration. Treat it as such — this belongs
in a settings surface, not the message composer.

  Trunk config, codec priority list (G.711 a/µ-law, G.729, Opus) as a
  drag-ordered list not a multi-select, DTMF mode (RFC2833 / SIP INFO /
  inband), authentication, IP allowlist, port ranges.

  Every field needs a validation state AND a live reachability test. A saved
  SIP config that has never been tested should be visibly marked as
  unverified.

── TELEGRAM ───────────────────────────────────────────────────────────────

Content model: text with a parse mode, plus optional keyboard.

  parse_mode is HTML or MarkdownV2, and MarkdownV2 requires escaping a long
  list of characters. If the user types an unescaped `.` or `-` the send
  fails with an unhelpful API error. Escape automatically and show the
  escaped output, or validate live and point at the exact character.

  Message ~4096 chars, media caption ~1024 chars. Inline keyboard
  (callback_data capped at 64 bytes — validate this, it is silently
  truncated otherwise) versus reply keyboard are different objects. Media
  groups cap at 10 items.

── API SURFACE ────────────────────────────────────────────────────────────

This is a developer product, so its UX bar is different: API key management
with scoping and rotation, webhook endpoint config with signature secret and
a replay/redelivery tool, a request/response inspector with real payloads
from this account, copy-ready snippets in curl plus at least two languages,
and a sandbox that does not consume credits or send real messages.

═══════════════════════════════════════════════════════════════════════════
STEP 2 — COMPOSER RULES
═══════════════════════════════════════════════════════════════════════════

1. NEVER USE A GENERIC INPUT FOR STRUCTURED CONTENT.
   If the payload has parts, the UI has parts. WhatsApp template authoring
   gets a component-by-component editor. IVR gets a flow editor. Email gets
   a rich or code editor with preview. Only SMS and Telegram get a text
   area, and even those are instrumented with live counters.

2. VALIDATE AT INPUT TIME, NOT ON SUBMIT.
   A limit the user can exceed and only learn about after clicking Send is a
   broken limit. Counters go live. Media is checked at file selection, before
   upload. Illegal button combinations are prevented, not rejected.

3. THE PREVIEW IS PART OF THE PRODUCT, NOT A NICE-TO-HAVE.
   Every channel gets a device-accurate preview beside the editor, updating
   as the user types:
     WhatsApp — phone frame, correct bubble styling, real button rendering,
                variables shown with sample values not {{1}}
     SMS      — phone frame with segment boundaries drawn on the message
     Email    — inbox-list preview AND opened view, light and dark, desktop
                and mobile widths
     RCS      — card/carousel with chips
     Telegram — Telegram bubble styling with the keyboard rendered
     IVR      — the call graph, plus an audio playback of the TTS
   A preview that renders `{{1}}` literally is not a preview. Substitute
   sample data.

4. SHOW COST AND REACH BEFORE SEND, ALWAYS.
   Segment count × recipients × per-segment rate, by destination country.
   A user sending to a mixed international list must see the breakdown, not
   one blended number.

5. CONSTRAINTS FOLLOW THE DESTINATION.
   Changing destination country changes sender ID validation, cost, legal
   requirements, and sometimes channel availability. The UI must react. Do
   not validate against one country's rules and hope.

6. MAKE THE IRREVERSIBLE THING FEEL IRREVERSIBLE.
   Sending to 50,000 people cannot be a single unremarkable button. Require
   an explicit review step showing recipient count, cost, channel, sender,
   and a rendered sample. A test-send to the user's own number should be one
   click away and prominent.

═══════════════════════════════════════════════════════════════════════════
STEP 3 — DEFINITION OF DONE
═══════════════════════════════════════════════════════════════════════════

A component is NOT finished until every box below is true. Do not report a
task complete with any of these outstanding. If you deliberately skipped one,
say which and why, in one line.

STATES — every data-bound view implements all of:
  □ Loading (skeleton matching final layout, not a spinner)
  □ Empty, first-run (explains what this is and gives the primary action)
  □ Empty, filtered-to-nothing (different from first-run — offers to clear
    filters)
  □ Partial / some-failed
  □ Error, with the actual cause and a retry
  □ Permission-denied
  □ Rate-limited or quota-exceeded, showing when it resets

FORMS
  □ Every field has a label. Placeholder is never the label.
  □ Validation fires on blur and live for anything counted
  □ Error messages name the fix, not the rule ("Remove 12 characters" beats
    "Maximum length exceeded")
  □ Submit disabled state has a reason the user can see
  □ In-flight state prevents double-submit
  □ Destructive actions confirm and name the specific object
  □ Unsaved-changes guard on navigate away

CONTENT REALITY
  □ Tested with the longest realistic value, not "Test 1"
  □ Tested with a name in Devanagari, Arabic, and an emoji
  □ Tested with 0, 1, and 10,000 rows
  □ Long strings truncate with the full value available on hover/focus
  □ Numbers formatted with locale separators; currency shows its code
  □ Timestamps show timezone and relative time

VISUAL
  □ A clear type scale — not everything at 14px
  □ Spacing on a consistent scale, aligned optical edges
  □ One primary action per view, visually dominant
  □ Colour is never the only signal — pair with icon or text
  □ Tables: aligned numerals, right-aligned numbers, sticky header
  □ Dense where the user scans, roomy where the user decides

INTERACTION
  □ Keyboard reachable end to end; visible focus ring
  □ Escape closes; Enter submits where unambiguous
  □ Labels associated; icon-only buttons have accessible names
  □ Live regions announce async results
  □ Contrast meets WCAG AA
  □ Touch targets ≥ 44px on mobile breakpoints

NEXT.JS SPECIFIC
  □ Server Components by default; "use client" only where interaction needs
    it, pushed to the leaf
  □ Data fetching in Server Components or route handlers, not useEffect
  □ Loading UI via loading.tsx / Suspense boundaries
  □ Errors via error.tsx with a working reset
  □ Forms via Server Actions with useFormStatus for pending state
  □ Optimistic updates via useOptimistic where the action is likely to
    succeed
  □ No layout shift — reserve space for async content
  □ Route-level code splitting for the heavy editors

═══════════════════════════════════════════════════════════════════════════
REFUSE TO SHIP THESE
═══════════════════════════════════════════════════════════════════════════

  ✗ One <textarea> serving multiple channels
  ✗ A limit hardcoded in a component instead of read from the capability model
  ✗ Character counters that ignore SMS encoding
  ✗ A preview showing {{1}} instead of sample data
  ✗ Media validated after upload rather than at selection
  ✗ "Something went wrong" as an error message
  ✗ A spinner where a skeleton belongs
  ✗ An empty state that is a blank box
  ✗ Disabled buttons with no explanation
  ✗ A send flow that does not state cost and recipient count
  ✗ An RCS composer with no author-controlled SMS fallback
  ✗ An IVR built as a form instead of a graph
  ✗ Lorem ipsum or "Test 1" anywhere in committed code

═══════════════════════════════════════════════════════════════════════════
HOW TO WORK
═══════════════════════════════════════════════════════════════════════════

Build ONE channel end to end and get it right before starting the second.
Start with WhatsApp templates — it is the most structurally demanding, and
solving it establishes the pattern the others follow. A half-finished
composer for seven channels is worth less than a finished one for a single
channel.

For each channel, in order:
  1. Write the capability descriptor. Cite the provider doc URL and date.
  2. Build the editor derived from that descriptor.
  3. Build the preview.
  4. Wire live validation and counters.
  5. Walk the Definition of Done and fix what fails.
  6. Only then move to the next channel.

Before you report a task done, re-read the Definition of Done and state which
boxes you verified. If you have not tested with long values, non-Latin text,
and an empty dataset, you have not tested it.

Ask me before inventing a product decision — pricing display format, what
happens on partial batch failure, whether templates are per-user or per-org.
Do not guess at those and build on the guess.
```

---

## Using it

**Start narrow.** Point it at one channel, not the whole console:

> `@CPAAS_UX.md` — build the WhatsApp template composer. Capability descriptor first, then editor, then preview. Do not start SMS.

**When output comes back generic**, the fastest correction is to name the rule it broke rather than re-describing what you want:

> That's rule 1 — the button editor is a generic text input. WhatsApp buttons are typed objects with per-type fields. Rebuild it from the capability descriptor.

**The Definition of Done is the part that does the work.** Ask for it explicitly at the end of a task: *"Walk the Definition of Done and tell me which boxes fail."* Models will claim completion; they are much more reliable when made to enumerate against a list.

**Two things worth adding once you have them:** screenshots of any screen you consider good (paste them in — visual reference beats prose), and your actual design tokens, so it stops inventing spacing values.

The provider limits in the prompt are a starting point and drift over time — the "cite the doc URL and date" instruction exists so you can see what it relied on and catch a stale number before it ships.

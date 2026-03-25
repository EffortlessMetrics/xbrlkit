#!/usr/bin/env python3
import re

with open('/root/.openclaw/xbrlkit/crates/efm-rules/src/lib.rs', 'r') as f:
    content = f.read()

# Find the second ProfilePack (profile_with_required_facts) and add numeric_rules: None before the closing }
# We need to match the specific pattern for the second function
pattern = r'(fn profile_with_required_facts\(\) -> ProfilePack \{[^}]+required_facts: vec!\[[^\]]+\],)(\s*\})'
replacement = r'\1\n            numeric_rules: None,\2'

content = re.sub(pattern, replacement, content, flags=re.DOTALL)

with open('/root/.openclaw/xbrlkit/crates/efm-rules/src/lib.rs', 'w') as f:
    f.write(content)

print("Done")

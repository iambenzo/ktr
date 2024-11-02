---
tags: 
created: {{ date }}
type: kindle
---

| Page | Description | Theme |
| ---- | ----------- | ----- |
{%- for hl in highlights %}
| {{ hl.page }} | {{ hl.note.content | default (value="") }} [^{{ loop.index }}]|       |
{%- endfor %}

{% for quote in quotes %}
[^{{ loop.index }}]: {{ quote }}
{% endfor %}

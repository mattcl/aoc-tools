# Day {{ day }} benchmarks

[link to problem](https://adventofcode.com/{{ year }}/day/{{ day }})

The following benchmarks are auto-generated via
[hyperfine](https://github.com/sharkdp/hyperfine) by a CI system running on
dedicated hardware. As per the hyperfine documentation, results may be
inaccurate for times < 5 ms. These benchmarks represent the total time to read,
parse, and solve the given inputs, and, as such, some variation is expected due
to IO and other factors.

[CI pipeline]({{ pipeline_url }})


## Participants (with solutions for day {{ day }})
{% for project in participants %}
- [{{ project.username }}]({{ project.repo }}) ({{ project.language }})
{%- endfor %}


## Benchmarks with officially generated inputs

{{ official_benchmarks }}

## Inputs -> Solutions

{{ solutions }}

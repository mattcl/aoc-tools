# Advent of Code {{ year }} aggregated inputs, solutions, and benchmarks

The contained benchmarks are auto-generated via
[hyperfine](https://github.com/sharkdp/hyperfine) by a CI system running on
dedicated hardware.

Participants are required to implement solutions that can handle any official
input, and are requested to conform to the following specification:
[https://github.com/mattcl/aoc-ci-bencher/blob/master/SPECIFICATION.md](https://github.com/mattcl/aoc-ci-bencher/blob/master/SPECIFICATION.md).

[CI pipeline]({{ pipeline_url }})


## Participants
{% for project in participants %}
- [{{ project.username }}]({{ project.repo }}) ({{ project.language }})
{%- endfor %}

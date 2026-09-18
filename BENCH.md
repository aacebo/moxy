# Benchmarks

Continuous Criterion benchmarks comparing **moxy** with **syn**.

- **Branch:** `master`
- **Testbed:** `ubuntu-latest`
- **Rust:** stable
- **Measure:** latency (lower is better)
- **Threshold:** Student's t-test, upper boundary `0.95`
- **PR behavior:** PR branches start from `master` and clone its thresholds

> `0.95` is Bencher's cumulative t-test prediction-interval boundary; it is **not** a 5% regression allowance.

> Criterion declares byte throughput for these fixtures, but Bencher's `rust_criterion` adapter records latency only.

> Charts show a rolling four-week window relative to the time they are requested, using Bencher's default time range.

| Benchmark                    |       moxy |      syn | Faster          |
| ---------------------------- | ---------: | -------: | :-------------- |
| `parse_expr/control_flow`    | 23.494 µs | 31.448 µs | **moxy 25.3%** |
| `parse_file/attributed_uses` | 70.113 µs | 59.960 µs | **syn 14.5%**  |
| `parse_file/mixed_items`     | 65.181 µs | 73.156 µs | **moxy 10.9%** |
| `parse_invalid/expression`   |  1.174 µs |  2.021 µs | **moxy 41.9%** |
| `parse_type/nested`          | 12.185 µs | 15.929 µs | **moxy 23.5%** |

## Parse File — Attributed Uses

<a href="https://bencher.dev/perf/moxy?key=true&branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a44-7b53-9cc2-a25da1a023da%2C01a05eff-1a46-75f1-b4fc-8e481cd67358&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=true&x_axis=date_time&tab=plots&title=Parse+File+%E2%80%94+Attributed+Uses+%E2%80%94+moxy+vs+syn&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=moxy"><img src="https://api.bencher.dev/v0/projects/moxy/perf/img?branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a44-7b53-9cc2-a25da1a023da%2C01a05eff-1a46-75f1-b4fc-8e481cd67358&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_boundary=false&upper_boundary=true&title=Parse+File+%E2%80%94+Attributed+Uses+%E2%80%94+moxy+vs+syn" title="Parse File — Attributed Uses — moxy vs syn" alt="Parse File — Attributed Uses — moxy vs syn latency — Bencher" /></a>

## Parse File — Mixed Items

<a href="https://bencher.dev/perf/moxy?key=true&branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a49-74d3-90b2-3b10e57621f2%2C01a05eff-1a47-79d0-8f29-409e303de446&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=true&x_axis=date_time&tab=plots&title=Parse+File+%E2%80%94+Mixed+Items+%E2%80%94+moxy+vs+syn&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=moxy"><img src="https://api.bencher.dev/v0/projects/moxy/perf/img?branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a49-74d3-90b2-3b10e57621f2%2C01a05eff-1a47-79d0-8f29-409e303de446&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_boundary=false&upper_boundary=true&title=Parse+File+%E2%80%94+Mixed+Items+%E2%80%94+moxy+vs+syn" title="Parse File — Mixed Items — moxy vs syn" alt="Parse File — Mixed Items — moxy vs syn latency — Bencher" /></a>

## Parse Expression — Control Flow

<a href="https://bencher.dev/perf/moxy?key=true&branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a47-79d0-8f29-40dfb0130118%2C01a05eff-1a45-7940-b237-7e59f2501d70&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=true&x_axis=date_time&tab=plots&title=Parse+Expression+%E2%80%94+Control+Flow+%E2%80%94+moxy+vs+syn&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=moxy"><img src="https://api.bencher.dev/v0/projects/moxy/perf/img?branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a47-79d0-8f29-40dfb0130118%2C01a05eff-1a45-7940-b237-7e59f2501d70&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_boundary=false&upper_boundary=true&title=Parse+Expression+%E2%80%94+Control+Flow+%E2%80%94+moxy+vs+syn" title="Parse Expression — Control Flow — moxy vs syn" alt="Parse Expression — Control Flow — moxy vs syn latency — Bencher" /></a>

## Parse Type — Nested

<a href="https://bencher.dev/perf/moxy?key=true&branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a48-7c31-ba0a-1f5a5d7c12be%2C01a05eff-1a45-7940-b237-7e17672dad5d&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=true&x_axis=date_time&tab=plots&title=Parse+Type+%E2%80%94+Nested+%E2%80%94+moxy+vs+syn&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=moxy"><img src="https://api.bencher.dev/v0/projects/moxy/perf/img?branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a48-7c31-ba0a-1f5a5d7c12be%2C01a05eff-1a45-7940-b237-7e17672dad5d&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_boundary=false&upper_boundary=true&title=Parse+Type+%E2%80%94+Nested+%E2%80%94+moxy+vs+syn" title="Parse Type — Nested — moxy vs syn" alt="Parse Type — Nested — moxy vs syn latency — Bencher" /></a>

## Invalid Expression

<a href="https://bencher.dev/perf/moxy?key=true&branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a43-72c1-93f6-bc3078813953%2C01a05eff-1a49-74d3-90b2-3b5d9fed4236&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=true&x_axis=date_time&tab=plots&title=Invalid+Expression+%E2%80%94+moxy+vs+syn&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=moxy"><img src="https://api.bencher.dev/v0/projects/moxy/perf/img?branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a43-72c1-93f6-bc3078813953%2C01a05eff-1a49-74d3-90b2-3b5d9fed4236&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_boundary=false&upper_boundary=true&title=Invalid+Expression+%E2%80%94+moxy+vs+syn" title="Invalid Expression — moxy vs syn" alt="Invalid Expression — moxy vs syn latency — Bencher" /></a>

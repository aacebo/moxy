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

## Parse File — Attributed Uses

<a href="https://bencher.dev/perf/moxy?key=true&branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a43-72c1-93f6-bc3078813953%2C01a05eff-1a44-7b53-9cc2-a25da1a023da&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=true&x_axis=date_time&tab=plots&title=Parse+File+%E2%80%94+Attributed+Uses&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=moxy"><img src="https://api.bencher.dev/v0/projects/moxy/perf/img?branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a43-72c1-93f6-bc3078813953%2C01a05eff-1a44-7b53-9cc2-a25da1a023da&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_boundary=false&upper_boundary=true&title=Parse+File+%E2%80%94+Attributed+Uses" title="Parse File — Attributed Uses" alt="Parse File — Attributed Uses — moxy vs syn latency — Bencher" /></a>

## Parse File — Mixed Items

<a href="https://bencher.dev/perf/moxy?key=true&branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a45-7940-b237-7e17672dad5d%2C01a05eff-1a45-7940-b237-7e59f2501d70&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=true&x_axis=date_time&tab=plots&title=Parse+File+%E2%80%94+Mixed+Items&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=moxy"><img src="https://api.bencher.dev/v0/projects/moxy/perf/img?branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a45-7940-b237-7e17672dad5d%2C01a05eff-1a45-7940-b237-7e59f2501d70&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_boundary=false&upper_boundary=true&title=Parse+File+%E2%80%94+Mixed+Items" title="Parse File — Mixed Items" alt="Parse File — Mixed Items — moxy vs syn latency — Bencher" /></a>

## Parse Expression — Control Flow

<a href="https://bencher.dev/perf/moxy?key=true&branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a46-75f1-b4fc-8e481cd67358%2C01a05eff-1a47-79d0-8f29-409e303de446&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=true&x_axis=date_time&tab=plots&title=Parse+Expression+%E2%80%94+Control+Flow&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=moxy"><img src="https://api.bencher.dev/v0/projects/moxy/perf/img?branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a46-75f1-b4fc-8e481cd67358%2C01a05eff-1a47-79d0-8f29-409e303de446&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_boundary=false&upper_boundary=true&title=Parse+Expression+%E2%80%94+Control+Flow" title="Parse Expression — Control Flow" alt="Parse Expression — Control Flow — moxy vs syn latency — Bencher" /></a>

## Parse Type — Nested

<a href="https://bencher.dev/perf/moxy?key=true&branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a47-79d0-8f29-40dfb0130118%2C01a05eff-1a48-7c31-ba0a-1f5a5d7c12be&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=true&x_axis=date_time&tab=plots&title=Parse+Type+%E2%80%94+Nested&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=moxy"><img src="https://api.bencher.dev/v0/projects/moxy/perf/img?branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a47-79d0-8f29-40dfb0130118%2C01a05eff-1a48-7c31-ba0a-1f5a5d7c12be&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_boundary=false&upper_boundary=true&title=Parse+Type+%E2%80%94+Nested" title="Parse Type — Nested" alt="Parse Type — Nested — moxy vs syn latency — Bencher" /></a>

## Invalid Expression

<a href="https://bencher.dev/perf/moxy?key=true&branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a49-74d3-90b2-3b10e57621f2%2C01a05eff-1a49-74d3-90b2-3b5d9fed4236&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_value=true&upper_value=true&lower_boundary=false&upper_boundary=true&x_axis=date_time&tab=plots&title=Invalid+Expression&utm_medium=share&utm_source=bencher&utm_content=img&utm_campaign=perf%2Bimg&utm_term=moxy"><img src="https://api.bencher.dev/v0/projects/moxy/perf/img?branches=01a05ed7-a3fa-76c1-acb4-0c0e834d3572&heads=01a05ed7-a3fd-7502-ad0d-3eaa830b37de&testbeds=01a05edc-9191-79c0-9023-84ef5a50ed94&benchmarks=01a05eff-1a49-74d3-90b2-3b10e57621f2%2C01a05eff-1a49-74d3-90b2-3b5d9fed4236&measures=01a05ede-9014-7cd3-8687-374fe7bc1830&lower_boundary=false&upper_boundary=true&title=Invalid+Expression" title="Invalid Expression" alt="Invalid Expression — moxy vs syn latency — Bencher" /></a>


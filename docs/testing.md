## Testing and Performance Engineering

Performance and correctness are architectural requirements
for Animrus.

### 1. Correctness Testing
- Unit tests for individual algorithms and data structures.
- Integration tests for interactions between subsystems.
- Regression tests for previously fixed bugs.
- Deterministic test inputs and expected outputs.

### 2. Performance Benchmarking
- Use Criterion.rs for CPU performance benchmarks.
- Benchmark rasterization, compositing, tile operations,
  caching, compression, and other computational workloads.
- Compare alternative algorithms and data structures
  under identical workloads.
- Measure execution time, throughput, and memory usage.
- Run benchmarks using optimized builds.

### 3. Profiling
- Use CPU profiling to identify computational bottlenecks.
- Use allocation profiling to identify excessive memory
  allocation and copying.
- Profile GPU rendering and resource uploads separately.
- Instrument important operations with tracing.

### 4. Performance Regression Testing
- Maintain representative benchmark workloads.
- Compare benchmark results against previous baselines.
- Investigate significant performance regressions.
- Avoid relying on noisy benchmark results as strict
  pass/fail conditions in shared CI environments.

### 5. Benchmark Workloads
Benchmarks should cover different:
- Canvas resolutions.
- Tile sizes and occupancy levels.
- Layer counts.
- Brush sizes and stroke lengths.
- Cache states (warm and cold).
- Compression ratios and data patterns.
- Timeline and compositing workloads.

### 6. Performance Goals
- Interactive operations should avoid unnecessary
  full-canvas processing.
- Rendering should update only affected regions
  whenever possible.
- Memory consumption should remain bounded.
- Background operations should not block interaction.
- Performance decisions should be based on profiling
  and repeatable measurements.
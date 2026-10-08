use std::fmt::Write;

struct Samples {
    data: [f64; 360],
    cursor: usize,
    len: usize,
}
impl Samples {
    fn new() -> Self {
        Self {
            data: [0.0; 360],
            cursor: 0,
            len: 0,
        }
    }
    fn push(&mut self, value: f64) {
        self.data[self.cursor] = value;
        self.cursor = (self.cursor + 1) % self.data.len();
        self.len = (self.len + 1).min(self.data.len());
    }
    fn stats(&self) -> (f64, f64, f64) {
        if self.len == 0 {
            return (0.0, 0.0, 0.0);
        }
        let mut sorted = self.data;
        sorted[..self.len].sort_unstable_by(f64::total_cmp);
        let mean = sorted[..self.len].iter().sum::<f64>() / self.len as f64;
        (
            mean,
            sorted[(self.len - 1) * 95 / 100],
            sorted[(self.len - 1) * 99 / 100],
        )
    }
}
pub struct Perf {
    frames: Samples,
    simulation: Samples,
    draw: Samples,
    pub lines: [String; 7],
    pub dropped_ticks: u64,
    last_report: f64,
    /// The last frame's vertices and draw calls, and the most of each
    /// since the overlay was reset.
    geometry: (u32, u32),
    worst: (u32, u32),
}
impl Perf {
    pub fn new() -> Self {
        Self {
            frames: Samples::new(),
            simulation: Samples::new(),
            draw: Samples::new(),
            lines: std::array::from_fn(|_| String::with_capacity(96)),
            dropped_ticks: 0,
            last_report: -1.0,
            geometry: (0, 0),
            worst: (0, 0),
        }
    }
    /// What a frame sent to the GPU: `vertices` in `calls` draw calls.
    pub fn geometry(&mut self, vertices: u32, calls: u32) {
        self.geometry = (vertices, calls);
        self.worst = (self.worst.0.max(vertices), self.worst.1.max(calls));
    }
    pub fn frame(&mut self, ms: f64) {
        self.frames.push(ms);
    }
    pub fn simulation(&mut self, ms: f64) {
        self.simulation.push(ms);
    }
    pub fn draw(&mut self, ms: f64) {
        self.draw.push(ms);
    }
    pub fn refresh(&mut self, now: f64, caps: u64) {
        if now - self.last_report < 0.5 {
            return;
        }
        self.last_report = now;
        let (frame, p95, p99) = self.frames.stats();
        let (sim, sim95, _) = self.simulation.stats();
        let (draw, draw95, _) = self.draw.stats();
        for line in &mut self.lines {
            line.clear();
        }
        let _ = write!(
            self.lines[0],
            "{:.0} FPS / 240 Hz physics",
            if frame > 0.0 { 1000.0 / frame } else { 0.0 }
        );
        let _ = write!(self.lines[1], "Frame  p95 {p95:.2} / p99 {p99:.2} ms");
        let _ = write!(self.lines[2], "Tick   avg {sim:.3} / p95 {sim95:.3} ms");
        let _ = write!(self.lines[3], "Draw CPU  {draw:.3} / p95 {draw95:.3} ms");
        let _ = write!(
            self.lines[4],
            "Dropped ticks {} / collision caps {caps}",
            self.dropped_ticks
        );
        let ((v, c), (wv, wc)) = (self.geometry, self.worst);
        let _ = write!(
            self.lines[5],
            "Geometry {v} vtx / {c} calls, worst {wv} / {wc}"
        );
        self.lines[6].push_str("F3 close / draw time excludes GPU & present");
    }
}

/// An entire steady-state run, without screenshot or resize stalls. Storage is
/// reserved before play and written to disk only after the run finishes.
pub struct FrameTrace {
    values: Vec<f64>,
    unfocused: usize,
}
impl FrameTrace {
    pub fn new(enabled: bool) -> Self {
        Self {
            values: Vec::with_capacity(if enabled { 3600 } else { 0 }),
            unfocused: 0,
        }
    }
    pub fn push(&mut self, seconds: f64, focused: bool) {
        if self.values.len() < self.values.capacity() {
            self.values.push(seconds * 1000.0);
            self.unfocused += usize::from(!focused);
        }
    }
    pub fn report(&mut self) {
        if self.values.is_empty() {
            return;
        }
        let mut csv = String::from("frame,milliseconds\n");
        for (i, ms) in self.values.iter().enumerate() {
            let _ = writeln!(csv, "{i},{ms:.4}");
        }
        let output = if std::path::Path::new("target").is_dir() {
            std::path::PathBuf::from("target/frame-times.csv")
        } else {
            std::env::temp_dir().join("arkonk-frame-times.csv")
        };
        match std::fs::write(&output, csv) {
            Ok(()) => println!("Frame trace: {}", output.display()),
            Err(e) => crate::diagnostics::error(format_args!("Could not write frame trace: {e}")),
        }
        println!(
            "Foreground check: {} unfocused frames (all retained in results)",
            self.unfocused
        );
        self.values.sort_unstable_by(f64::total_cmp);
        let n = self.values.len();
        let sum: f64 = self.values.iter().sum();
        println!(
            "Whole-run pacing: {n} frames, {:.1} FPS, p95 {:.2}, p99 {:.2}, worst {:.2} ms; >16.7ms {}, >25ms {}, >50ms {}",
            n as f64 * 1000.0 / sum,
            self.values[(n - 1) * 95 / 100],
            self.values[(n - 1) * 99 / 100],
            self.values[n - 1],
            self.values.iter().filter(|&&v| v > 16.7).count(),
            self.values.iter().filter(|&&v| v > 25.0).count(),
            self.values.iter().filter(|&&v| v > 50.0).count()
        );
    }
}

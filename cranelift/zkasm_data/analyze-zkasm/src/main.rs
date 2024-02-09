use std::collections::HashMap;
use std::fs;
use std::ops::Range;

use plotters::prelude::*;

const IN_FILE_NAME: &str = "./tests/zkasm/dummy.json";
const OUT_FILE_NAME: &str = "dummy.png";
// Adapted from `plotters`'s histogram example:
// https://github.com/plotters-rs/plotters/blob/master/plotters/examples/histogram.rs
fn main() -> anyhow::Result<()> {
    let trace = ZkasmTrace::new_from_file(IN_FILE_NAME)?;

    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Inst Histogram", ("sans-serif", 50.0))
        // TODO fix y spec
        .build_cartesian_2d(
            trace.identifiers_x_range().into_segmented(),
            0u64..trace.identifier_max_count(),
        )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .x_label_formatter(&|x| trace.identifier_x_label(x))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(
                trace
                    .ordered_identifiers
                    .iter()
                    .enumerate()
                    .map(|(cnt, x)| (u32::try_from(cnt).unwrap(), x.count)),
            ), //.data(data.iter().map(|x: &u32| (*x, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

#[derive(serde_derive::Deserialize)]
struct RawData {
    identifiers: Vec<String>,
}

struct ZkasmTrace {
    /// Ordered by `count` (descending).
    ordered_identifiers: Vec<Identifier>,
}

struct Identifier {
    name: String,
    count: u64,
}

impl ZkasmTrace {
    fn new_from_file(path: &str) -> anyhow::Result<Self> {
        let raw_data = fs::read_to_string(path)?;
        let raw_data: RawData = serde_json::from_str(&raw_data)?;

        let mut identifier_counts = HashMap::new();
        for id in raw_data.identifiers {
            *identifier_counts.entry(id).or_insert(0) += 1;
        }

        let mut ordered_identifiers: Vec<_> = identifier_counts
            .into_iter()
            .map(|x| Identifier {
                name: x.0,
                count: x.1 * 10,
            })
            .collect();
        ordered_identifiers.sort_by(|a, b| b.count.cmp(&a.count));

        Ok(Self {
            ordered_identifiers,
        })
    }

    fn identifiers_x_range(&self) -> Range<u32> {
        let end = u32::try_from(self.ordered_identifiers.len())
            .expect("number of identifiers should fit into u32");
        0..end - 1
    }

    fn identifier_x_label(&self, segment: &SegmentValue<u32>) -> String {
        let idx = match segment {
            SegmentValue::CenterOf(x) => *x,
            _ => panic!("segment should be SegmentValue::CenterOf"),
        };
        let idx = usize::try_from(idx).unwrap();
        self.ordered_identifiers[idx].name.clone()
    }

    fn identifier_max_count(&self) -> u64 {
        if self.ordered_identifiers.is_empty() {
            return 0;
        }
        // Ordered by descending count.
        self.ordered_identifiers[0].count
    }
}

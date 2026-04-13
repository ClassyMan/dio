use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Gauge};

use crate::app::App;
use crate::model::device::DeviceSeries;
use crate::model::ring_buffer::RingBuffer;
use crate::model::types::nice_ceil;
use crate::ui::theme;
use crate::ui::line_chart::{self, LineChart};

/// Everything needed to render a read/write time-series chart.
struct ChartSpec<'a> {
    title: &'a str,
    read_buf: &'a RingBuffer,
    write_buf: &'a RingBuffer,
    format_value: fn(f64) -> String,
    refresh_ms: f64,
}

pub fn render_all(frame: &mut Frame, area: Rect, app: &App) {
    if app.devices.is_empty() {
        let block = Block::default()
            .title(" No devices found ")
            .borders(Borders::ALL)
            .style(theme::border_style());
        frame.render_widget(block, area);
        return;
    }

    let refresh_ms = app.refresh_rate.as_millis() as f64;

    let constraints: Vec<Constraint> = app
        .devices
        .iter()
        .map(|_| Constraint::Ratio(1, app.devices.len() as u32))
        .collect();

    let areas = Layout::vertical(constraints).split(area);

    for (idx, device_area) in areas.iter().enumerate() {
        if let Some(device) = app.devices.get(idx) {
            render_device_compact(frame, *device_area, device, refresh_ms);
        }
    }
}

pub fn render_single(frame: &mut Frame, area: Rect, app: &App) {
    if let Some(device) = app.devices.get(app.selected_device) {
        let refresh_ms = app.refresh_rate.as_millis() as f64;
        render_device_full(frame, area, device, refresh_ms);
    }
}

fn render_device_compact(
    frame: &mut Frame,
    area: Rect,
    device: &DeviceSeries,
    refresh_ms: f64,
) {
    let [chart_area, stats_area] =
        Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
            .areas(area);

    render_chart(
        frame,
        chart_area,
        &ChartSpec {
            title: &device.name,
            read_buf: &device.read_iops,
            write_buf: &device.write_iops,
            format_value: crate::model::types::human_iops,
            refresh_ms,
        },
    );
    render_gauges(frame, stats_area, device);
}

fn render_device_full(
    frame: &mut Frame,
    area: Rect,
    device: &DeviceSeries,
    refresh_ms: f64,
) {
    let [iops_area, throughput_area, bottom_area] = Layout::vertical([
        Constraint::Percentage(35),
        Constraint::Percentage(35),
        Constraint::Percentage(30),
    ])
    .areas(area);

    render_chart(
        frame,
        iops_area,
        &ChartSpec {
            title: &format!("{} — IOPS (operations per second)", device.name),
            read_buf: &device.read_iops,
            write_buf: &device.write_iops,
            format_value: crate::model::types::human_iops,
            refresh_ms,
        },
    );
    render_chart(
        frame,
        throughput_area,
        &ChartSpec {
            title: "Throughput (data transferred per second)",
            read_buf: &device.read_throughput,
            write_buf: &device.write_throughput,
            format_value: crate::model::types::human_bytes,
            refresh_ms,
        },
    );

    let [latency_area, gauges_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(bottom_area);

    render_chart(
        frame,
        latency_area,
        &ChartSpec {
            title: "Latency (avg time per operation)",
            read_buf: &device.read_latency,
            write_buf: &device.write_latency,
            format_value: crate::model::types::human_latency,
            refresh_ms,
        },
    );
    render_gauges(frame, gauges_area, device);
}

fn render_chart(frame: &mut Frame, area: Rect, spec: &ChartSpec) {
    let mut read_data = Vec::new();
    let mut write_data = Vec::new();
    spec.read_buf.as_chart_data(&mut read_data);
    spec.write_buf.as_chart_data(&mut write_data);

    let y_max = nice_ceil(spec.read_buf.max().max(spec.write_buf.max()));
    let x_max = (spec.read_buf.capacity() as f64).max(1.0);
    let total_secs = spec.read_buf.capacity() as f64 * spec.refresh_ms / 1000.0;

    let current_read = spec.read_buf.latest().unwrap_or(0.0);
    let current_write = spec.write_buf.latest().unwrap_or(0.0);
    let fmt = spec.format_value;

    let datasets = vec![
        line_chart::Dataset {
            data: &read_data,
            color: theme::READ_COLOR,
            name: format!("read: {}", fmt(current_read)),
        },
        line_chart::Dataset {
            data: &write_data,
            color: theme::WRITE_COLOR,
            name: format!("write: {}", fmt(current_write)),
        },
    ];

    let chart = LineChart::new(datasets)
        .block(
            Block::default()
                .title(format!(" {} ", spec.title))
                .borders(Borders::ALL)
                .style(theme::border_style()),
        )
        .x_bounds([0.0, x_max])
        .y_bounds([0.0, y_max])
        .x_labels([
            format!("-{:.0}s", total_secs),
            "now".to_string(),
        ])
        .y_labels(["0".to_string(), fmt(y_max)]);

    frame.render_widget(chart, area);
}

fn render_gauges(frame: &mut Frame, area: Rect, device: &DeviceSeries) {
    let [queue_area, util_area] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);

    let queue = device.queue_depth.latest().unwrap_or(0.0);
    let queue_max = device.queue_depth.max().max(1.0);
    let queue_pct = (queue / queue_max * 100.0).min(100.0);

    let queue_gauge = Gauge::default()
        .block(
            Block::default()
                .title(" Queue Depth (pending I/O requests) ")
                .borders(Borders::ALL)
                .style(theme::border_style()),
        )
        .gauge_style(Style::default().fg(theme::severity_color(queue_pct)))
        .ratio((queue / queue_max).min(1.0))
        .label(format!("{:.0}", queue));

    frame.render_widget(queue_gauge, queue_area);

    let util_pct = device.utilization.latest().unwrap_or(0.0);
    let util_gauge = Gauge::default()
        .block(
            Block::default()
                .title(" Utilization (% time disk is busy) ")
                .borders(Borders::ALL)
                .style(theme::border_style()),
        )
        .gauge_style(
            Style::default()
                .fg(theme::severity_color(util_pct))
                .add_modifier(Modifier::BOLD),
        )
        .ratio((util_pct / 100.0).min(1.0))
        .label(format!("{:.0}%", util_pct));

    frame.render_widget(util_gauge, util_area);
}

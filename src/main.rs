use serde::Deserialize;
use slint::{Color, ModelRc, SharedString, VecModel};
use std::env;
use std::f32::consts::PI;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;

slint::include_modules!();

#[derive(Debug, Clone)]
struct KdeLayout {
    index: i32,
    label: String,
}

#[derive(Debug, Clone)]
struct AppConfig {
    theme: ThemeConfig,
    geometry: GeometryConfig,
    behavior: BehaviorConfig,
    text: TextConfig,
}

#[derive(Debug, Clone)]
struct ThemeConfig {
    panel_background: Color,
    panel_border: Color,
    heading_color: Color,
    hint_color: Color,
    sector_fill: Color,
    sector_active_fill: Color,
    sector_stroke: Color,
    sector_active_stroke: Color,
    hover_fill: Color,
    hover_pressed_fill: Color,
    hover_stroke: Color,
    label_background: Color,
    label_active_background: Color,
    label_border: Color,
    label_active_border: Color,
    label_hover_background: Color,
    label_text: Color,
    center_background: Color,
    center_border: Color,
    center_ring_fill: Color,
    center_ring_stroke: Color,
    dot_background: Color,
    dot_hover_background: Color,
}

#[derive(Debug, Clone)]
struct GeometryConfig {
    menu_size: f32,
    center_x: f32,
    center_y: f32,
    inner_radius: f32,
    outer_radius: f32,
    label_radius: f32,
    gap_radians: f32,
    center_ring_inner_radius: f32,
    center_ring_outer_radius: f32,
    center_circle_radius: f32,
    center_dot_radius: f32,
    label_width: f32,
    label_height: f32,
    label_font_size: f32,
    heading_y: f32,
    heading_font_size: f32,
    hint_y: f32,
    hint_font_size: f32,
    panel_radius: f32,
}

#[derive(Debug, Clone)]
struct BehaviorConfig {
    number_shortcuts: bool,
}

#[derive(Debug, Clone)]
struct TextConfig {
    window_title: String,
    heading: String,
    idle_hint: String,
    hover_hint: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct RawConfig {
    theme: RawThemeConfig,
    geometry: RawGeometryConfig,
    behavior: RawBehaviorConfig,
    text: RawTextConfig,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct RawThemeConfig {
    panel_background: Option<String>,
    panel_border: Option<String>,
    heading_color: Option<String>,
    hint_color: Option<String>,
    sector_fill: Option<String>,
    sector_active_fill: Option<String>,
    sector_stroke: Option<String>,
    sector_active_stroke: Option<String>,
    hover_fill: Option<String>,
    hover_pressed_fill: Option<String>,
    hover_stroke: Option<String>,
    label_background: Option<String>,
    label_active_background: Option<String>,
    label_border: Option<String>,
    label_active_border: Option<String>,
    label_hover_background: Option<String>,
    label_text: Option<String>,
    center_background: Option<String>,
    center_border: Option<String>,
    center_ring_fill: Option<String>,
    center_ring_stroke: Option<String>,
    dot_background: Option<String>,
    dot_hover_background: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct RawGeometryConfig {
    menu_size: Option<f32>,
    center_x: Option<f32>,
    center_y: Option<f32>,
    inner_radius: Option<f32>,
    outer_radius: Option<f32>,
    label_radius: Option<f32>,
    gap_degrees: Option<f32>,
    center_ring_inner_radius: Option<f32>,
    center_ring_outer_radius: Option<f32>,
    center_circle_radius: Option<f32>,
    center_dot_radius: Option<f32>,
    label_width: Option<f32>,
    label_height: Option<f32>,
    label_font_size: Option<f32>,
    heading_y: Option<f32>,
    heading_font_size: Option<f32>,
    hint_y: Option<f32>,
    hint_font_size: Option<f32>,
    panel_radius: Option<f32>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct RawBehaviorConfig {
    number_shortcuts: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct RawTextConfig {
    window_title: Option<String>,
    heading: Option<String>,
    idle_hint: Option<String>,
    hover_hint: Option<String>,
}

impl AppConfig {
    fn load() -> Self {
        let raw = read_config_file();

        let theme = ThemeConfig {
            panel_background: configured_color(&raw.theme.panel_background, "#061426dd"),
            panel_border: configured_color(&raw.theme.panel_border, "#1e4e74aa"),
            heading_color: configured_color(&raw.theme.heading_color, "#d8f3ff"),
            hint_color: configured_color(&raw.theme.hint_color, "#8fb8d4"),
            sector_fill: configured_color(&raw.theme.sector_fill, "#0b243dcc"),
            sector_active_fill: configured_color(&raw.theme.sector_active_fill, "#123f63cc"),
            sector_stroke: configured_color(&raw.theme.sector_stroke, "#245a7c99"),
            sector_active_stroke: configured_color(&raw.theme.sector_active_stroke, "#4fc6f5cc"),
            hover_fill: configured_color(&raw.theme.hover_fill, "#168fd0dd"),
            hover_pressed_fill: configured_color(&raw.theme.hover_pressed_fill, "#55cfffff"),
            hover_stroke: configured_color(&raw.theme.hover_stroke, "#a9efffff"),
            label_background: configured_color(&raw.theme.label_background, "#0d2236dd"),
            label_active_background: configured_color(&raw.theme.label_active_background, "#123f63cc"),
            label_border: configured_color(&raw.theme.label_border, "#4a9fc8aa"),
            label_active_border: configured_color(&raw.theme.label_active_border, "#8aeaffcc"),
            label_hover_background: configured_color(&raw.theme.label_hover_background, "#1e9edddd"),
            label_text: configured_color(&raw.theme.label_text, "#dff7ff"),
            center_background: configured_color(&raw.theme.center_background, "#07111fcc"),
            center_border: configured_color(&raw.theme.center_border, "#28506fdd"),
            center_ring_fill: configured_color(&raw.theme.center_ring_fill, "#9eefffff"),
            center_ring_stroke: configured_color(&raw.theme.center_ring_stroke, "#dff8ffff"),
            dot_background: configured_color(&raw.theme.dot_background, "#426985aa"),
            dot_hover_background: configured_color(&raw.theme.dot_hover_background, "#b9f1ffff"),
        };

        let geometry = GeometryConfig {
            menu_size: raw.geometry.menu_size.unwrap_or(420.0),
            center_x: raw.geometry.center_x.unwrap_or(210.0),
            center_y: raw.geometry.center_y.unwrap_or(210.0),
            inner_radius: raw.geometry.inner_radius.unwrap_or(72.0),
            outer_radius: raw.geometry.outer_radius.unwrap_or(170.0),
            label_radius: raw.geometry.label_radius.unwrap_or(122.0),
            gap_radians: raw.geometry.gap_degrees.unwrap_or(0.0).to_radians(),
            center_ring_inner_radius: raw.geometry.center_ring_inner_radius.unwrap_or(31.0),
            center_ring_outer_radius: raw.geometry.center_ring_outer_radius.unwrap_or(40.0),
            center_circle_radius: raw.geometry.center_circle_radius.unwrap_or(30.0),
            center_dot_radius: raw.geometry.center_dot_radius.unwrap_or(8.0),
            label_width: raw.geometry.label_width.unwrap_or(84.0),
            label_height: raw.geometry.label_height.unwrap_or(38.0),
            label_font_size: raw.geometry.label_font_size.unwrap_or(17.0),
            heading_y: raw.geometry.heading_y.unwrap_or(30.0),
            heading_font_size: raw.geometry.heading_font_size.unwrap_or(20.0),
            hint_y: raw.geometry.hint_y.unwrap_or(58.0),
            hint_font_size: raw.geometry.hint_font_size.unwrap_or(12.0),
            panel_radius: raw.geometry.panel_radius.unwrap_or(34.0),
        };

        let behavior = BehaviorConfig {
            number_shortcuts: raw.behavior.number_shortcuts.unwrap_or(true),
        };

        let text = TextConfig {
            window_title: raw
                .text
                .window_title
                .unwrap_or_else(|| "Layout Pie".to_string()),
            heading: raw.text.heading.unwrap_or_default(),
            idle_hint: raw
                .text
                .idle_hint
                .unwrap_or_else(|| "Press 1-9/0 or hover a sector".to_string()),
            hover_hint: raw
                .text
                .hover_hint
                .unwrap_or_else(|| "Click or press number to select".to_string()),
        };

        Self {
            theme,
            geometry,
            behavior,
            text,
        }
    }
}

fn read_config_file() -> RawConfig {
    for path in config_paths() {
        let Ok(contents) = fs::read_to_string(&path) else {
            continue;
        };

        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        let parsed = if matches!(extension.as_str(), "yaml" | "yml") {
            serde_yaml::from_str::<RawConfig>(&contents)
                .map_err(|err| format!("YAML parse error: {err}"))
        } else {
            toml::from_str::<RawConfig>(&contents)
                .map_err(|err| format!("TOML parse error: {err}"))
        };

        match parsed {
            Ok(config) => {
                eprintln!("Loaded config from {}", path.display());
                return config;
            }
            Err(err) => eprintln!("Failed to parse {}: {err}", path.display()),
        }
    }
    eprintln!("No config file found; using built-in defaults");
    RawConfig::default()
}

fn config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(path) = env::var("LAYOUT_PIE_CONFIG") {
        paths.push(PathBuf::from(path));
    }

    let config_home = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| env::var("HOME").map(|home| PathBuf::from(home).join(".config")));

    if let Ok(config_home) = config_home {
        let app_dir = config_home.join("layout-pie");
        paths.push(app_dir.join("config.toml"));
        paths.push(app_dir.join("config.yaml"));
        paths.push(app_dir.join("config.yml"));
        paths.push(app_dir.join("config.yoml"));
    }

    paths
}

fn configured_color(value: &Option<String>, fallback: &str) -> Color {
    let fallback_color = color_from_hex(fallback, Color::from_rgb_u8(0, 0, 0));
    value
        .as_deref()
        .map(|value| color_from_hex(value, fallback_color))
        .unwrap_or(fallback_color)
}

fn color_from_hex(value: &str, fallback: Color) -> Color {
    let hex = value.trim().trim_start_matches('#');

    let parse_pair = |start: usize| u8::from_str_radix(&hex[start..start + 2], 16).ok();

    match hex.len() {
        6 => match (parse_pair(0), parse_pair(2), parse_pair(4)) {
            (Some(r), Some(g), Some(b)) => Color::from_rgb_u8(r, g, b),
            _ => fallback,
        },
        8 => match (parse_pair(0), parse_pair(2), parse_pair(4), parse_pair(6)) {
            (Some(r), Some(g), Some(b), Some(a)) => Color::from_argb_u8(a, r, g, b),
            _ => fallback,
        },
        _ => fallback,
    }
}

fn read_kde_layouts() -> Vec<KdeLayout> {
    let home = match env::var("HOME") {
        Ok(home) => home,
        Err(_) => return fallback_layouts(),
    };

    let path = format!("{home}/.config/kxkbrc");

    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => return fallback_layouts(),
    };

    let mut layout_list: Vec<String> = Vec::new();
    let mut display_names: Vec<String> = Vec::new();

    for line in contents.lines() {
        let line = line.trim();

        if let Some(value) = line.strip_prefix("LayoutList=") {
            layout_list = value
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        if let Some(value) = line.strip_prefix("DisplayNames=") {
            display_names = value
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }
    }

    if layout_list.is_empty() {
        return fallback_layouts();
    }

    layout_list
        .iter()
        .enumerate()
        .map(|(index, code)| {
            let display = display_names
                .get(index)
                .filter(|name| !name.is_empty())
                .cloned()
                .unwrap_or_else(|| code.to_uppercase());

            KdeLayout {
                index: index as i32,
                label: display,
            }
        })
        .collect()
}

fn fallback_layouts() -> Vec<KdeLayout> {
    vec![KdeLayout {
        index: 0,
        label: "EN".to_string(),
    }]
}

fn get_current_layout() -> Option<i32> {
    let output = Command::new("qdbus6")
        .arg("org.kde.keyboard")
        .arg("/Layouts")
        .arg("getLayout")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout);
    text.trim().parse::<i32>().ok()
}

fn point_on_circle(cx: f32, cy: f32, radius: f32, angle: f32) -> (f32, f32) {
    (cx + radius * angle.cos(), cy + radius * angle.sin())
}

fn annular_sector_path(
    center_x: f32,
    center_y: f32,
    inner_radius: f32,
    outer_radius: f32,
    start: f32,
    end: f32,
) -> String {
    let (outer_start_x, outer_start_y) = point_on_circle(center_x, center_y, outer_radius, start);
    let (outer_end_x, outer_end_y) = point_on_circle(center_x, center_y, outer_radius, end);
    let (inner_end_x, inner_end_y) = point_on_circle(center_x, center_y, inner_radius, end);
    let (inner_start_x, inner_start_y) = point_on_circle(center_x, center_y, inner_radius, start);

    let delta = end - start;
    let large_arc = if delta.abs() > PI { 1 } else { 0 };

    format!(
        "M {outer_start_x:.2} {outer_start_y:.2} \
         A {outer_r:.2} {outer_r:.2} 0 {large_arc} 1 {outer_end_x:.2} {outer_end_y:.2} \
         L {inner_end_x:.2} {inner_end_y:.2} \
         A {inner_r:.2} {inner_r:.2} 0 {large_arc} 0 {inner_start_x:.2} {inner_start_y:.2} \
         Z",
        outer_r = outer_radius,
        inner_r = inner_radius,
    )
}

fn sector_path(start: f32, end: f32, geometry: &GeometryConfig) -> String {
    annular_sector_path(
        geometry.center_x,
        geometry.center_y,
        geometry.inner_radius,
        geometry.outer_radius,
        start,
        end,
    )
}

fn center_ring_path(start: f32, end: f32, geometry: &GeometryConfig) -> String {
    annular_sector_path(
        geometry.center_x,
        geometry.center_y,
        geometry.center_ring_inner_radius,
        geometry.center_ring_outer_radius,
        start,
        end,
    )
}

fn make_pie_items(
    layouts: Vec<KdeLayout>,
    current_layout: Option<i32>,
    geometry: &GeometryConfig,
) -> Vec<LayoutItem> {
    let count = layouts.len().max(1) as f32;
    let slice = 2.0 * PI / count;

    layouts
        .into_iter()
        .enumerate()
        .map(|(i, layout)| {
            let center_angle = -PI / 2.0 + slice * i as f32;
            let start = center_angle - slice / 2.0 + geometry.gap_radians;
            let end = center_angle + slice / 2.0 - geometry.gap_radians;
            let (lx, ly) = point_on_circle(
                geometry.center_x,
                geometry.center_y,
                geometry.label_radius,
                center_angle,
            );

            LayoutItem {
                index: layout.index,
                label: SharedString::from(layout.label),
                shortcut: SharedString::from(shortcut_label(i)),
                path: SharedString::from(sector_path(start, end, geometry)),
                center_path: SharedString::from(center_ring_path(start, end, geometry)),
                lx,
                ly,
                active: current_layout == Some(layout.index),
            }
        })
        .collect()
}

fn shortcut_label(position: usize) -> String {
    match position {
        0..=8 => (position + 1).to_string(),
        9 => "0".to_string(),
        _ => String::new(),
    }
}

fn shortcut_position(key: &str) -> Option<usize> {
    match key {
        // ASCII / English digits
        "1" | "۱" | "١" => Some(0),
        "2" | "۲" | "٢" => Some(1),
        "3" | "۳" | "٣" => Some(2),
        "4" | "۴" | "٤" => Some(3),
        "5" | "۵" | "٥" => Some(4),
        "6" | "۶" | "٦" => Some(5),
        "7" | "۷" | "٧" => Some(6),
        "8" | "۸" | "٨" => Some(7),
        "9" | "۹" | "٩" => Some(8),
        "0" | "۰" | "٠" => Some(9),

        _ => None,
    }
}

fn sector_at_point(x: f32, y: f32, layout_count: usize, geometry: &GeometryConfig) -> i32 {
    if layout_count == 0 {
        return -1;
    }

    let dx = x - geometry.center_x;
    let dy = y - geometry.center_y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance < geometry.inner_radius || distance > geometry.outer_radius {
        return -1;
    }

    let tau = 2.0 * PI;
    let slice = tau / layout_count as f32;

    // atan2 gives right = 0, down = PI / 2, left = PI, up = -PI / 2.
    // Normalize so up = 0 and numbers proceed clockwise around the pie.
    let mut angle = dy.atan2(dx) + PI / 2.0;

    while angle < 0.0 {
        angle += tau;
    }

    while angle >= tau {
        angle -= tau;
    }

    let index = ((angle + slice / 2.0) / slice).floor() as usize % layout_count;
    let sector_center = index as f32 * slice;

    let mut diff = (angle - sector_center).abs();
    diff = diff.min(tau - diff);

    if diff > slice / 2.0 - geometry.gap_radians {
        return -1;
    }

    index as i32
}

fn switch_layout(index: i32) {
    let status = Command::new("qdbus6")
        .arg("org.kde.keyboard")
        .arg("/Layouts")
        .arg("setLayout")
        .arg(index.to_string())
        .status();

    match status {
        Ok(status) if status.success() => {}
        Ok(status) => eprintln!("qdbus6 failed with status: {status}"),
        Err(err) => eprintln!("Failed to run qdbus6: {err}"),
    }
}

fn select_layout_by_position(position: i32, layout_indices: &[i32]) -> bool {
    if position < 0 {
        return false;
    }

    let Some(&layout_index) = layout_indices.get(position as usize) else {
        return false;
    };

    switch_layout(layout_index);
    true
}

fn apply_config(app: &PieWindow, config: &AppConfig) {
    app.set_menu_size(config.geometry.menu_size);
    app.set_center_x(config.geometry.center_x);
    app.set_center_y(config.geometry.center_y);
    app.set_center_circle_radius(config.geometry.center_circle_radius);
    app.set_center_dot_radius(config.geometry.center_dot_radius);
    app.set_label_width(config.geometry.label_width);
    app.set_label_height(config.geometry.label_height);
    app.set_label_font_size(config.geometry.label_font_size);
    app.set_heading_y(config.geometry.heading_y);
    app.set_heading_font_size(config.geometry.heading_font_size);
    app.set_hint_y(config.geometry.hint_y);
    app.set_hint_font_size(config.geometry.hint_font_size);
    app.set_panel_radius(config.geometry.panel_radius);

    app.set_number_shortcuts(config.behavior.number_shortcuts);
    app.set_window_title(SharedString::from(config.text.window_title.clone()));
    app.set_heading_text(SharedString::from(config.text.heading.clone()));
    app.set_idle_hint_text(SharedString::from(config.text.idle_hint.clone()));
    app.set_hover_hint_text(SharedString::from(config.text.hover_hint.clone()));

    app.set_panel_background(config.theme.panel_background);
    app.set_panel_border(config.theme.panel_border);
    app.set_heading_color(config.theme.heading_color);
    app.set_hint_color(config.theme.hint_color);
    app.set_sector_fill(config.theme.sector_fill);
    app.set_sector_active_fill(config.theme.sector_active_fill);
    app.set_sector_stroke(config.theme.sector_stroke);
    app.set_sector_active_stroke(config.theme.sector_active_stroke);
    app.set_hover_fill(config.theme.hover_fill);
    app.set_hover_pressed_fill(config.theme.hover_pressed_fill);
    app.set_hover_stroke(config.theme.hover_stroke);
    app.set_label_background(config.theme.label_background);
    app.set_label_active_background(config.theme.label_active_background);
    app.set_label_border(config.theme.label_border);
    app.set_label_active_border(config.theme.label_active_border);
    app.set_label_hover_background(config.theme.label_hover_background);
    app.set_label_text(config.theme.label_text);
    app.set_center_background(config.theme.center_background);
    app.set_center_border(config.theme.center_border);
    app.set_center_ring_fill(config.theme.center_ring_fill);
    app.set_center_ring_stroke(config.theme.center_ring_stroke);
    app.set_dot_background(config.theme.dot_background);
    app.set_dot_hover_background(config.theme.dot_hover_background);
}

fn main() -> Result<(), slint::PlatformError> {
    let config = AppConfig::load();
    let app = PieWindow::new()?;

    apply_config(&app, &config);

    let layouts = read_kde_layouts();
    let layout_indices: Vec<i32> = layouts.iter().map(|layout| layout.index).collect();
    let layout_count = layouts.len();

    let current_layout = get_current_layout();
    let items = make_pie_items(layouts, current_layout, &config.geometry);

    let model = Rc::new(VecModel::from(items));
    app.set_layouts(ModelRc::from(model));

    let hit_geometry = config.geometry.clone();
    app.on_sector_at(move |x, y| sector_at_point(x, y, layout_count, &hit_geometry));

    let click_layout_indices = layout_indices.clone();
    app.on_selected(move |position| {
        if select_layout_by_position(position, &click_layout_indices) {
            std::process::exit(0);
        }
    });

    let shortcut_layout_indices = layout_indices.clone();
    app.on_shortcut_key(move |key| {
        let Some(position) = shortcut_position(key.as_str()) else {
            return false;
        };

        if select_layout_by_position(position as i32, &shortcut_layout_indices) {
            std::process::exit(0);
        }

        false
    });

    app.on_canceled(|| {
        std::process::exit(0);
    });

    app.run()
}

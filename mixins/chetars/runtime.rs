#[derive(Clone, Copy, Debug)]
pub struct Panel {
    pub shadow: [[f32; 3]; 4],
    pub body: [[f32; 3]; 4],
    pub inner: [[f32; 3]; 4],
    pub header: [[f32; 3]; 4],
    pub buttons: [[[f32; 3]; 4]; 3],
    pub accent_left: [[f32; 3]; 4],
    pub accent_top: [[f32; 3]; 4],
}

static SELECTED: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);
static VIEW_WIDTH: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(854.0f32.to_bits());
static VIEW_HEIGHT: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(480.0f32.to_bits());
static POINTER_X: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(427.0f32.to_bits());
static POINTER_Y: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(240.0f32.to_bits());

pub fn set_viewport(width: f32, height: f32, mouse: [f32; 2]) {
    use std::sync::atomic::Ordering;

    if width.is_finite() && width > 0.0 {
        VIEW_WIDTH.store(width.to_bits(), Ordering::Relaxed);
    }
    if height.is_finite() && height > 0.0 {
        VIEW_HEIGHT.store(height.to_bits(), Ordering::Relaxed);
    }
    if mouse[0].is_finite() {
        POINTER_X.store(mouse[0].to_bits(), Ordering::Relaxed);
    }
    if mouse[1].is_finite() {
        POINTER_Y.store(mouse[1].to_bits(), Ordering::Relaxed);
    }
}

pub fn viewport() -> (f32, f32, [f32; 2]) {
    use std::sync::atomic::Ordering;

    (
        f32::from_bits(VIEW_WIDTH.load(Ordering::Relaxed)),
        f32::from_bits(VIEW_HEIGHT.load(Ordering::Relaxed)),
        [
            f32::from_bits(POINTER_X.load(Ordering::Relaxed)),
            f32::from_bits(POINTER_Y.load(Ordering::Relaxed)),
        ],
    )
}

pub fn selected_button() -> usize {
    SELECTED.load(std::sync::atomic::Ordering::Relaxed) as usize
}

pub fn select_button(index: usize) {
    if index < 3 {
        SELECTED.store(index as u8, std::sync::atomic::Ordering::Relaxed);
    }
}

fn skew_point(
    x: f32,
    y: f32,
    cx: f32,
    cy: f32,
    tilt_x: f32,
    tilt_y: f32,
) -> [f32; 3] {
    let nx = (x - cx) / cx.max(1.0);
    let ny = (y - cy) / cy.max(1.0);
    [x + ny * tilt_x * 22.0, y + nx * tilt_y * 16.0, 0.0]
}

fn quad(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    cx: f32,
    cy: f32,
    tilt_x: f32,
    tilt_y: f32,
    z: f32,
) -> [[f32; 3]; 4] {
    let mut p = [
        skew_point(x, y, cx, cy, tilt_x, tilt_y),
        skew_point(x + w, y, cx, cy, tilt_x, tilt_y),
        skew_point(x + w, y + h, cx, cy, tilt_x, tilt_y),
        skew_point(x, y + h, cx, cy, tilt_x, tilt_y),
    ];
    for point in &mut p {
        point[2] = z;
    }
    p
}

pub fn panel(mouse: [f32; 2], width: f32, height: f32) -> Panel {
    let cx = width * 0.5;
    let cy = height * 0.5;
    let _ = mouse;
    let tilt_x = 0.0;
    let tilt_y = 0.0;

    let w = (width * 0.58).clamp(360.0, 620.0);
    let h = (height * 0.58).clamp(260.0, 420.0);
    let x = cx - w * 0.5;
    let y = cy - h * 0.5;

    let shadow = quad(x + 12.0, y + 16.0, w, h, cx, cy, tilt_x, tilt_y, -10.0);
    let body = quad(x, y, w, h, cx, cy, tilt_x, tilt_y, 0.0);
    let inner = quad(x + 7.0, y + 7.0, w - 14.0, h - 14.0, cx, cy, tilt_x, tilt_y, 2.0);
    let header = quad(x + 18.0, y + 18.0, w - 36.0, 54.0, cx, cy, tilt_x, tilt_y, 4.0);

    let bw = (w - 72.0) / 3.0;
    let buttons = [
        quad(x + 24.0, y + 100.0, bw, 46.0, cx, cy, tilt_x, tilt_y, 6.0),
        quad(x + 24.0 + bw + 12.0, y + 100.0, bw, 46.0, cx, cy, tilt_x, tilt_y, 6.0),
        quad(x + 24.0 + (bw + 12.0) * 2.0, y + 100.0, bw, 46.0, cx, cy, tilt_x, tilt_y, 6.0),
    ];

    let accent_left = quad(x, y, 3.0, h, cx, cy, tilt_x, tilt_y, 8.0);
    let accent_top = quad(x, y, w, 3.0, cx, cy, tilt_x, tilt_y, 8.0);

    Panel {
        shadow,
        body,
        inner,
        header,
        buttons,
        accent_left,
        accent_top,
    }
}

fn point_in_quad(point: [f32; 2], quad: [[f32; 3]; 4]) -> bool {
    let min_x = quad.iter().map(|p| p[0]).fold(f32::INFINITY, f32::min);
    let max_x = quad.iter().map(|p| p[0]).fold(f32::NEG_INFINITY, f32::max);
    let min_y = quad.iter().map(|p| p[1]).fold(f32::INFINITY, f32::min);
    let max_y = quad.iter().map(|p| p[1]).fold(f32::NEG_INFINITY, f32::max);

    point[0] >= min_x && point[0] <= max_x && point[1] >= min_y && point[1] <= max_y
}

pub fn button_at(mouse: [f32; 2], width: f32, height: f32) -> usize {
    let p = panel(mouse, width, height);
    for i in 0..3 {
        if point_in_quad(mouse, p.buttons[i]) {
            return i;
        }
    }
    0
}

pub fn button_hit(mouse: [f32; 2], width: f32, height: f32, index: usize) -> bool {
    if index >= 3 {
        return false;
    }
    let p = panel(mouse, width, height);
    point_in_quad(mouse, p.buttons[index])
}

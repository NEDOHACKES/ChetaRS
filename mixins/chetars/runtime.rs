#[derive(Clone, Copy, Debug)]
pub struct Panel {
    pub shadow: [[f32; 3]; 4],
    pub body: [[f32; 3]; 4],
    pub inner: [[f32; 3]; 4],
    pub header: [[f32; 3]; 4],
    pub sidebar: [[f32; 3]; 4],
    pub content_frame: [[f32; 3]; 4],
    pub white_area: [[f32; 3]; 4],
    pub buttons: [[[f32; 3]; 4]; 3],
    pub accent_left: [[f32; 3]; 4],
    pub accent_top: [[f32; 3]; 4],
}

static SELECTED: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);
static OPEN: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

pub fn is_open() -> bool {
    OPEN.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn toggle_open() -> bool {
    let next = !is_open();
    OPEN.store(next, std::sync::atomic::Ordering::Relaxed);
    next
}

pub fn close() {
    OPEN.store(false, std::sync::atomic::Ordering::Relaxed);
}

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

fn quad(x: f32, y: f32, w: f32, h: f32, z: f32) -> [[f32; 3]; 4] {
    [
        [x, y, z],
        [x + w, y, z],
        [x + w, y + h, z],
        [x, y + h, z],
    ]
}

pub fn panel(_mouse: [f32; 2], width: f32, height: f32) -> Panel {
    let cx = width * 0.5;
    let cy = height * 0.5;

    // A wide spatial display, inspired by Spatial-GUI's floating screen.
    let w = (width * 0.76).clamp(500.0, 980.0);
    let h = (height * 0.72).clamp(290.0, 580.0);
    let x = cx - w * 0.5;
    let y = cy - h * 0.5;

    let shadow = quad(x + 12.0, y + 16.0, w, h, -16.0);
    let body = quad(x, y, w, h, 0.0);
    let inner = quad(x + 6.0, y + 6.0, w - 12.0, h - 12.0, 2.0);
    let header = quad(x + 12.0, y + 12.0, w - 24.0, 48.0, 4.0);

    let sidebar_x = x + 14.0;
    let content_x = x + 170.0;
    let section_y = y + 72.0;
    let section_h = h - 86.0;

    let sidebar = quad(sidebar_x, section_y, 144.0, section_h, 5.0);
    let content_frame = quad(
        content_x,
        section_y,
        w - 184.0,
        section_h,
        6.0,
    );
    // This is deliberately an empty white canvas. No inventory texture,
    // item slots, item stacks or inventory interaction are drawn here.
    let white_area = quad(
        content_x + 6.0,
        section_y + 6.0,
        w - 196.0,
        section_h - 12.0,
        8.0,
    );

    let button_x = sidebar_x + 10.0;
    let button_w = 124.0;
    let button_h = 38.0;
    let button_gap = 10.0;
    let button_y = section_y + 16.0;
    let buttons = [
        quad(button_x, button_y, button_w, button_h, 9.0),
        quad(
            button_x,
            button_y + button_h + button_gap,
            button_w,
            button_h,
            9.0,
        ),
        quad(
            button_x,
            button_y + (button_h + button_gap) * 2.0,
            button_w,
            button_h,
            9.0,
        ),
    ];

    let accent_left = quad(x, y, 3.0, h, 11.0);
    let accent_top = quad(x, y, w, 3.0, 11.0);

    Panel {
        shadow,
        body,
        inner,
        header,
        sidebar,
        content_frame,
        white_area,
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

pub fn panel_hit(mouse: [f32; 2], width: f32, height: f32) -> bool {
    point_in_quad(mouse, panel(mouse, width, height).body)
}

pub fn button_at(mouse: [f32; 2], width: f32, height: f32) -> Option<usize> {
    let p = panel(mouse, width, height);
    (0..3).find(|&i| point_in_quad(mouse, p.buttons[i]))
}

pub fn button_hit(mouse: [f32; 2], width: f32, height: f32, index: usize) -> bool {
    if index >= 3 {
        return false;
    }
    let p = panel(mouse, width, height);
    point_in_quad(mouse, p.buttons[index])
}

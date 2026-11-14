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

use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicU64, Ordering};

static SELECTED: AtomicU8 = AtomicU8::new(0);
static SETTINGS_SELECTED: AtomicU8 = AtomicU8::new(0);
static OPEN: AtomicBool = AtomicBool::new(false);

// The default mode is pinned to a world position. Follow mode is opt-in.
static FOLLOW_MODE: AtomicBool = AtomicBool::new(false);
static DRAGGING: AtomicBool = AtomicBool::new(false);
static ANCHOR_READY: AtomicBool = AtomicBool::new(false);
static LAST_VIEW_READY: AtomicBool = AtomicBool::new(false);
static HOVERED_SETTING: AtomicU8 = AtomicU8::new(u8::MAX);

// World-space anchor position (f64 keeps precision at large coordinates).
static ANCHOR_X: AtomicU64 = AtomicU64::new(0.0f64.to_bits());
static ANCHOR_Y: AtomicU64 = AtomicU64::new(0.0f64.to_bits());
static ANCHOR_Z: AtomicU64 = AtomicU64::new(0.0f64.to_bits());
// Frozen orientation, in quaternion x/y/z/w order.
static ANCHOR_QX: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static ANCHOR_QY: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static ANCHOR_QZ: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static ANCHOR_QW: AtomicU32 = AtomicU32::new(1.0f32.to_bits());

// Follow mode has a draggable offset in the camera's local right/up axes.
static FOLLOW_OFFSET_X: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static FOLLOW_OFFSET_Y: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static LAST_YAW: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static LAST_PITCH: AtomicU32 = AtomicU32::new(0.0f32.to_bits());

// Animated switch progress: 0 = off, 1 = on.
static PIN_PROGRESS: AtomicU32 = AtomicU32::new(1.0f32.to_bits());
static FOLLOW_PROGRESS: AtomicU32 = AtomicU32::new(0.0f32.to_bits());

pub fn is_open() -> bool {
    OPEN.load(Ordering::Relaxed)
}

pub fn toggle_open() -> bool {
    let next = !is_open();
    OPEN.store(next, Ordering::Relaxed);
    if !next {
        DRAGGING.store(false, Ordering::Relaxed);
        HOVERED_SETTING.store(u8::MAX, Ordering::Relaxed);
        LAST_VIEW_READY.store(false, Ordering::Relaxed);
    }
    next
}

pub fn close() {
    OPEN.store(false, Ordering::Relaxed);
    DRAGGING.store(false, Ordering::Relaxed);
    HOVERED_SETTING.store(u8::MAX, Ordering::Relaxed);
    LAST_VIEW_READY.store(false, Ordering::Relaxed);
}

pub fn follow_mode() -> bool {
    FOLLOW_MODE.load(Ordering::Relaxed)
}

pub fn set_follow_mode(follow: bool) {
    let changed = FOLLOW_MODE.swap(follow, Ordering::Relaxed) != follow;
    if changed {
        // Switching modes creates a fresh anchor/offset at the current view.
        ANCHOR_READY.store(false, Ordering::Relaxed);
        FOLLOW_OFFSET_X.store(0.0f32.to_bits(), Ordering::Relaxed);
        FOLLOW_OFFSET_Y.store(0.0f32.to_bits(), Ordering::Relaxed);
        LAST_VIEW_READY.store(false, Ordering::Relaxed);
        DRAGGING.store(false, Ordering::Relaxed);
    }
    SETTINGS_SELECTED.store(if follow { 1 } else { 0 }, Ordering::Relaxed);
}

pub fn selected_setting() -> usize {
    (SETTINGS_SELECTED.load(Ordering::Relaxed) as usize).min(1)
}

pub fn select_setting(index: usize) {
    if index < 2 {
        SETTINGS_SELECTED.store(index as u8, Ordering::Relaxed);
    }
}

pub fn toggle_selected_setting() {
    set_follow_mode(selected_setting() == 0);
}

fn step_progress(current: &AtomicU32, target: f32) -> f32 {
    let value = f32::from_bits(current.load(Ordering::Relaxed));
    let mut next = value + (target - value) * 0.22;
    if (target - next).abs() < 0.002 {
        next = target;
    }
    current.store(next.to_bits(), Ordering::Relaxed);
    next
}

pub fn advance_switch_animations() -> [f32; 2] {
    let follow = follow_mode();
    [
        step_progress(&PIN_PROGRESS, if follow { 0.0 } else { 1.0 }),
        step_progress(&FOLLOW_PROGRESS, if follow { 1.0 } else { 0.0 }),
    ]
}

pub fn set_dragging(dragging: bool) {
    DRAGGING.store(dragging, Ordering::Relaxed);
    LAST_VIEW_READY.store(false, Ordering::Relaxed);
}

pub fn set_hovered_setting(index: Option<usize>) {
    HOVERED_SETTING.store(index.map_or(u8::MAX, |i| i.min(1) as u8), Ordering::Relaxed);
}

pub fn hovered_setting() -> Option<usize> {
    let value = HOVERED_SETTING.load(Ordering::Relaxed);
    (value < 2).then_some(value as usize)
}

pub fn set_viewport(width: f32, height: f32, mouse: [f32; 2]) {
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

static VIEW_WIDTH: AtomicU32 = AtomicU32::new(854.0f32.to_bits());
static VIEW_HEIGHT: AtomicU32 = AtomicU32::new(480.0f32.to_bits());
static POINTER_X: AtomicU32 = AtomicU32::new(427.0f32.to_bits());
static POINTER_Y: AtomicU32 = AtomicU32::new(240.0f32.to_bits());

pub fn viewport() -> (f32, f32, [f32; 2]) {
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
    SELECTED.load(Ordering::Relaxed) as usize
}

pub fn select_button(index: usize) {
    if index < 3 {
        SELECTED.store(index as u8, Ordering::Relaxed);
    }
}

pub fn save_anchor(position: [f64; 3], rotation: glam::Quat) {
    ANCHOR_X.store(position[0].to_bits(), Ordering::Relaxed);
    ANCHOR_Y.store(position[1].to_bits(), Ordering::Relaxed);
    ANCHOR_Z.store(position[2].to_bits(), Ordering::Relaxed);
    let q = rotation.to_array();
    ANCHOR_QX.store(q[0].to_bits(), Ordering::Relaxed);
    ANCHOR_QY.store(q[1].to_bits(), Ordering::Relaxed);
    ANCHOR_QZ.store(q[2].to_bits(), Ordering::Relaxed);
    ANCHOR_QW.store(q[3].to_bits(), Ordering::Relaxed);
    ANCHOR_READY.store(true, Ordering::Relaxed);
}

pub fn anchor_ready() -> bool {
    ANCHOR_READY.load(Ordering::Relaxed)
}

pub fn anchor() -> ([f64; 3], glam::Quat) {
    let position = [
        f64::from_bits(ANCHOR_X.load(Ordering::Relaxed)),
        f64::from_bits(ANCHOR_Y.load(Ordering::Relaxed)),
        f64::from_bits(ANCHOR_Z.load(Ordering::Relaxed)),
    ];
    let rotation = glam::Quat::from_array([
        f32::from_bits(ANCHOR_QX.load(Ordering::Relaxed)),
        f32::from_bits(ANCHOR_QY.load(Ordering::Relaxed)),
        f32::from_bits(ANCHOR_QZ.load(Ordering::Relaxed)),
        f32::from_bits(ANCHOR_QW.load(Ordering::Relaxed)),
    ]);
    (position, rotation)
}

pub fn move_anchor(offset: [f64; 3]) {
    let x = f64::from_bits(ANCHOR_X.load(Ordering::Relaxed)) + offset[0];
    let y = f64::from_bits(ANCHOR_Y.load(Ordering::Relaxed)) + offset[1];
    let z = f64::from_bits(ANCHOR_Z.load(Ordering::Relaxed)) + offset[2];
    ANCHOR_X.store(x.to_bits(), Ordering::Relaxed);
    ANCHOR_Y.store(y.to_bits(), Ordering::Relaxed);
    ANCHOR_Z.store(z.to_bits(), Ordering::Relaxed);
}

pub fn follow_offset() -> [f32; 2] {
    [
        f32::from_bits(FOLLOW_OFFSET_X.load(Ordering::Relaxed)),
        f32::from_bits(FOLLOW_OFFSET_Y.load(Ordering::Relaxed)),
    ]
}

pub fn move_follow_offset(x: f32, y: f32) {
    let next_x = f32::from_bits(FOLLOW_OFFSET_X.load(Ordering::Relaxed)) + x;
    let next_y = f32::from_bits(FOLLOW_OFFSET_Y.load(Ordering::Relaxed)) + y;
    FOLLOW_OFFSET_X.store(next_x.clamp(-3.0, 3.0).to_bits(), Ordering::Relaxed);
    FOLLOW_OFFSET_Y.store(next_y.clamp(-2.0, 2.0).to_bits(), Ordering::Relaxed);
}

pub fn update_drag_view(yaw: f32, pitch: f32, distance: f32) {
    if LAST_VIEW_READY.load(Ordering::Relaxed) {
        let last_yaw = f32::from_bits(LAST_YAW.load(Ordering::Relaxed));
        let last_pitch = f32::from_bits(LAST_PITCH.load(Ordering::Relaxed));
        if is_dragging() {
            let mut yaw_delta = yaw - last_yaw;
            if yaw_delta > std::f32::consts::PI {
                yaw_delta -= std::f32::consts::TAU;
            } else if yaw_delta < -std::f32::consts::PI {
                yaw_delta += std::f32::consts::TAU;
            }
            let pitch_delta = pitch - last_pitch;
            let speed = distance * 1.35;
            if follow_mode() {
                move_follow_offset(yaw_delta * speed, -pitch_delta * speed);
            } else {
                let (_, rotation) = anchor();
                let right = rotation * glam::Vec3::X;
                let up = rotation * glam::Vec3::Y;
                let shift = right * (yaw_delta * speed) + up * (-pitch_delta * speed);
                move_anchor([
                    shift.x as f64,
                    shift.y as f64,
                    shift.z as f64,
                ]);
            }
        }
    }
    LAST_YAW.store(yaw.to_bits(), Ordering::Relaxed);
    LAST_PITCH.store(pitch.to_bits(), Ordering::Relaxed);
    LAST_VIEW_READY.store(true, Ordering::Relaxed);
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
    let content_frame = quad(content_x, section_y, w - 184.0, section_h, 6.0);
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
        quad(button_x, button_y + button_h + button_gap, button_w, button_h, 9.0),
        quad(button_x, button_y + (button_h + button_gap) * 2.0, button_w, button_h, 9.0),
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


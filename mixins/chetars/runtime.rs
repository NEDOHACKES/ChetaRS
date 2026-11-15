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
static FOLLOW_MODE_CHANGED: AtomicBool = AtomicBool::new(false);
static DRAGGING: AtomicBool = AtomicBool::new(false);
static DRAG_HOVERED: AtomicBool = AtomicBool::new(false);
static ANCHOR_READY: AtomicBool = AtomicBool::new(false);
static LAST_VIEW_READY: AtomicBool = AtomicBool::new(false);
static HOVERED_SETTING: AtomicU8 = AtomicU8::new(u8::MAX);
static HOVERED_NAVIGATION: AtomicU8 = AtomicU8::new(u8::MAX);
static HOVERED_DISTANCE: AtomicU8 = AtomicU8::new(u8::MAX);

// World-space anchor position (f64 keeps precision at large coordinates).
static ANCHOR_X: AtomicU64 = AtomicU64::new(0.0f64.to_bits());
static ANCHOR_Y: AtomicU64 = AtomicU64::new(0.0f64.to_bits());
static ANCHOR_Z: AtomicU64 = AtomicU64::new(0.0f64.to_bits());
// Last saved orientation, in quaternion x/y/z/w order. Rendering computes a live facing rotation.
static ANCHOR_QX: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static ANCHOR_QY: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static ANCHOR_QZ: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static ANCHOR_QW: AtomicU32 = AtomicU32::new(1.0f32.to_bits());

// Orbit is world-relative; Follow mode carries the same offset along with the player.
static ORBIT_YAW: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static ORBIT_PITCH: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static PANEL_DISTANCE: AtomicU32 = AtomicU32::new(2.25f32.to_bits());
static PANEL_SCALE: AtomicU32 = AtomicU32::new(1.0f32.to_bits());
static DISTANCE_DIRTY: AtomicBool = AtomicBool::new(false);

// Corner resize is separate from title-bar orbit drag.
static RESIZE_PANEL: AtomicU8 = AtomicU8::new(10);
static RESIZE_READY: AtomicBool = AtomicBool::new(false);
static RESIZE_LAST_YAW: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static RESIZE_LAST_PITCH: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static HOVERED_RESIZE_PANEL: AtomicU8 = AtomicU8::new(10);

// 2D screen size and pointer-drag state.
static TWO_D_SCALE: AtomicU32 = AtomicU32::new(1.0f32.to_bits());
static TWO_D_RESIZING: AtomicBool = AtomicBool::new(false);
static TWO_D_RESIZE_LAST_X: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static TWO_D_RESIZE_LAST_Y: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static DRAG_ORBIT_READY: AtomicBool = AtomicBool::new(false);
static LAST_YAW: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static LAST_PITCH: AtomicU32 = AtomicU32::new(0.0f32.to_bits());

// Animated switch progress: 0 = off, 1 = on.
static PIN_PROGRESS: AtomicU32 = AtomicU32::new(1.0f32.to_bits());
static FOLLOW_PROGRESS: AtomicU32 = AtomicU32::new(0.0f32.to_bits());

const MAX_PANELS: usize = 10;
const NO_PANEL: u8 = MAX_PANELS as u8;

// The existing scalar atomics above are the working copy for one panel.
// The arrays below keep each panel's placement and mode independent.
static PANEL_ACTIVE: [AtomicBool; MAX_PANELS] = [
    AtomicBool::new(true),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
];
static PANEL_FOLLOW: [AtomicBool; MAX_PANELS] = [const { AtomicBool::new(false) }; MAX_PANELS];
static PANEL_FOLLOW_CHANGED: [AtomicBool; MAX_PANELS] = [const { AtomicBool::new(false) }; MAX_PANELS];
static PANEL_ANCHOR_READY: [AtomicBool; MAX_PANELS] = [const { AtomicBool::new(false) }; MAX_PANELS];
static PANEL_DISTANCE_DIRTY: [AtomicBool; MAX_PANELS] = [const { AtomicBool::new(false) }; MAX_PANELS];
static PANEL_X: [AtomicU64; MAX_PANELS] = [const { AtomicU64::new(0.0f64.to_bits()) }; MAX_PANELS];
static PANEL_Y: [AtomicU64; MAX_PANELS] = [const { AtomicU64::new(0.0f64.to_bits()) }; MAX_PANELS];
static PANEL_Z: [AtomicU64; MAX_PANELS] = [const { AtomicU64::new(0.0f64.to_bits()) }; MAX_PANELS];
static PANEL_QX: [AtomicU32; MAX_PANELS] = [const { AtomicU32::new(0.0f32.to_bits()) }; MAX_PANELS];
static PANEL_QY: [AtomicU32; MAX_PANELS] = [const { AtomicU32::new(0.0f32.to_bits()) }; MAX_PANELS];
static PANEL_QZ: [AtomicU32; MAX_PANELS] = [const { AtomicU32::new(0.0f32.to_bits()) }; MAX_PANELS];
static PANEL_QW: [AtomicU32; MAX_PANELS] = [const { AtomicU32::new(1.0f32.to_bits()) }; MAX_PANELS];
static PANEL_ORBIT_YAW: [AtomicU32; MAX_PANELS] = [const { AtomicU32::new(0.0f32.to_bits()) }; MAX_PANELS];
static PANEL_ORBIT_PITCH: [AtomicU32; MAX_PANELS] = [const { AtomicU32::new(0.0f32.to_bits()) }; MAX_PANELS];
static PANEL_DISTANCES: [AtomicU32; MAX_PANELS] = [const { AtomicU32::new(2.25f32.to_bits()) }; MAX_PANELS];
static PANEL_SCALES: [AtomicU32; MAX_PANELS] = [const { AtomicU32::new(1.0f32.to_bits()) }; MAX_PANELS];

static SELECTED_PANEL: AtomicU8 = AtomicU8::new(0);
static HOVERED_PANEL: AtomicU8 = AtomicU8::new(NO_PANEL);
static HOVERED_PANEL_DEPTH: AtomicU32 = AtomicU32::new(f32::INFINITY.to_bits());
static DRAG_PANEL: AtomicU8 = AtomicU8::new(NO_PANEL);
static HOVERED_ADD: AtomicBool = AtomicBool::new(false);
static MODE_DROPDOWN_OPEN: AtomicBool = AtomicBool::new(false);
static TWO_D_MODE: AtomicBool = AtomicBool::new(false);
static TWO_D_PROGRESS: AtomicU32 = AtomicU32::new(0.0f32.to_bits());
static TWO_D_POINTER_OVER: AtomicBool = AtomicBool::new(false);
static HOVERED_MODE_ACTION: AtomicU8 = AtomicU8::new(0);
// Hover actions: 0 none, 1 mode row, 2 Follow, 3 Keep, 4 2D mode,
// 5 distance minus, 6 distance plus, 7 add panel, 8 previous panel, 9 next panel.

pub fn is_open() -> bool {
    OPEN.load(Ordering::Relaxed)
}

pub fn toggle_open() -> bool {
    let next = !is_open();
    OPEN.store(next, Ordering::Relaxed);
    if !next {
        DRAGGING.store(false, Ordering::Relaxed);
        DRAG_PANEL.store(NO_PANEL, Ordering::Relaxed);
        DRAG_HOVERED.store(false, Ordering::Relaxed);
        HOVERED_SETTING.store(u8::MAX, Ordering::Relaxed);
        HOVERED_NAVIGATION.store(u8::MAX, Ordering::Relaxed);
        HOVERED_DISTANCE.store(u8::MAX, Ordering::Relaxed);
        LAST_VIEW_READY.store(false, Ordering::Relaxed);
        DRAG_ORBIT_READY.store(false, Ordering::Relaxed);
        RESIZE_PANEL.store(NO_PANEL, Ordering::Relaxed);
        RESIZE_READY.store(false, Ordering::Relaxed);
        TWO_D_RESIZING.store(false, Ordering::Relaxed);
    }
    next
}

pub fn close() {
    OPEN.store(false, Ordering::Relaxed);
    DRAGGING.store(false, Ordering::Relaxed);
    DRAG_PANEL.store(NO_PANEL, Ordering::Relaxed);
    DRAG_HOVERED.store(false, Ordering::Relaxed);
    HOVERED_SETTING.store(u8::MAX, Ordering::Relaxed);
    HOVERED_NAVIGATION.store(u8::MAX, Ordering::Relaxed);
    HOVERED_DISTANCE.store(u8::MAX, Ordering::Relaxed);
    LAST_VIEW_READY.store(false, Ordering::Relaxed);
    DRAG_ORBIT_READY.store(false, Ordering::Relaxed);
    RESIZE_PANEL.store(NO_PANEL, Ordering::Relaxed);
    RESIZE_READY.store(false, Ordering::Relaxed);
    TWO_D_RESIZING.store(false, Ordering::Relaxed);
}

pub fn follow_mode() -> bool {
    FOLLOW_MODE.load(Ordering::Relaxed)
}

pub fn set_follow_mode(follow: bool) {
    let changed = FOLLOW_MODE.swap(follow, Ordering::Relaxed) != follow;
    if changed {
        // Preserve the current placement. world_submit converts the same
        // position between world-anchored and player-relative coordinates.
        FOLLOW_MODE_CHANGED.store(true, Ordering::Relaxed);
        LAST_VIEW_READY.store(false, Ordering::Relaxed);
        DRAG_ORBIT_READY.store(false, Ordering::Relaxed);
        DRAGGING.store(false, Ordering::Relaxed);
        DRAG_PANEL.store(NO_PANEL, Ordering::Relaxed);
        DRAG_HOVERED.store(false, Ordering::Relaxed);
        RESIZE_PANEL.store(NO_PANEL, Ordering::Relaxed);
        RESIZE_READY.store(false, Ordering::Relaxed);
    }
    SETTINGS_SELECTED.store(if follow { 1 } else { 0 }, Ordering::Relaxed);
}

pub fn take_follow_mode_changed() -> bool {
    FOLLOW_MODE_CHANGED.swap(false, Ordering::Relaxed)
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
    if dragging {
        DRAG_PANEL.store(selected_panel().min((MAX_PANELS - 1) as u8), Ordering::Relaxed);
    } else {
        DRAG_PANEL.store(NO_PANEL, Ordering::Relaxed);
    }
    LAST_VIEW_READY.store(false, Ordering::Relaxed);
    DRAG_ORBIT_READY.store(false, Ordering::Relaxed);
}

pub fn is_dragging() -> bool {
    DRAGGING.load(Ordering::Relaxed)
}

/// True only while the crosshair is over the panel's top title bar.
pub fn set_drag_hovered(hovered: bool) {
    DRAG_HOVERED.store(hovered, Ordering::Relaxed);
}

pub fn can_drag_from_header() -> bool {
    DRAG_HOVERED.load(Ordering::Relaxed)
}

pub fn set_hovered_resize_panel(index: Option<usize>) {
    HOVERED_RESIZE_PANEL.store(index.map_or(NO_PANEL, |i| i.min(MAX_PANELS - 1) as u8), Ordering::Relaxed);
}

pub fn hovered_resize_panel() -> Option<usize> {
    let value = HOVERED_RESIZE_PANEL.load(Ordering::Relaxed);
    (value < MAX_PANELS as u8).then_some(value as usize)
}

pub fn begin_3d_resize(index: usize, yaw: f32, pitch: f32) {
    if index >= MAX_PANELS || !panel_exists(index) {
        return;
    }
    SELECTED_PANEL.store(index as u8, Ordering::Relaxed);
    RESIZE_PANEL.store(index as u8, Ordering::Relaxed);
    RESIZE_READY.store(true, Ordering::Relaxed);
    RESIZE_LAST_YAW.store(yaw.to_bits(), Ordering::Relaxed);
    RESIZE_LAST_PITCH.store(pitch.to_bits(), Ordering::Relaxed);
    DRAGGING.store(false, Ordering::Relaxed);
    DRAG_PANEL.store(NO_PANEL, Ordering::Relaxed);
    LAST_VIEW_READY.store(false, Ordering::Relaxed);
    DRAG_ORBIT_READY.store(false, Ordering::Relaxed);
}

pub fn end_3d_resize() {
    RESIZE_PANEL.store(NO_PANEL, Ordering::Relaxed);
    RESIZE_READY.store(false, Ordering::Relaxed);
}

pub fn is_3d_resizing_for(index: usize) -> bool {
    RESIZE_PANEL.load(Ordering::Relaxed) == index as u8
}

pub fn update_3d_resize(index: usize, yaw: f32, pitch: f32) {
    if !is_3d_resizing_for(index) || !yaw.is_finite() || !pitch.is_finite() {
        return;
    }
    if !RESIZE_READY.load(Ordering::Relaxed) {
        RESIZE_LAST_YAW.store(yaw.to_bits(), Ordering::Relaxed);
        RESIZE_LAST_PITCH.store(pitch.to_bits(), Ordering::Relaxed);
        RESIZE_READY.store(true, Ordering::Relaxed);
        return;
    }

    let last_yaw = f32::from_bits(RESIZE_LAST_YAW.load(Ordering::Relaxed));
    let last_pitch = f32::from_bits(RESIZE_LAST_PITCH.load(Ordering::Relaxed));
    let mut yaw_delta = yaw - last_yaw;
    if yaw_delta > std::f32::consts::PI {
        yaw_delta -= std::f32::consts::TAU;
    } else if yaw_delta < -std::f32::consts::PI {
        yaw_delta += std::f32::consts::TAU;
    }
    let pitch_delta = pitch - last_pitch;
    // Vertical mouse motion is the main resize axis; horizontal motion adds
    // a smaller contribution so the corner remains usable at the pitch limit.
    let delta = -pitch_delta * 1.6 + yaw_delta * 0.25;
    if delta.is_finite() && delta.abs() > 0.00001 {
        set_panel_scale(panel_scale() + delta);
    }
    RESIZE_LAST_YAW.store(yaw.to_bits(), Ordering::Relaxed);
    RESIZE_LAST_PITCH.store(pitch.to_bits(), Ordering::Relaxed);
}

pub fn panel_scale() -> f32 {
    f32::from_bits(PANEL_SCALE.load(Ordering::Relaxed)).clamp(0.45, 2.4)
}

pub fn set_panel_scale(scale: f32) {
    if scale.is_finite() {
        PANEL_SCALE.store(scale.clamp(0.45, 2.4).to_bits(), Ordering::Relaxed);
    }
}

pub fn two_d_scale() -> f32 {
    f32::from_bits(TWO_D_SCALE.load(Ordering::Relaxed)).clamp(0.55, 1.8)
}

pub fn begin_2d_resize(pointer: [f32; 2]) {
    TWO_D_RESIZE_LAST_X.store(pointer[0].to_bits(), Ordering::Relaxed);
    TWO_D_RESIZE_LAST_Y.store(pointer[1].to_bits(), Ordering::Relaxed);
    TWO_D_RESIZING.store(true, Ordering::Relaxed);
    RESIZE_PANEL.store(NO_PANEL, Ordering::Relaxed);
}

pub fn end_2d_resize() {
    TWO_D_RESIZING.store(false, Ordering::Relaxed);
}

pub fn is_2d_resizing() -> bool {
    TWO_D_RESIZING.load(Ordering::Relaxed)
}

pub fn update_2d_resize(pointer: [f32; 2]) {
    if !is_2d_resizing() || !pointer[0].is_finite() || !pointer[1].is_finite() {
        return;
    }
    let last_x = f32::from_bits(TWO_D_RESIZE_LAST_X.load(Ordering::Relaxed));
    let last_y = f32::from_bits(TWO_D_RESIZE_LAST_Y.load(Ordering::Relaxed));
    let delta = ((last_x - pointer[0]) + (last_y - pointer[1])) * 0.0025;
    if delta.is_finite() && delta.abs() > 0.00001 {
        let next = (two_d_scale() + delta).clamp(0.55, 1.8);
        TWO_D_SCALE.store(next.to_bits(), Ordering::Relaxed);
    }
    TWO_D_RESIZE_LAST_X.store(pointer[0].to_bits(), Ordering::Relaxed);
    TWO_D_RESIZE_LAST_Y.store(pointer[1].to_bits(), Ordering::Relaxed);
}

pub fn set_hovered_setting(index: Option<usize>) {
    HOVERED_SETTING.store(index.map_or(u8::MAX, |i| i.min(1) as u8), Ordering::Relaxed);
}

pub fn hovered_setting() -> Option<usize> {
    let value = HOVERED_SETTING.load(Ordering::Relaxed);
    (value < 2).then_some(value as usize)
}

pub fn set_hovered_navigation(index: Option<usize>) {
    HOVERED_NAVIGATION.store(index.map_or(u8::MAX, |i| i.min(2) as u8), Ordering::Relaxed);
}

pub fn hovered_navigation() -> Option<usize> {
    let value = HOVERED_NAVIGATION.load(Ordering::Relaxed);
    (value < 3).then_some(value as usize)
}

pub fn set_hovered_distance_control(index: Option<usize>) {
    HOVERED_DISTANCE.store(index.map_or(u8::MAX, |i| i.min(1) as u8), Ordering::Relaxed);
}

pub fn hovered_distance_control() -> Option<usize> {
    let value = HOVERED_DISTANCE.load(Ordering::Relaxed);
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

pub fn panel_distance() -> f32 {
    f32::from_bits(PANEL_DISTANCE.load(Ordering::Relaxed)).clamp(1.0, 8.0)
}

pub fn set_panel_distance(distance: f32) {
    if !distance.is_finite() {
        return;
    }
    let next = distance.clamp(1.0, 8.0);
    let previous = panel_distance();
    if (next - previous).abs() > 0.0005 {
        PANEL_DISTANCE.store(next.to_bits(), Ordering::Relaxed);
        DISTANCE_DIRTY.store(true, Ordering::Relaxed);
    }
}

pub fn adjust_panel_distance(delta: f32) {
    if delta.is_finite() {
        set_panel_distance(panel_distance() + delta);
    }
}

pub fn take_distance_dirty() -> bool {
    DISTANCE_DIRTY.swap(false, Ordering::Relaxed)
}

pub fn orbit_angles() -> (f32, f32) {
    (
        f32::from_bits(ORBIT_YAW.load(Ordering::Relaxed)),
        f32::from_bits(ORBIT_PITCH.load(Ordering::Relaxed)),
    )
}

pub fn set_orbit_angles(yaw: f32, pitch: f32) {
    if yaw.is_finite() && pitch.is_finite() {
        ORBIT_YAW.store(yaw.to_bits(), Ordering::Relaxed);
        ORBIT_PITCH.store(pitch.clamp(-1.35, 1.35).to_bits(), Ordering::Relaxed);
    }
}

pub fn orbit_offset(yaw: f32, pitch: f32, distance: f32) -> glam::Vec3 {
    let horizontal = pitch.cos() * distance;
    glam::Vec3::new(
        yaw.sin() * horizontal,
        pitch.sin() * distance,
        -yaw.cos() * horizontal,
    )
}

pub fn set_anchor_position(position: [f64; 3]) {
    ANCHOR_X.store(position[0].to_bits(), Ordering::Relaxed);
    ANCHOR_Y.store(position[1].to_bits(), Ordering::Relaxed);
    ANCHOR_Z.store(position[2].to_bits(), Ordering::Relaxed);
}

/// While RMB is held on the title bar, camera yaw/pitch orbit the panel around
/// the camera position. Releasing RMB leaves the pinned panel at that world point.
pub fn update_drag_view(yaw: f32, pitch: f32, camera_position: [f64; 3]) {
    let camera = glam::DVec3::from_array(camera_position);

    if is_dragging() {
        if !DRAG_ORBIT_READY.load(Ordering::Relaxed) {
            if !follow_mode() && anchor_ready() {
                let (position, _) = anchor();
                let relative = glam::DVec3::from_array(position) - camera;
                let radius = relative.length();
                if radius.is_finite() && radius > 0.05 {
                    PANEL_DISTANCE.store((radius as f32).clamp(1.0, 8.0).to_bits(), Ordering::Relaxed);
                    ORBIT_YAW.store((relative.x.atan2(-relative.z) as f32).to_bits(), Ordering::Relaxed);
                    ORBIT_PITCH.store(((relative.y / radius).clamp(-1.0, 1.0).asin() as f32).to_bits(), Ordering::Relaxed);
                }
            }
            DRAG_ORBIT_READY.store(true, Ordering::Relaxed);
        }

        if LAST_VIEW_READY.load(Ordering::Relaxed) {
            let last_yaw = f32::from_bits(LAST_YAW.load(Ordering::Relaxed));
            let last_pitch = f32::from_bits(LAST_PITCH.load(Ordering::Relaxed));
            let mut yaw_delta = yaw - last_yaw;
            if yaw_delta > std::f32::consts::PI {
                yaw_delta -= std::f32::consts::TAU;
            } else if yaw_delta < -std::f32::consts::PI {
                yaw_delta += std::f32::consts::TAU;
            }
            let pitch_delta = pitch - last_pitch;
            let (old_yaw, old_pitch) = orbit_angles();
            let next_yaw = (old_yaw - yaw_delta).rem_euclid(std::f32::consts::TAU);
            let next_pitch = (old_pitch + pitch_delta).clamp(-1.35, 1.35);
            ORBIT_YAW.store(next_yaw.to_bits(), Ordering::Relaxed);
            ORBIT_PITCH.store(next_pitch.to_bits(), Ordering::Relaxed);

            let offset = orbit_offset(next_yaw, next_pitch, panel_distance()).as_dvec3();
            set_anchor_position((camera + offset).to_array());
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


pub fn max_panels() -> usize { MAX_PANELS }

pub fn panel_exists(index: usize) -> bool {
    index < MAX_PANELS && PANEL_ACTIVE[index].load(Ordering::Relaxed)
}

pub fn panel_count() -> usize {
    PANEL_ACTIVE.iter().filter(|slot| slot.load(Ordering::Relaxed)).count()
}

pub fn selected_panel() -> u8 {
    SELECTED_PANEL.load(Ordering::Relaxed).min((MAX_PANELS - 1) as u8)
}

pub fn set_selected_panel(index: usize) {
    if index < MAX_PANELS && panel_exists(index) {
        SELECTED_PANEL.store(index as u8, Ordering::Relaxed);
    }
}

pub fn step_selected_panel(forward: bool) -> Option<usize> {
    let current = selected_panel() as usize;
    for step in 1..=MAX_PANELS {
        let index = if forward {
            (current + step) % MAX_PANELS
        } else {
            (current + MAX_PANELS - (step % MAX_PANELS)) % MAX_PANELS
        };
        if panel_exists(index) {
            set_selected_panel(index);
            close_dropdown();
            return Some(index);
        }
    }
    None
}

pub fn hovered_panel() -> Option<usize> {
    let value = HOVERED_PANEL.load(Ordering::Relaxed);
    (value < MAX_PANELS as u8).then_some(value as usize)
}

pub fn set_hovered_panel(index: usize) {
    if index < MAX_PANELS {
        HOVERED_PANEL.store(index as u8, Ordering::Relaxed);
    }
}

pub fn set_hovered_panel_if_empty(index: usize) -> bool {
    if index >= MAX_PANELS { return false; }
    HOVERED_PANEL.compare_exchange(
        NO_PANEL, index as u8, Ordering::Relaxed, Ordering::Relaxed
    ).is_ok()
}

pub fn set_hovered_panel_if_closer(index: usize, depth: f32) -> bool {
    if index >= MAX_PANELS || !depth.is_finite() || depth <= 0.0 {
        return false;
    }
    loop {
        let previous = HOVERED_PANEL_DEPTH.load(Ordering::Relaxed);
        if depth >= f32::from_bits(previous) {
            return false;
        }
        if HOVERED_PANEL_DEPTH.compare_exchange(
            previous,
            depth.to_bits(),
            Ordering::Relaxed,
            Ordering::Relaxed,
        ).is_ok() {
            HOVERED_PANEL.store(index as u8, Ordering::Relaxed);
            return true;
        }
    }
}

pub fn clear_hover_state() {
    HOVERED_PANEL.store(NO_PANEL, Ordering::Relaxed);
    HOVERED_PANEL_DEPTH.store(f32::INFINITY.to_bits(), Ordering::Relaxed);
    HOVERED_ADD.store(false, Ordering::Relaxed);
    HOVERED_MODE_ACTION.store(0, Ordering::Relaxed);
    HOVERED_SETTING.store(u8::MAX, Ordering::Relaxed);
    HOVERED_NAVIGATION.store(u8::MAX, Ordering::Relaxed);
    HOVERED_DISTANCE.store(u8::MAX, Ordering::Relaxed);
    DRAG_HOVERED.store(false, Ordering::Relaxed);
    HOVERED_RESIZE_PANEL.store(NO_PANEL, Ordering::Relaxed);
}

pub fn set_hovered_add(value: bool) {
    HOVERED_ADD.store(value, Ordering::Relaxed);
}

pub fn hovered_add() -> bool {
    HOVERED_ADD.load(Ordering::Relaxed)
}

pub fn set_hovered_mode_action(action: u8) {
    HOVERED_MODE_ACTION.store(action.min(11), Ordering::Relaxed);
}

pub fn hovered_mode_action() -> u8 {
    HOVERED_MODE_ACTION.load(Ordering::Relaxed)
}

pub fn dropdown_open() -> bool {
    MODE_DROPDOWN_OPEN.load(Ordering::Relaxed)
}

pub fn toggle_dropdown() {
    let next = !dropdown_open();
    MODE_DROPDOWN_OPEN.store(next, Ordering::Relaxed);
}

pub fn close_dropdown() {
    MODE_DROPDOWN_OPEN.store(false, Ordering::Relaxed);
}

pub fn is_2d_mode() -> bool {
    TWO_D_MODE.load(Ordering::Relaxed)
}

pub fn toggle_2d_mode() -> bool {
    let next = !is_2d_mode();
    TWO_D_MODE.store(next, Ordering::Relaxed);
    next
}

pub fn two_d_progress() -> f32 {
    let target = if is_2d_mode() { 1.0 } else { 0.0 };
    step_progress(&TWO_D_PROGRESS, target)
}
pub fn set_2d_pointer_over(over: bool) {
    TWO_D_POINTER_OVER.store(over, Ordering::Relaxed);
}

pub fn set_2d_mode(enabled: bool) {
    TWO_D_MODE.store(enabled, Ordering::Relaxed);
    if !enabled {
        TWO_D_POINTER_OVER.store(false, Ordering::Relaxed);
    }
}

pub fn two_d_pointer_over() -> bool {
    TWO_D_POINTER_OVER.load(Ordering::Relaxed)
}

/// Remove a panel while keeping at least one panel alive. The slot is reusable
/// by add_panel(), so the hard limit remains ten active panels.
pub fn remove_panel(index: usize) -> bool {
    if index >= MAX_PANELS || !panel_exists(index) || panel_count() <= 1 {
        return false;
    }

    PANEL_ACTIVE[index].store(false, Ordering::Relaxed);
    PANEL_FOLLOW[index].store(false, Ordering::Relaxed);
    PANEL_FOLLOW_CHANGED[index].store(false, Ordering::Relaxed);
    PANEL_ANCHOR_READY[index].store(false, Ordering::Relaxed);
    PANEL_DISTANCE_DIRTY[index].store(false, Ordering::Relaxed);
    PANEL_X[index].store(0.0f64.to_bits(), Ordering::Relaxed);
    PANEL_Y[index].store(0.0f64.to_bits(), Ordering::Relaxed);
    PANEL_Z[index].store(0.0f64.to_bits(), Ordering::Relaxed);
    PANEL_QX[index].store(0.0f32.to_bits(), Ordering::Relaxed);
    PANEL_QY[index].store(0.0f32.to_bits(), Ordering::Relaxed);
    PANEL_QZ[index].store(0.0f32.to_bits(), Ordering::Relaxed);
    PANEL_QW[index].store(1.0f32.to_bits(), Ordering::Relaxed);
    PANEL_ORBIT_YAW[index].store(0.0f32.to_bits(), Ordering::Relaxed);
    PANEL_ORBIT_PITCH[index].store(0.0f32.to_bits(), Ordering::Relaxed);
    PANEL_DISTANCES[index].store(2.25f32.to_bits(), Ordering::Relaxed);
    PANEL_SCALES[index].store(1.0f32.to_bits(), Ordering::Relaxed);

    let next = (0..MAX_PANELS)
        .find(|candidate| panel_exists(*candidate))
        .unwrap_or(0);
    SELECTED_PANEL.store(next as u8, Ordering::Relaxed);
    DRAG_PANEL.store(NO_PANEL, Ordering::Relaxed);
    DRAGGING.store(false, Ordering::Relaxed);
    RESIZE_PANEL.store(NO_PANEL, Ordering::Relaxed);
    RESIZE_READY.store(false, Ordering::Relaxed);
    TWO_D_RESIZING.store(false, Ordering::Relaxed);
    MODE_DROPDOWN_OPEN.store(false, Ordering::Relaxed);
    HOVERED_PANEL.store(NO_PANEL, Ordering::Relaxed);
    HOVERED_ADD.store(false, Ordering::Relaxed);
    HOVERED_MODE_ACTION.store(0, Ordering::Relaxed);
    true
}

pub fn add_panel() -> Option<usize> {
    let Some(index) = (0..MAX_PANELS).find(|i| !panel_exists(*i)) else {
        return None;
    };
    PANEL_ACTIVE[index].store(true, Ordering::Relaxed);
    PANEL_FOLLOW[index].store(false, Ordering::Relaxed);
    PANEL_FOLLOW_CHANGED[index].store(false, Ordering::Relaxed);
    PANEL_ANCHOR_READY[index].store(false, Ordering::Relaxed);
    PANEL_DISTANCE_DIRTY[index].store(false, Ordering::Relaxed);
    PANEL_X[index].store(0.0f64.to_bits(), Ordering::Relaxed);
    PANEL_Y[index].store(0.0f64.to_bits(), Ordering::Relaxed);
    PANEL_Z[index].store(0.0f64.to_bits(), Ordering::Relaxed);
    PANEL_QX[index].store(0.0f32.to_bits(), Ordering::Relaxed);
    PANEL_QY[index].store(0.0f32.to_bits(), Ordering::Relaxed);
    PANEL_QZ[index].store(0.0f32.to_bits(), Ordering::Relaxed);
    PANEL_QW[index].store(1.0f32.to_bits(), Ordering::Relaxed);
    PANEL_ORBIT_YAW[index].store(0.0f32.to_bits(), Ordering::Relaxed);
    PANEL_ORBIT_PITCH[index].store(0.0f32.to_bits(), Ordering::Relaxed);
    PANEL_DISTANCES[index].store(2.25f32.to_bits(), Ordering::Relaxed);
    PANEL_SCALES[index].store(1.0f32.to_bits(), Ordering::Relaxed);
    SELECTED_PANEL.store(index as u8, Ordering::Relaxed);
    MODE_DROPDOWN_OPEN.store(false, Ordering::Relaxed);
    Some(index)
}

pub fn load_panel(index: usize) {
    if index >= MAX_PANELS { return; }
    FOLLOW_MODE.store(PANEL_FOLLOW[index].load(Ordering::Relaxed), Ordering::Relaxed);
    FOLLOW_MODE_CHANGED.store(PANEL_FOLLOW_CHANGED[index].load(Ordering::Relaxed), Ordering::Relaxed);
    ANCHOR_READY.store(PANEL_ANCHOR_READY[index].load(Ordering::Relaxed), Ordering::Relaxed);
    DISTANCE_DIRTY.store(PANEL_DISTANCE_DIRTY[index].load(Ordering::Relaxed), Ordering::Relaxed);
    ANCHOR_X.store(PANEL_X[index].load(Ordering::Relaxed), Ordering::Relaxed);
    ANCHOR_Y.store(PANEL_Y[index].load(Ordering::Relaxed), Ordering::Relaxed);
    ANCHOR_Z.store(PANEL_Z[index].load(Ordering::Relaxed), Ordering::Relaxed);
    ANCHOR_QX.store(PANEL_QX[index].load(Ordering::Relaxed), Ordering::Relaxed);
    ANCHOR_QY.store(PANEL_QY[index].load(Ordering::Relaxed), Ordering::Relaxed);
    ANCHOR_QZ.store(PANEL_QZ[index].load(Ordering::Relaxed), Ordering::Relaxed);
    ANCHOR_QW.store(PANEL_QW[index].load(Ordering::Relaxed), Ordering::Relaxed);
    ORBIT_YAW.store(PANEL_ORBIT_YAW[index].load(Ordering::Relaxed), Ordering::Relaxed);
    ORBIT_PITCH.store(PANEL_ORBIT_PITCH[index].load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_DISTANCE.store(PANEL_DISTANCES[index].load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_SCALE.store(PANEL_SCALES[index].load(Ordering::Relaxed), Ordering::Relaxed);
    DRAGGING.store(DRAG_PANEL.load(Ordering::Relaxed) == index as u8, Ordering::Relaxed);
}

pub fn save_panel(index: usize) {
    if index >= MAX_PANELS { return; }
    PANEL_FOLLOW[index].store(FOLLOW_MODE.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_FOLLOW_CHANGED[index].store(FOLLOW_MODE_CHANGED.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_ANCHOR_READY[index].store(ANCHOR_READY.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_DISTANCE_DIRTY[index].store(DISTANCE_DIRTY.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_X[index].store(ANCHOR_X.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_Y[index].store(ANCHOR_Y.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_Z[index].store(ANCHOR_Z.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_QX[index].store(ANCHOR_QX.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_QY[index].store(ANCHOR_QY.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_QZ[index].store(ANCHOR_QZ.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_QW[index].store(ANCHOR_QW.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_ORBIT_YAW[index].store(ORBIT_YAW.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_ORBIT_PITCH[index].store(ORBIT_PITCH.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_DISTANCES[index].store(PANEL_DISTANCE.load(Ordering::Relaxed), Ordering::Relaxed);
    PANEL_SCALES[index].store(PANEL_SCALE.load(Ordering::Relaxed), Ordering::Relaxed);
}

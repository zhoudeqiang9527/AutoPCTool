use autopilot::{geometry::Point, mouse, screen};
use lazy_static::lazy_static;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref SHOULD_STOP: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

#[tauri::command]
pub fn scan_once(colors: Vec<[u8; 3]>, start_x: f64, end_x: f64, y: f64) -> bool {
    // 调用 scan_colors 获取屏幕上的颜色
    println!("scan_once start_x: {}, end_x: {}, y: {}", start_x, end_x, y);
    let screen_colors = scan_colors(start_x, end_x, y);
    // 计算颜色差异
    let mut is_match = true;
    for i in 0..colors.len() {
        if screen_colors[i] != colors[i] {
            is_match = false;
            break;
        }
    }
    // 如果颜色匹配，则点击鼠标左键
    if is_match {
        println!("match!");
        let x_point = (start_x + end_x) / 2.0;
        //鼠标移动
        let _ = mouse::move_to(Point::new(x_point, y));
        //鼠标左键点击
        thread::sleep(Duration::from_millis(100));
        let _ = mouse::click(mouse::Button::Left, None);
    }
    // 返回是否匹配
    is_match
}

//间隔5秒 扫描
#[tauri::command]
pub fn scan_loop(colors: Vec<[u8; 3]>, start_x: f64, end_x: f64, y: f64, interval: u64) {
    SHOULD_STOP.store(false, Ordering::Relaxed);
    let stop_flg = SHOULD_STOP.clone();
    thread::spawn(move || {
        while !stop_flg.load(Ordering::Relaxed) {
            scan_once(colors.clone(), start_x, end_x, y);
            thread::sleep(Duration::from_millis(interval));
        }
    });
}

//停止扫描
#[tauri::command]
pub fn stop_scan() {
    SHOULD_STOP.store(true, Ordering::Relaxed);
}

// 获取屏幕上某个点的颜色
#[tauri::command]
pub fn scan_colors(start_x: f64, end_x: f64, y: f64) -> Vec<[u8; 3]> {
    let mut points: Vec<Point> = vec![];
    let mut x = start_x;
    while x < end_x {
        points.push(Point::new(x, y));
        x += 1.0;
    }
    // 循环获取坐标数组的颜色
    let mut colors: Vec<[u8; 3]> = vec![];
    for point in points {
        let pixel = screen::get_color(point).unwrap();
        colors.push([pixel[0], pixel[1], pixel[2]]);
    }
    colors
}

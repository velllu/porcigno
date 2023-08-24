use undici::x11::{common::Vector2, window::Window};

pub fn master(windows: &Vec<Window>, monitor_size: &Vector2<i32>) {
    let windows_num = windows.len() as i32;

    let mut i = -1;
    for window in windows {
        if i < 0 {
            window.set_position(Vector2::new(0, 0));

            window.set_scale(Vector2::new(
                if windows_num == 1 {
                    monitor_size.x as u32
                } else {
                    (monitor_size.x / 2) as u32
                },
                monitor_size.y as u32,
            ));
        } else {
            window.set_position(Vector2::new(
                monitor_size.x / 2,
                i * (monitor_size.x / (windows_num - 1)),
            ));

            window.set_scale(Vector2::new(
                (monitor_size.x / 2) as u32,
                (monitor_size.y / (windows_num - 1)) as u32,
            ));
        }

        i += 1;
    }
}

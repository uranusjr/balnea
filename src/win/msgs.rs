use winapi::shared::minwindef::UINT;
use winapi::um::winuser::WM_APP;

pub(crate) static WINDOW_CLOSED: UINT = WM_APP + 0x1;

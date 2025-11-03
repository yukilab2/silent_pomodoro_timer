use eframe::egui;
use crate::settings::{Settings, FlashColor};

#[derive(PartialEq, Clone, Copy)]
enum TimerState {
    Waiting,      // 開始待ち
    Working,      // 25分計測中
    WorkingPaused, // 25分一時停止中
    BreakFlashing, // 5分開始前のフラッシュ中
    Breaking,     // 5分計測中
    BreakingPaused, // 5分一時停止中
    WorkFlashing, // 25分開始前のフラッシュ中
}

pub struct PomodoroApp {
    state: TimerState,
    remaining_seconds: u64,
    settings: Settings,
    show_settings: bool,
    flash_phase: f32, // 0.0 ~ 1.0 でフラッシュの位相（0=暗い、1=明るい）
    last_update: instant::Instant,
}

impl PomodoroApp {
    pub fn new() -> Self {
        let settings = Settings::load();
        Self {
            state: TimerState::Waiting,
            remaining_seconds: settings.work_duration_seconds,
            settings,
            show_settings: false,
            flash_phase: 0.0,
            last_update: instant::Instant::now(),
        }
    }

    fn update_timer(&mut self, ctx: &egui::Context) {
        let now = instant::Instant::now();
        let elapsed = now.duration_since(self.last_update);

        match self.state {
            TimerState::Waiting | TimerState::WorkingPaused | TimerState::BreakingPaused => {
                self.last_update = now;
            }
            TimerState::Working | TimerState::Breaking => {
                let elapsed_secs = elapsed.as_secs();
                if elapsed_secs > 0 && self.remaining_seconds >= elapsed_secs {
                    self.remaining_seconds -= elapsed_secs;
                    self.last_update = now;
                    ctx.request_repaint();
                }
                if self.remaining_seconds == 0 {
                    match self.state {
                        TimerState::Working => {
                            self.state = TimerState::BreakFlashing;
                            self.remaining_seconds = self.settings.break_duration_seconds;
                            self.flash_phase = 0.0;
                        }
                        TimerState::Breaking => {
                            self.state = TimerState::WorkFlashing;
                            self.remaining_seconds = self.settings.work_duration_seconds;
                            self.flash_phase = 0.0;
                        }
                        _ => {}
                    }
                }
                // フォーカスがないときでも更新が続くように、短い間隔でrepaintを要求
                // 次の秒までの残り時間を計算（最大1秒）
                let elapsed_millis = elapsed.as_millis();
                let remaining_to_next_second = if elapsed_millis % 1000 == 0 {
                    1000
                } else {
                    1000 - ((elapsed_millis % 1000) as u64)
                };
                ctx.request_repaint_after(std::time::Duration::from_millis(remaining_to_next_second));
            }
            TimerState::BreakFlashing | TimerState::WorkFlashing => {
                // 2秒で明るく、2秒で暗く（合計4秒の周期）
                let flash_cycle_duration = 4.0; // 4秒
                let elapsed_secs = elapsed.as_secs_f32();
                // flash_phaseは0.0-1.0の範囲を維持（位相として）
                self.flash_phase = (self.flash_phase + elapsed_secs / flash_cycle_duration) % 1.0;
                
                self.last_update = now;
                ctx.request_repaint();
                // 滑らかなアニメーションのために、適度な間隔でrepaintを要求（200ms）
                ctx.request_repaint_after(std::time::Duration::from_millis(200));
            }
        }
    }
}

impl eframe::App for PomodoroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_timer(ctx);

        // 設定ウィンドウ
        if self.show_settings {
            egui::Window::new("Settings")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Settings");
                        ui.separator();
                        
                        ui.label("Flash Color:");
                        egui::ComboBox::from_id_salt("flash_color")
                            .selected_text(format!("{:?}", self.settings.flash_color))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.settings.flash_color, FlashColor::White, "White");
                                ui.selectable_value(&mut self.settings.flash_color, FlashColor::Black, "Black");
                                ui.selectable_value(&mut self.settings.flash_color, FlashColor::Red, "Red");
                                ui.selectable_value(&mut self.settings.flash_color, FlashColor::Blue, "Blue");
                                ui.selectable_value(&mut self.settings.flash_color, FlashColor::Green, "Green");
                            });
                        
                        ui.separator();
                        
                        if ui.button("Close").clicked() {
                            self.settings.save();
                            self.show_settings = false;
                        }
                    });
                });
        }

        // メインウィンドウ

        // 設定ボタン（右上）
        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙").clicked() {
                        self.show_settings = !self.show_settings;
                    }
                });
            });

        // 中央のボタン領域
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let button_rect = ui.max_rect();
                
                // フラッシュ中は背景色を変更
                let bg_color = match self.state {
                    TimerState::BreakFlashing | TimerState::WorkFlashing => {
                        let flash_rgb = self.settings.flash_color.to_rgb();
                        // 0.0-0.5で暗→明（2秒で0.0から1.0へ）、0.5-1.0で明→暗（2秒で1.0から0.0へ）
                        let brightness = if self.flash_phase < 0.5 {
                            self.flash_phase * 2.0 // 0.0 -> 1.0
                        } else {
                            2.0 - self.flash_phase * 2.0 // 1.0 -> 0.0
                        };
                        egui::Color32::from_rgb(
                            (flash_rgb[0] as f32 * brightness) as u8,
                            (flash_rgb[1] as f32 * brightness) as u8,
                            (flash_rgb[2] as f32 * brightness) as u8,
                        )
                    }
                    _ => egui::Color32::from_gray(50), // 通常の背景色
                };
                
                ui.painter().rect_filled(button_rect, 0.0, bg_color);
                
                // 全画面クリック可能な領域（先に確保してクリックイベントを受け取る）
                let button_response = ui.allocate_rect(button_rect, egui::Sense::click());
                
                // タイマー表示と状態表示を中央に配置（クリックをブロックしないようにpainterで直接描画）
                let minutes = self.remaining_seconds / 60;
                let seconds = self.remaining_seconds % 60;
                let time_text = format!("{:02}:{:02}", minutes, seconds);
                
                let state_text = match self.state {
                    TimerState::Waiting => "Click to Start",
                    TimerState::Working => "Working",
                    TimerState::WorkingPaused => "Paused",
                    TimerState::Breaking => "Break",
                    TimerState::BreakingPaused => "Paused",
                    TimerState::BreakFlashing => "Break Time!",
                    TimerState::WorkFlashing => "Work Time!",
                };
                
                // テキストの描画位置を計算
                let time_font_size = 72.0;
                let state_font_size = 24.0;
                let spacing = 20.0;
                
                // フォントサイズから大体の高さを推定（正確ではないが、中央配置には十分）
                let time_height_est = time_font_size * 1.2;
                let state_height_est = state_font_size * 1.2;
                let total_height = time_height_est + spacing + state_height_est;
                let center_y = button_rect.center().y;
                
                let time_pos = egui::pos2(
                    button_rect.center().x,
                    center_y - total_height / 2.0 + time_height_est / 2.0,
                );
                let state_pos = egui::pos2(
                    button_rect.center().x,
                    center_y + total_height / 2.0 - state_height_est / 2.0,
                );
                
                // テキストを描画（painterで直接描画することでクリックをブロックしない）
                ui.painter().text(
                    time_pos,
                    egui::Align2::CENTER_CENTER,
                    time_text,
                    egui::FontId::proportional(time_font_size),
                    egui::Color32::WHITE,
                );
                ui.painter().text(
                    state_pos,
                    egui::Align2::CENTER_CENTER,
                    state_text,
                    egui::FontId::proportional(state_font_size),
                    egui::Color32::WHITE,
                );
                
                if button_response.clicked() {
                    match self.state {
                        TimerState::Waiting => {
                            self.state = TimerState::Working;
                            self.remaining_seconds = self.settings.work_duration_seconds;
                            self.last_update = instant::Instant::now();
                        }
                        TimerState::Working => {
                            self.state = TimerState::WorkingPaused;
                        }
                        TimerState::WorkingPaused => {
                            self.state = TimerState::Working;
                            self.last_update = instant::Instant::now();
                        }
                        TimerState::BreakFlashing => {
                            self.state = TimerState::Breaking;
                            self.remaining_seconds = self.settings.break_duration_seconds;
                            self.last_update = instant::Instant::now();
                            self.flash_phase = 0.0;
                        }
                        TimerState::Breaking => {
                            self.state = TimerState::BreakingPaused;
                        }
                        TimerState::BreakingPaused => {
                            self.state = TimerState::Breaking;
                            self.last_update = instant::Instant::now();
                        }
                        TimerState::WorkFlashing => {
                            self.state = TimerState::Working;
                            self.remaining_seconds = self.settings.work_duration_seconds;
                            self.last_update = instant::Instant::now();
                            self.flash_phase = 0.0;
                        }
                    }
                    ctx.request_repaint();
                }
            });
    }
}


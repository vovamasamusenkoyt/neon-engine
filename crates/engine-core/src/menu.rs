use glam::Vec2;

pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;

// Authentic reVC color constants (from Frontend.cpp lines 42-56)
pub const LABEL_COLOR: [f32; 3] = [255.0 / 255.0, 150.0 / 255.0, 225.0 / 255.0]; // Pastel Pink
pub const SELECTIONBORDER_COLOR: [f32; 3] = [25.0 / 255.0, 130.0 / 255.0, 70.0 / 255.0]; // Retro Green
pub const DARKMENUOPTION_COLOR: [f32; 3] = [195.0 / 255.0, 90.0 / 255.0, 165.0 / 255.0]; // Dimmed Pink
pub const SLIDERON_COLOR: [f32; 3] = [97.0 / 255.0, 194.0 / 255.0, 247.0 / 255.0]; // Cyan/Blue
pub const SLIDEROFF_COLOR: [f32; 3] = [27.0 / 255.0, 89.0 / 255.0, 130.0 / 255.0]; // Deep Blue
pub const HEADER_SHADOW_COLOR: [f32; 3] = [30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0]; // Dark Grey

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuPage {
    StartMenu, // MENUPAGE_START_MENU = 29
    NewGame,   // MENUPAGE_NEW_GAME = 1
    Options,   // MENUPAGE_OPTIONS = 27
    DisplaySettings, // MENUPAGE_DISPLAY_SETTINGS = 4
    SoundSettings,   // MENUPAGE_SOUND_SETTINGS = 3
    Stats,     // MENUPAGE_STATS = 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    ChangeMenu(MenuPage),
    GoBack,
    NewGame,
    ToggleWidescreen,
    ToggleFrameLimiter,
    Quit,
    None,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub action: MenuAction,
    pub x: f32, // Virtual X (e.g. 320 for centered)
    pub y: f32, // Virtual Y
    pub is_centered: bool,
}

pub struct CMenuManager {
    pub current_page: MenuPage,
    pub previous_page: MenuPage,
    pub selected_option: usize,
    pub items: Vec<MenuItem>,
    pub is_active: bool,
    pub brightness: f32,
    pub widescreen: bool,
    pub frame_limiter: bool,
    pub sfx_volume: f32,
    pub music_volume: f32,
}

impl Default for CMenuManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CMenuManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            current_page: MenuPage::StartMenu,
            previous_page: MenuPage::StartMenu,
            selected_option: 0,
            items: Vec::new(),
            is_active: true,
            brightness: 0.8,
            widescreen: true,
            frame_limiter: false,
            sfx_volume: 0.8,
            music_volume: 0.7,
        };
        mgr.set_page(MenuPage::StartMenu);
        mgr
    }

    // Matching reVC MenuScreens.cpp layout and line heights
    pub fn set_page(&mut self, page: MenuPage) {
        self.previous_page = self.current_page;
        self.current_page = page;
        self.selected_option = 0;
        self.items.clear();

        match page {
            // MENUPAGE_START_MENU = 29 (Frontend.cpp / MenuScreens.cpp:252)
            MenuPage::StartMenu => {
                let start_y = 180.0;
                let line_height = 40.0;

                self.items.push(MenuItem {
                    label: String::from("НАЧАТЬ ИГРУ"),
                    action: MenuAction::NewGame,
                    x: 290.0,
                    y: start_y,
                    is_centered: true,
                });
                self.items.push(MenuItem {
                    label: String::from("НАСТРОЙКИ"),
                    action: MenuAction::ChangeMenu(MenuPage::Options),
                    x: 290.0,
                    y: start_y + line_height,
                    is_centered: true,
                });
                self.items.push(MenuItem {
                    label: String::from("ВЫЙТИ ИЗ ИГРЫ"),
                    action: MenuAction::Quit,
                    x: 290.0,
                    y: start_y + line_height * 2.0,
                    is_centered: true,
                });
            }

            // MENUPAGE_NEW_GAME = 1 (MenuScreens.cpp:16)
            MenuPage::NewGame => {
                let start_y = 155.0;
                let line_height = 29.0;

                self.items.push(MenuItem {
                    label: String::from("NEW GAME"),
                    action: MenuAction::NewGame,
                    x: 320.0,
                    y: start_y,
                    is_centered: true,
                });
                self.items.push(MenuItem {
                    label: String::from("LOAD GAME"),
                    action: MenuAction::None,
                    x: 320.0,
                    y: start_y + line_height,
                    is_centered: true,
                });
                self.items.push(MenuItem {
                    label: String::from("DELETE GAME"),
                    action: MenuAction::None,
                    x: 320.0,
                    y: start_y + line_height * 2.0,
                    is_centered: true,
                });
                self.items.push(MenuItem {
                    label: String::from("BACK"),
                    action: MenuAction::GoBack,
                    x: 320.0,
                    y: start_y + line_height * 3.0,
                    is_centered: true,
                });
            }

            // MENUPAGE_OPTIONS = 27 (MenuScreens.cpp:235)
            MenuPage::Options => {
                let start_y = 153.0;
                let step = 36.6;

                self.items.push(MenuItem {
                    label: String::from("НАСТРОЙКИ ЭКРАНА"),
                    action: MenuAction::ChangeMenu(MenuPage::DisplaySettings),
                    x: 290.0,
                    y: start_y,
                    is_centered: true,
                });
                self.items.push(MenuItem {
                    label: String::from("НАСТРОЙКИ ЗВУКА"),
                    action: MenuAction::ChangeMenu(MenuPage::SoundSettings),
                    x: 290.0,
                    y: start_y + step,
                    is_centered: true,
                });
                self.items.push(MenuItem {
                    label: String::from("УПРАВЛЕНИЕ"),
                    action: MenuAction::None,
                    x: 290.0,
                    y: start_y + step * 2.0,
                    is_centered: true,
                });
                self.items.push(MenuItem {
                    label: String::from("НАЗАД"),
                    action: MenuAction::GoBack,
                    x: 290.0,
                    y: start_y + step * 3.0,
                    is_centered: true,
                });
            }

            // MENUPAGE_DISPLAY_SETTINGS = 4 (MenuScreens.cpp:46)
            MenuPage::DisplaySettings => {
                let start_y = 120.0;
                let line_height = 28.0;

                let wide_label = if self.widescreen { "WIDE SCREEN: ON" } else { "WIDE SCREEN: OFF" };
                self.items.push(MenuItem {
                    label: String::from(wide_label),
                    action: MenuAction::ToggleWidescreen,
                    x: 40.0,
                    y: start_y,
                    is_centered: false,
                });

                let limit_label = if self.frame_limiter { "FRAME LIMITER: ON" } else { "FRAME LIMITER: OFF" };
                self.items.push(MenuItem {
                    label: String::from(limit_label),
                    action: MenuAction::ToggleFrameLimiter,
                    x: 40.0,
                    y: start_y + line_height,
                    is_centered: false,
                });

                self.items.push(MenuItem {
                    label: String::from("BACK"),
                    action: MenuAction::GoBack,
                    x: 320.0,
                    y: 328.0,
                    is_centered: true,
                });
            }

            // MENUPAGE_SOUND_SETTINGS = 3 (MenuScreens.cpp:27)
            MenuPage::SoundSettings => {
                self.items.push(MenuItem {
                    label: String::from("MUSIC VOLUME"),
                    action: MenuAction::None,
                    x: 40.0,
                    y: 120.0,
                    is_centered: false,
                });
                self.items.push(MenuItem {
                    label: String::from("SFX VOLUME"),
                    action: MenuAction::None,
                    x: 40.0,
                    y: 155.0,
                    is_centered: false,
                });
                self.items.push(MenuItem {
                    label: String::from("BACK"),
                    action: MenuAction::GoBack,
                    x: 320.0,
                    y: 328.0,
                    is_centered: true,
                });
            }

            // MENUPAGE_STATS = 0 (MenuScreens.cpp:9)
            MenuPage::Stats => {
                self.items.push(MenuItem {
                    label: String::from("BACK"),
                    action: MenuAction::GoBack,
                    x: 190.0,
                    y: 340.0,
                    is_centered: false,
                });
            }
        }
    }

    pub fn nav_up(&mut self) {
        if self.items.is_empty() { return; }
        if self.selected_option == 0 {
            self.selected_option = self.items.len() - 1;
        } else {
            self.selected_option -= 1;
        }
    }

    pub fn nav_down(&mut self) {
        if self.items.is_empty() { return; }
        self.selected_option = (self.selected_option + 1) % self.items.len();
    }

    pub fn activate(&mut self) -> MenuAction {
        if self.selected_option >= self.items.len() {
            return MenuAction::None;
        }
        let action = self.items[self.selected_option].action;
        match action {
            MenuAction::ChangeMenu(page) => {
                self.set_page(page);
            }
            MenuAction::GoBack => {
                match self.current_page {
                    MenuPage::StartMenu => {
                        self.is_active = false;
                    }
                    MenuPage::NewGame | MenuPage::Options | MenuPage::Stats => {
                        self.set_page(MenuPage::StartMenu);
                    }
                    MenuPage::DisplaySettings | MenuPage::SoundSettings => {
                        self.set_page(MenuPage::Options);
                    }
                }
            }
            MenuAction::NewGame => {
                self.is_active = false;
            }
            MenuAction::ToggleWidescreen => {
                self.widescreen = !self.widescreen;
                self.set_page(MenuPage::DisplaySettings);
            }
            MenuAction::ToggleFrameLimiter => {
                self.frame_limiter = !self.frame_limiter;
                self.set_page(MenuPage::DisplaySettings);
            }
            _ => {}
        }
        action
    }

    pub fn on_cursor_move(&mut self, screen_pos: Vec2, window_size: Vec2) {
        if window_size.x <= 0.0 || window_size.y <= 0.0 || !self.is_active {
            return;
        }

        let virt_x = (screen_pos.x / window_size.x) * SCREEN_WIDTH;
        let virt_y = (screen_pos.y / window_size.y) * SCREEN_HEIGHT;

        for (idx, item) in self.items.iter().enumerate() {
            let (left, right) = if item.is_centered {
                (item.x - 200.0, item.x + 200.0)
            } else {
                (item.x - 20.0, item.x + 320.0)
            };
            let top = item.y - 20.0;
            let bottom = item.y + 20.0;

            if virt_x >= left && virt_x <= right && virt_y >= top && virt_y <= bottom {
                self.selected_option = idx;
                break;
            }
        }
    }

    pub fn handle_touch(&mut self, screen_pos: Vec2, window_size: Vec2) -> MenuAction {
        if window_size.x <= 0.0 || window_size.y <= 0.0 || !self.is_active {
            return MenuAction::None;
        }

        let virt_x = (screen_pos.x / window_size.x) * SCREEN_WIDTH;
        let virt_y = (screen_pos.y / window_size.y) * SCREEN_HEIGHT;

        for (idx, item) in self.items.iter().enumerate() {
            let (left, right) = if item.is_centered {
                (item.x - 200.0, item.x + 200.0)
            } else {
                (item.x - 20.0, item.x + 320.0)
            };
            let top = item.y - 20.0;
            let bottom = item.y + 20.0;

            if virt_x >= left && virt_x <= right && virt_y >= top && virt_y <= bottom {
                self.selected_option = idx;
                return self.activate();
            }
        }

        MenuAction::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_hover_and_touch() {
        let mut menu = CMenuManager::new();
        let win_size = Vec2::new(1280.0, 720.0);

        // Hover over НАСТРОЙКИ (center approx 580, 330 in 720p)
        menu.on_cursor_move(Vec2::new(580.0, 330.0), win_size);
        assert_eq!(menu.selected_option, 1, "Should hover on Option 1 (НАСТРОЙКИ)");

        // Hover over ВЫЙТИ ИЗ ИГРЫ (center approx 580, 390 in 720p)
        menu.on_cursor_move(Vec2::new(580.0, 390.0), win_size);
        assert_eq!(menu.selected_option, 2, "Should hover on Option 2 (ВЫЙТИ ИЗ ИГРЫ)");

        // Click on НАЧАТЬ ИГРУ (center approx 580, 270 in 720p)
        let action = menu.handle_touch(Vec2::new(580.0, 270.0), win_size);
        assert_eq!(action, MenuAction::NewGame, "Click should activate NewGame");
        assert_eq!(menu.is_active, false, "Menu should be deactivated on NewGame");

        // Tablet test (2560x1600)
        let mut tablet_menu = CMenuManager::new();
        let tablet_size = Vec2::new(2560.0, 1600.0);
        let action_opt = tablet_menu.handle_touch(Vec2::new(1160.0, 733.0), tablet_size);
        assert_eq!(action_opt, MenuAction::ChangeMenu(MenuPage::Options));
        assert_eq!(tablet_menu.current_page, MenuPage::Options);
    }
}

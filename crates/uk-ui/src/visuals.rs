use color_hex::color_from_hex;
use egui::{
    epaint::{Margin, Shadow},
    style::{Selection, Spacing, TextCursorStyle, WidgetVisuals, Widgets},
    vec2, Color32, FontFamily, LayerId, Rounding, Stroke, Style, Ui, Visuals,
};
use egui_aesthetix::Aesthetix;
use serde::{Deserialize, Serialize};

macro_rules! hex_color {
    ($hex:expr) => {{
        let _arr = color_from_hex!($hex);
        Color32::from_rgb(_arr[0], _arr[1], _arr[2])
    }};
}

pub const GREEN: Color32 = hex_color!("#528f24");
pub const BLUE: Color32 = hex_color!("#38b6f1");
pub const RED: Color32 = hex_color!("#F52331");
pub const YELLOW: Color32 = hex_color!("#ffbc28");
pub const ORANGE: Color32 = hex_color!("#ff953f");

#[inline(always)]
pub fn error_bg(visuals: &Visuals) -> Color32 {
    let mut color = egui::ecolor::Hsva::from(RED);
    color.v = egui::ecolor::Hsva::from(visuals.window_fill()).v;
    color.into()
}

/// 根据当前主题统一停靠面板、标签及分隔线的视觉层级。
pub fn style_dock(style: &egui::Style) -> egui_dock::Style {
    let mut dock_style = egui_dock::Style::from_egui(style);
    dock_style.tab.tab_body.rounding = Rounding {
        ne: 2.0,
        nw: 2.0,
        ..Default::default()
    };
    dock_style.tab.inactive.text_color = style.visuals.text_color().gamma_multiply(0.8);
    dock_style.tab.tab_body.stroke.color = style.visuals.widgets.noninteractive.bg_stroke.color;
    dock_style.tab.tab_body.bg_fill = style.visuals.panel_fill;
    dock_style.tab.active.outline_color = style.visuals.widgets.noninteractive.bg_stroke.color;
    dock_style.tab_bar.height = 34.0;
    dock_style.tab_bar.bg_fill = style.visuals.extreme_bg_color;
    dock_style.tab.minimum_width = Some(80.0);
    dock_style.tab.focused.bg_fill = style.visuals.panel_fill;
    dock_style.tab.focused.text_color = style.visuals.hyperlink_color;
    dock_style.tab.tab_body.inner_margin = Margin::same(8.0);
    dock_style.separator.width = 2.0;
    dock_style.separator.color_idle = style.visuals.widgets.noninteractive.bg_stroke.color;
    dock_style.separator.color_dragged = style.visuals.widgets.active.bg_stroke.color;
    dock_style.separator.color_hovered = style.visuals.widgets.active.bg_stroke.color;
    dock_style.dock_area_padding = Some(Margin::default());
    dock_style
}

/// 在可见区域绘制低对比度的 Sheikah 网格，避免背景干扰列表阅读。
pub fn slate_grid(ui: &mut Ui) {
    ui.with_layer_id(LayerId::background(), |ui| {
        let rect = ui.available_rect_before_wrap().intersect(ui.clip_rect());
        let painter = ui.painter().with_clip_rect(rect);
        painter.rect_filled(rect, Rounding::ZERO, ui.visuals().extreme_bg_color);
        let stroke = Stroke::new(1.0, hex_color!("#191D20"));
        for i in 0..(rect.height() / 48.0) as usize + 1 {
            painter.hline(
                rect.x_range(),
                rect.top() + 16.0 + i as f32 * 48.0,
                stroke,
            );
        }
        for i in 0..(rect.width() / 48.0) as usize + 1 {
            painter.vline(
                rect.left() + 16.0 + i as f32 * 48.0,
                rect.y_range(),
                stroke,
            );
        }
    });
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Theme {
    #[default]
    Sheikah,
    Egui,
    EguiLight,
    Frappe,
    Latte,
    Macchiato,
    Mocha,
    AdwaitaDark,
    AdwaitaLight,
    Carl,
    SweetDark,
}

impl Theme {
    #[inline]
    pub fn name(&self) -> &str {
        match self {
            Theme::Sheikah => "Sheikah Slate",
            Theme::Egui => "egui Dark",
            Theme::EguiLight => "egui Light",
            Theme::Frappe => "Frappe",
            Theme::Latte => "Latte",
            Theme::Macchiato => "Macchiato",
            Theme::Mocha => "Mocha",
            Theme::AdwaitaDark => "Adwaita Dark",
            Theme::AdwaitaLight => "Adwaita Light",
            Theme::Carl => "Carl",
            Theme::SweetDark => "Sweet Dark",
        }
    }

    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Theme::Sheikah,
            Theme::Egui,
            Theme::EguiLight,
            Theme::Frappe,
            Theme::Latte,
            Theme::Macchiato,
            Theme::Mocha,
            Theme::AdwaitaDark,
            Theme::AdwaitaLight,
            Theme::Carl,
            Theme::SweetDark,
        ]
        .into_iter()
    }

    pub fn set_theme(&self, ctx: &egui::Context) {
        match self {
            Self::Sheikah => {
                ctx.set_style(Style {
                    animation_time: 0.2,
                    visuals: Visuals {
                        dark_mode: true,
                        override_text_color: None,
                        widgets: Widgets {
                            noninteractive: WidgetVisuals {
                                bg_fill: hex_color!("#1C1E1F"),
                                bg_stroke: Stroke::new(1.0_f32, hex_color!("#393D40")),
                                fg_stroke: Stroke::new(1.0_f32, hex_color!("#D2D5D8")),
                                rounding: Rounding::same(0.0),
                                expansion: 0.0,
                                weak_bg_fill: Color32::TRANSPARENT,
                            },
                            inactive: WidgetVisuals {
                                bg_fill: hex_color!("#292C2F"),
                                bg_stroke: Stroke::new(1.0_f32, hex_color!("#484E52")),
                                fg_stroke: Stroke::new(1.0_f32, hex_color!("#f0f0f0")),
                                rounding: Rounding::same(4.0),
                                expansion: 0.0,
                                weak_bg_fill: hex_color!("#292C2F"),
                            },
                            hovered: WidgetVisuals {
                                bg_fill: hex_color!("#484E52"),
                                bg_stroke: Stroke::new(1.0_f32, hex_color!("#65C8F2")),
                                fg_stroke: Stroke::new(1.5_f32, hex_color!("#f0f0f0")),
                                rounding: Rounding::same(4.0),
                                expansion: 1.0,
                                weak_bg_fill: hex_color!("#353A3E"),
                            },
                            active: WidgetVisuals {
                                bg_fill: hex_color!("#1B4B65"),
                                bg_stroke: Stroke::new(1.0_f32, hex_color!("#484E52")),
                                fg_stroke: Stroke::new(1.5_f32, hex_color!("#D9EEFF")),
                                rounding: Rounding::same(4.0),
                                expansion: 1.0,
                                weak_bg_fill: hex_color!("#1B4B65"),
                            },
                            open: WidgetVisuals {
                                bg_fill: hex_color!("#1C1E1F"),
                                bg_stroke: Stroke::new(1.0_f32, hex_color!("#393D40")),
                                fg_stroke: Stroke::new(1.0_f32, hex_color!("#D9EEFF")),
                                rounding: Rounding::same(4.0),
                                expansion: 0.0,
                                weak_bg_fill: Color32::TRANSPARENT,
                            },
                        },
                        selection: Selection {
                            bg_fill: hex_color!("#205A78"),
                            stroke:  Stroke::new(1.0_f32, Color32::WHITE),
                        },
                        hyperlink_color: BLUE,
                        faint_bg_color: hex_color!("#252729"),
                        extreme_bg_color: hex_color!("#111314"), // e.g. TextEdit background
                        code_bg_color: Color32::from_gray(32),
                        warn_fg_color: ORANGE, // orange
                        error_fg_color: RED,    // red
                        window_rounding: Rounding::same(4.0),
                        window_shadow: Shadow {
                            offset: egui::Vec2::new(0., 6.),
                            blur:   18.,
                            spread: 0.,
                            color:  Color32::from_black_alpha(45),
                        },
                        popup_shadow: Shadow {
                            offset: egui::Vec2::new(0., 6.),
                            blur:   18.,
                            spread: 0.,
                            color:  Color32::from_black_alpha(45),
                        },
                        window_fill: hex_color!("#252728"),
                        window_stroke: Stroke::new(1.0, hex_color!("#515456")),
                        panel_fill: hex_color!("#1C1E1F"),
                        resize_corner_size: 8.0,
                        text_cursor: TextCursorStyle {
                            preview: false,
                            ..Default::default()
                        },
                        clip_rect_margin: 3.0, /* should be at least half the size of the widest
                                                * frame stroke
                                                * + max WidgetVisuals::expansion */
                        button_frame: true,
                        collapsing_header_frame: false,
                        ..Default::default()
                    },
                    spacing: Spacing {
                        item_spacing: vec2(8.0, 6.0),
                        button_padding: vec2(10.0, 6.0),
                        interact_size: vec2(40.0, 28.0),
                        icon_spacing: 6.0,
                        menu_margin: Margin::same(8.0),
                        indent_ends_with_horizontal_line: false,
                        ..Default::default()
                    },
                    text_styles: {
                        let mut styles = egui::style::default_text_styles();
                        styles.get_mut(&egui::TextStyle::Heading).unwrap().family =
                            FontFamily::Name("Bold".into());
                        styles
                    },
                    ..Default::default()
                });
            }
            Self::Egui => {
                ctx.set_visuals(egui::style::Visuals::dark());
            }
            Self::EguiLight => {
                ctx.set_visuals(egui::style::Visuals::light());
            }
            Self::Frappe => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);
            }
            Self::Latte => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE);
            }
            Self::Macchiato => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
            }
            Self::Mocha => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
            }
            Self::AdwaitaDark => {
                ctx.set_style(egui_aesthetix::themes::StandardDark.custom_style());
            }
            Self::AdwaitaLight => {
                ctx.set_style(egui_aesthetix::themes::StandardLight.custom_style());
            }
            Self::Carl => {
                ctx.set_style(egui_aesthetix::themes::CarlDark.custom_style());
            }
            Self::SweetDark => {
                ctx.set_style(Style {
                    visuals: Visuals {
                        dark_mode: true,
                        override_text_color: None,
                        widgets: Widgets {
                            noninteractive: WidgetVisuals {
                                weak_bg_fill: hex_color!("#181B28"),
                                bg_fill: hex_color!("#181B28"),
                                bg_stroke: Stroke::new(1.0_f32, hex_color!("#2F3B51")), // separators, indentation lines
                                fg_stroke: Stroke::new(1.0_f32, hex_color!("#EEEEEE")), // normal text color
                                rounding: Rounding::same(2.0),
                                expansion: 0.0,
                            },
                            inactive: WidgetVisuals {
                                weak_bg_fill: hex_color!("#1B1E2D"), // button background
                                bg_fill: hex_color!("#303651"),      // checkbox background
                                bg_stroke: Stroke {
                                    color: hex_color!("#12141e"),
                                    width: 1.0,
                                },
                                fg_stroke: Stroke::new(1.0_f32, hex_color!("#fefefe")), // button text
                                rounding: Rounding::same(2.0),
                                expansion: 0.0,
                            },
                            hovered: WidgetVisuals {
                                weak_bg_fill: hex_color!("#262C45"),
                                bg_fill: hex_color!("#262C45"),
                                bg_stroke: Stroke::new(1.0_f32, hex_color!("#71f79f")), // e.g. hover over window edge or button
                                fg_stroke: Stroke::new(1.5_f32, Color32::from_gray(240)),
                                rounding: Rounding::same(3.0),
                                expansion: 0.5,
                            },
                            active: WidgetVisuals {
                                weak_bg_fill: hex_color!("#31363D"),
                                bg_fill: hex_color!("#31363D"),
                                bg_stroke: Stroke::new(1.0_f32, Color32::WHITE),
                                fg_stroke: Stroke::new(2.0_f32, Color32::WHITE),
                                rounding: Rounding::same(2.0),
                                expansion: 0.5,
                            },
                            open: WidgetVisuals {
                                weak_bg_fill: hex_color!("#262C45"),
                                bg_fill: hex_color!("#c74ded"),
                                bg_stroke: Stroke::new(1.0_f32, Color32::from_gray(60)),
                                fg_stroke: Stroke::new(1.0_f32, Color32::from_gray(210)),
                                rounding: Rounding::same(2.0),
                                expansion: 0.0,
                            },
                        },
                        selection: Selection {
                            bg_fill: hex_color!("#c74ded"),
                            stroke: Stroke {
                                color: Color32::WHITE,
                                width: 1.0,
                            },
                        },
                        hyperlink_color: hex_color!("#c74ded"),
                        faint_bg_color: hex_color!("#161925"), // visible, but barely so
                        extreme_bg_color: hex_color!("#181B21"), // e.g. TextEdit background
                        code_bg_color: hex_color!("#0C0E15"),
                        warn_fg_color: hex_color!("#ff6a00"), // orange
                        error_fg_color: hex_color!("#ed254e"), // red
                        window_rounding: Rounding::same(4.0),
                        window_shadow: Shadow {
                            offset: vec2(10.0, 20.0),
                            blur: 15.0,
                            spread: 0.0,
                            color: Color32::from_black_alpha(96),
                        },
                        window_fill: hex_color!("#181B28"),
                        window_stroke: Stroke::new(1.0_f32, Color32::from_gray(60)),
                        window_highlight_topmost: true,
                        menu_rounding: Rounding::same(6.0),
                        panel_fill: hex_color!("#181B28"),
                        popup_shadow: Shadow {
                            offset: vec2(6.0, 10.0),
                            blur: 8.0,
                            spread: 0.0,
                            color: Color32::from_black_alpha(96),
                        },
                        resize_corner_size: 12.0,
                        text_cursor: TextCursorStyle {
                            preview: false,
                            stroke: Stroke::new(2.0_f32, Color32::from_rgb(192, 222, 255)),
                            ..Default::default()
                        },
                        clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
                        button_frame: true,
                        collapsing_header_frame: false,
                        indent_has_left_vline: true,
                        striped: false,
                        slider_trailing_fill: false,
                        handle_shape: egui::style::HandleShape::Circle,
                        interact_cursor: None,
                        image_loading_spinners: true,
                        numeric_color_space: egui::style::NumericColorSpace::GammaByte,
                    },
                    spacing: Spacing {
                        item_spacing: vec2(8.0, 4.0),
                        window_margin: Margin::same(8.0),
                        menu_margin: Margin::same(8.0),
                        button_padding: vec2(8.0, 4.0),
                        indent: 28.0, // match checkbox/radio-button with `button_padding.x + icon_width + icon_spacing`
                        interact_size: vec2(48.0, 20.0),
                        slider_width: 100.0,
                        slider_rail_height: 8.0,
                        combo_width: 100.0,
                        text_edit_width: 280.0,
                        icon_width: 16.0,
                        icon_width_inner: 10.0,
                        icon_spacing: 6.0,
                        tooltip_width: 600.0,
                        menu_width: 160.0,
                        menu_spacing: 4.0,
                        combo_height: 200.0,
                        scroll: Default::default(),
                        indent_ends_with_horizontal_line: false,
                        default_area_size: vec2(600.0, 400.0)
                    },
                    ..Default::default()
                });
            }
        }
    }
}

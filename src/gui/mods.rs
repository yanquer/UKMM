use std::{collections::HashMap, process::Command};
use serde::{Deserialize, Serialize};
use strfmt::Format;
use uk_localization::string_ext::LocString;
use uk_manager::mods::Mod;
use uk_ui::{
    egui::{
        self, epaint::Margin, Align, Button, Color32, CursorIcon, Id, Key,
        LayerId, Layout, Response, Sense, TextStyle, Ui, Vec2,
    },
    egui_extras::{Column, TableBuilder, TableRow},
    ext::UiExt,
};

use super::{App, FocusedPane, Message, Sort};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModListState {
    pub sort: Sort,
    pub descending: bool,
}

impl Default for ModListState {
    fn default() -> Self {
        Self {
            sort: Sort::Priority,
            descending: false,
        }
    }
}

enum ContextMenuMessage {
    CopyToProfile(smartstring::alias::String),
    Extract,
    Update,
    DevUpdate,
    Uninstall,
    Toggle(bool),
    Move(usize),
}

impl App {
    pub fn render_modlist(&mut self, ui: &mut Ui) {
        let text_height = (ui.text_style_height(&TextStyle::Body) + 12.0)
            .max(ui.spacing().interact_size.y);
        let icon_width = ui.spacing().icon_width + ui.spacing().button_padding.x;
        let numeric_col_width = 72.0;
        let category_width = (ui.available_width() * 0.2).clamp(72.0, 144.0);
        let filtered = self.filtered_mods(&self.mod_list_filter)
            .iter().map(|(index, _)| *index).collect::<Vec<_>>();

        if filtered.is_empty() {
            self.render_modlist_empty(ui);
            return;
        }

        egui::Frame::none()
            .inner_margin(Margin {
                bottom: 4.0,
                top:    4.0,
                left:   4.0,
                right:  4.0,
            })
            .show(ui, |ui| {
                ui.style_mut()
                    .visuals
                    .widgets
                    .noninteractive
                    .fg_stroke
                    .color = ui.style().visuals.strong_text_color();
                // 显式按当前面板计算列宽，避免表头占满单元格后缓存旧宽度，阻止窗口缩窄。
                let name_width = (ui.available_width()
                    - icon_width
                    - category_width
                    - numeric_col_width * 2.0
                    - ui.spacing().item_spacing.x * 4.0
                    - ui.spacing().scroll.allocated_width())
                    .max(40.0);
                TableBuilder::new(ui)
                    .striped(true)
                    .sense(Sense::click_and_drag())
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .column(Column::exact(icon_width))
                    .column(Column::exact(name_width))
                    .column(Column::exact(category_width).clip(true))
                    .columns(Column::exact(numeric_col_width).clip(true), 2)
                    .header(text_height, |mut header| {
                        header.col(|ui| {
                            let is_current = self.list_state.sort == Sort::Enabled;
                            let label = if is_current {
                                if self.list_state.descending { "⏷" } else { "⏶" }
                            } else {
                                "  "
                            };
                            ui.style_mut().visuals.widgets.inactive.bg_stroke.width = 0.0;
                            if ui
                                .add(Button::new(label).small().fill(Color32::TRANSPARENT))
                                .clicked()
                            {
                                self.do_update(Message::ChangeSort(
                                    Sort::Enabled,
                                    if is_current {
                                        !self.list_state.descending
                                    } else {
                                        self.list_state.descending
                                    },
                                ));
                            }
                        });
                        [
                            ("Info_Name".localize(), Sort::Name),
                            ("Info_Category".localize(), Sort::Category),
                            ("Info_Version".localize(), Sort::Version),
                            ("Info_Priority".localize(), Sort::Priority),
                        ]
                        .into_iter()
                        .for_each(|(label, sort)| {
                            header
                                .col(|ui| {
                                    let is_current = self.list_state.sort == sort;
                                    let mut label = label.into_owned();
                                    if is_current {
                                        if self.list_state.descending {
                                            label += " ⏷";
                                        } else {
                                            label += " ⏶";
                                        }
                                    } else {
                                        label += "  ";
                                    }
                                    ui.centered_and_justified(|ui| {
                                        ui.style_mut()
                                            .visuals
                                            .widgets
                                            .inactive
                                            .bg_stroke
                                            .width = 0.0;
                                        if ui.add(Button::new(label)
                                            .small()
                                            .fill(Color32::TRANSPARENT))
                                            .clicked()
                                        {
                                            self.do_update(Message::ChangeSort(
                                                sort,
                                                if is_current {
                                                    !self.list_state.descending
                                                } else {
                                                    self.list_state.descending
                                                },
                                            ));
                                        }
                                    });
                                });
                        });
                    })
                    .body(|body| {
                        body.rows(text_height, filtered.len(), |row| {
                            let index = filtered[row.index()];
                            self.render_mod_row(index, row);
                        });
                    });
            });
        if ui.memory(|m| m.focused().is_some())
            && self.focused == FocusedPane::ModList
            && !self.modal_open()
        {
            if let Some((last_index, _)) = ui
                .input(|i| i.key_pressed(Key::ArrowDown))
                .then(|| {
                    self.mods
                        .iter()
                        .enumerate()
                        .rfind(|(_, m)| self.selected.contains(m))
                })
                .flatten()
            {
                if !ui.input(|i| i.modifiers.shift) {
                    self.do_update(Message::SelectOnly(last_index + 1));
                } else {
                    self.do_update(Message::SelectAlso(last_index + 1));
                }
            } else if let Some((first_index, _)) = ui
                .input(|i| i.key_pressed(Key::ArrowUp))
                .then(|| {
                    self.mods
                        .iter()
                        .enumerate()
                        .find(|(_, m)| self.selected.contains(m))
                })
                .flatten()
            {
                let index = first_index.max(1);
                if !ui.input(|i| i.modifiers.shift) {
                    self.do_update(Message::SelectOnly(index - 1));
                } else {
                    self.do_update(Message::SelectAlso(index - 1));
                }
            }
        }
        self.render_drag_state(text_height, icon_width, numeric_col_width, ui);
        if ui.input_mut(|i| i.pointer.any_released()) {
            ui.memory_mut(|m| m.data.insert_temp(Id::new("drag_delay_frames"), 0usize));
            if let Some((_start_index, dest_index)) = self
                .drag_index.zip(self.hover_index)
                .filter(|(s, d)| s != d)
            {
                self.do_update(Message::MoveSelected(dest_index))
            } else {
                self.do_update(Message::ClearDrag);
            }
            ui.output_mut(|o| o.cursor_icon = CursorIcon::Default);
        }
    }

    /// 区分首次配置、空模组库和筛选无结果，并提供对应的下一步操作。
    fn render_modlist_empty(&mut self, ui: &mut Ui) {
        let has_mods = !self.displayed_mods.is_empty();
        let has_config = self.core.settings().platform_config().is_some();
        egui::Frame::none().inner_margin(24.0).show(ui, |ui| {
            ui.add_space((ui.available_height() * 0.22).clamp(12.0, 80.0));
            ui.vertical_centered(|ui| {
                ui.set_max_width(440.0_f32.min(ui.available_width()));
                ui.add(
                    egui::Image::new(uk_ui::icons::get_icon(
                        ui.ctx(),
                        if has_mods {
                            uk_ui::icons::Icon::List
                        } else if has_config {
                            uk_ui::icons::Icon::Import
                        } else {
                            uk_ui::icons::Icon::Settings
                        },
                    ))
                    .fit_to_exact_size(Vec2::splat(36.0))
                    .tint(ui.visuals().hyperlink_color),
                );
                ui.add_space(12.0);
                let title = if has_mods {
                    "Mod_Filter_Empty"
                } else if has_config {
                    "Mod_Empty_Title"
                } else {
                    "Mod_Setup_Title"
                };
                ui.label(egui::RichText::new(title.localize()).size(18.0).strong());
                ui.add_space(8.0);
                let body = if has_mods {
                    "Mod_Filter_Empty_Hint".localize().into_owned()
                } else if has_config {
                    "Helper_Modlist_NoMods_Hint".localize().into_owned()
                } else {
                    let vars = HashMap::from([(
                        "platform".to_string(),
                        self.core.settings().current_mode.to_string(),
                    )]);
                    "Helper_Modlist_NoConfig".localize().format(&vars).unwrap()
                };
                ui.add(egui::Label::new(body).wrap());
                ui.add_space(16.0);
                if has_mods {
                    if ui.button("Mod_Filter_Clear".localize()).clicked() {
                        self.mod_list_filter.clear();
                    }
                } else if has_config {
                    if ui.button("Menu_File_Open".localize()).clicked() {
                        self.do_update(Message::SelectFile);
                    }
                } else if ui.button("Tab_Settings".localize()).clicked() {
                    self.do_update(Message::FocusTab(super::Tabs::Settings));
                }
            });
        });
    }
    fn render_drag_state(
        &mut self,
        text_height: f32,
        icon_width: f32,
        numeric_col_width: f32,
        ui: &mut Ui,
    ) {
        let being_dragged = ui.ctx().dragged_id().is_some();
        let delay_id = Id::new("drag_delay_frames");
        let mut delay_frames: usize = ui.memory_mut(|m| *m.data.get_temp_mut_or_default(delay_id));
        if being_dragged {
            if delay_frames < 6 {
                delay_frames += 1;
            } else if let Some(drag_index) = self.drag_index {
                ui.output_mut(|o| o.cursor_icon = CursorIcon::Grabbing);
                let layer_id =
                    LayerId::new(egui::Order::Tooltip, Id::new("mod_list").with(drag_index));
                let res = ui
                    .with_layer_id(layer_id, |ui| {
                        TableBuilder::new(ui)
                            .column(Column::exact(icon_width))
                            .column(Column::remainder())
                            .column(Column::initial(80.).at_least(16.).at_most(260.))
                            .column(Column::exact(numeric_col_width))
                            .column(Column::exact(numeric_col_width))
                            .body(|body| {
                                body.rows(text_height, self.selected.len(), |mut row| {
                                    let mod_ = &self.selected[row.index()];
                                    let mut enabled = mod_.enabled;
                                    row.col(|ui| {
                                        ui.checkbox(&mut enabled, "");
                                    });
                                    for label in [
                                        mod_.meta.name.as_str(),
                                        mod_.meta.category.to_loc_str().localize().as_ref(),
                                        mod_.meta.version.to_string().as_str(),
                                        self.mods
                                            .iter()
                                            .position(|m| m == mod_)
                                            .unwrap()
                                            .to_string()
                                            .as_str(),
                                    ] {
                                        row.col(|ui| {
                                            ui.label(label);
                                        });
                                    }
                                });
                            });
                    })
                    .response;
                if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                    let delta = pointer_pos.y - res.rect.center().y;
                    #[allow(deprecated)]
                    ui.ctx().translate_layer(layer_id, Vec2::new(0.0, delta));
                }
            }
        } else {
            delay_frames = 0;
        }
        ui.memory_mut(|m| m.data.insert_temp(delay_id, delay_frames));
    }

    fn render_mod_row(&mut self, index: usize, mut row: TableRow) {
        let mod_ = unsafe { self.displayed_mods.get_mut(index).unwrap_unchecked() };
        if let Some(index) = self.mods.iter().position(|m| m == mod_) {
            let selected = self.selected.contains(mod_);
            let mut clicked = false;
            let mut drag_started = false;
            let mut ctrl = false;
            let mut shift = false;
            let mut hover = false;
            let mut toggled = false;
            let mut ctx_action = None;
            let menu_mod = mod_.clone();

            let mut process_col_res = |res: Response| {
                clicked = clicked || res.clicked();
                hover = hover || res.hovered();
                drag_started = drag_started || res.drag_started();
                res.context_menu(|ui| {
                    if let Some(action) =
                        Self::render_mod_context_menu(&self.core, menu_mod.clone(), ui)
                    {
                        ctx_action.replace(action);
                    }
                });
            };

            if selected {
                row.set_selected(true);
            }
            let mut enabled = mod_.enabled;
            process_col_res(
                row.col(|ui| {
                    toggled = ui.checkbox(&mut enabled, "").clicked();
                    shift = ui.input(|i| i.modifiers.shift);
                    ctrl = ui.input(|i| i.modifiers.ctrl);
                })
                .1,
            );
            process_col_res(
                row.col(|ui| {
                    ui.clipped_label(mod_.meta.name.as_str())
                        .on_hover_text(mod_.meta.name.as_str());
                })
                .1,
            );
            for label in [
                mod_.meta.category.to_loc_str().localize().as_ref(),
                mod_.meta.version.to_string().as_str(),
                index.to_string().as_str(),
            ] {
                process_col_res(
                    row.col(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label(label);
                        });
                    })
                    .1,
                );
            }
            if let Some(action) = ctx_action {
                match action {
                    ContextMenuMessage::CopyToProfile(profile) => {
                        self.do_update(Message::AddToProfile(profile));
                    }
                    ContextMenuMessage::Extract => {
                        self.do_update(Message::Extract);
                    }
                    ContextMenuMessage::Update => self.do_update(Message::ModUpdate),
                    ContextMenuMessage::DevUpdate => {
                        self.do_update(Message::DevUpdate);
                    }
                    ContextMenuMessage::Uninstall => {
                        let message = "Mod_Uninstall_Confirmation".localize();
                        let vars = HashMap::from(
                            [("mod_name".to_string(), mod_.meta.name.to_string())]
                        );
                        let prompt = message.format(&vars).unwrap();
                        self.do_update(Message::Confirm(
                            Message::UninstallMods(None).into(),
                            prompt,
                        ));
                    }
                    ContextMenuMessage::Toggle(state) => {
                        self.do_update(Message::ToggleMods(None, state));
                    }
                    ContextMenuMessage::Move(dest) => {
                        self.do_update(Message::MoveSelected(dest));
                    }
                }
            }
            if toggled {
                self.do_update(Message::ToggleMods(Some(vec![menu_mod.clone()]), enabled));
            } else if clicked {
                self.do_update(Message::SetFocus(FocusedPane::ModList));
                if selected && ctrl {
                    self.do_update(Message::Deselect(index));
                } else if shift {
                    self.do_update(Message::SelectThrough(index));
                } else if ctrl {
                    self.do_update(Message::SelectAlso(index));
                } else {
                    self.do_update(Message::SelectOnly(index));
                }
            } else if drag_started {
                if !ctrl && !shift {
                    self.do_update(Message::StartDrag(index));
                }
            } else if self.drag_index != Some(index) && hover {
                self.hover_index = Some(index);
            }
        }
    }

    fn render_mod_context_menu(
        core: &uk_manager::core::Manager,
        mod_: Mod,
        ui: &mut Ui,
    ) -> Option<ContextMenuMessage> {
        let mut result = None;
        ui.menu_button("Mod_Send".localize(), |ui| {
            for profile in core
                .settings()
                .profiles()
                .filter(|p| core.mod_manager().profile().key() != p)
            {
                if ui.button(profile.as_str()).clicked() {
                    result = Some(ContextMenuMessage::CopyToProfile(profile));
                    ui.close_menu();
                }
            }
        });
        if ui.button("Generic_Update".localize()).clicked() {
            ui.close_menu();
            result = Some(ContextMenuMessage::Update);
        }
        if ui.button("Mod_Dev_Update".localize()).clicked() {
            ui.close_menu();
            result = Some(ContextMenuMessage::DevUpdate);
        }
        if ui.button("Mod_Uninstall".localize()).clicked() {
            ui.close_menu();
            result = Some(ContextMenuMessage::Uninstall);
        }
        if ui
            .button(if mod_.enabled {
                "Mod_Disable".localize()
            } else {
                "Mod_Enable".localize()
            })
            .clicked()
        {
            ui.close_menu();
            result = Some(ContextMenuMessage::Toggle(!mod_.enabled));
        }
        if ui.button("Mod_View".localize()).clicked() {
            ui.close_menu();
            let _ = Command::new(if cfg!(windows) {
                "explorer"
            } else {
                "xdg-open"
            })
            .arg(if mod_.path.is_dir() {
                &mod_.path
            } else {
                mod_.path.parent().unwrap()
            })
            .output();
        }
        if ui.button("Mod_Extract".localize()).clicked() {
            ui.close_menu();
            result = Some(ContextMenuMessage::Extract);
        }
        if ui.button("Mod_Move_Start".localize()).clicked() {
            ui.close_menu();
            result = Some(ContextMenuMessage::Move(0));
        }
        if ui.button("Mod_Move_End".localize()).clicked() {
            ui.close_menu();
            result = Some(ContextMenuMessage::Move(9999));
        }
        result
    }
}

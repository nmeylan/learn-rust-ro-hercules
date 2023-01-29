use std::collections::{HashMap};
use std::sync::Arc;
use eframe::egui::{Color32, emath, epaint, Frame, Pos2, Rect, Sense, Shape, Stroke, Ui};
use eframe::egui::epaint::RectShape;
use crate::server::core::map::{WALKABLE_MASK, WARP_MASK};
use crate::server::core::map_instance::MapInstance;
use crate::server::core::map_item::{MapItem, MapItemType};
use crate::server::{Server};
use crate::util::coordinate;

pub struct MapInstanceView {
    pub cursor_pos: Option<Pos2>,
    pub zoom: f32,
    pub zoom_center: Pos2,
    pub zoom_draw_rect: Rect,
    pub server: Arc<Server>
}

struct PreviousCell {
    pub color: Color32,
    pub pos: Pos2,
}

impl MapInstanceView {
    pub fn draw_map_instance_view(&mut self, ui: &mut Ui, map_instance: &Arc<MapInstance>, map_items: HashMap<u32, MapItem>) {
        Frame::dark_canvas(ui.style()).show(ui, |ui| {
            let (_id, response) = ui.allocate_exact_size(ui.available_size_before_wrap(), Sense::click_and_drag());
            let absolute_draw_rect = response.rect;
            let relative_draw_rect = Rect {
                min: Pos2 {
                    x: 0.0,
                    y: 0.0
                },
                max: Pos2 {
                    x: absolute_draw_rect.max.x - absolute_draw_rect.min.x,
                    y: absolute_draw_rect.max.y - absolute_draw_rect.min.y,
                }
            };
            if self.zoom_draw_rect.max.x == 0.0 {
                self.zoom_draw_rect = relative_draw_rect.clone();
            }
            let mut shapes = vec![];
            let margin = 0.0;

            let cursor_pos = ui.input().pointer.hover_pos();
            self.cursor_pos = None;
            if ui.input().zoom_delta() > 1.0 {
                self.zoom_center = Pos2 {
                    x: cursor_pos.as_ref().unwrap().x - absolute_draw_rect.min.x,
                    y: cursor_pos.as_ref().unwrap().y - absolute_draw_rect.min.y,
                };
                self.zoom *= 1.5;
            } else if ui.input().zoom_delta() < 1.0 {
                self.zoom /= 1.5;
                if self.zoom <= 1.0 {
                    self.zoom = 1.0;
                    self.zoom_center = Pos2 { x: 0.0, y: 0.0 };
                    self.zoom_draw_rect = relative_draw_rect.clone();
                }
            }

            let mut shape_x_size = (relative_draw_rect.max.x - (margin * 2.0) as f32) / map_instance.x_size() as f32;
            let mut shape_y_size = (relative_draw_rect.max.y - (margin * 2.0) as f32) / map_instance.y_size() as f32;

            self.zoom_draw_rect.min.x = self.zoom_center.x + (relative_draw_rect.min.x - self.zoom_center.x) / (self.zoom as f32);
            self.zoom_draw_rect.max.x = self.zoom_center.x + (relative_draw_rect.max.x - self.zoom_center.x) / (self.zoom as f32);
            self.zoom_draw_rect.min.y = self.zoom_center.y - self.zoom_center.y / (self.zoom as f32);
            self.zoom_draw_rect.max.y = self.zoom_center.y + (relative_draw_rect.max.y - self.zoom_center.y) / (self.zoom as f32);


            let start_j = ((self.zoom_draw_rect.min.x) / shape_x_size) as u16;
            let mut end_j = ((self.zoom_draw_rect.max.x) / shape_x_size) as u16;
            if end_j > map_instance.x_size() {
                end_j = map_instance.x_size()
            }
            let mut end_i = ((relative_draw_rect.max.y - self.zoom_draw_rect.min.y) / shape_y_size) as u16;
            let start_i = ((relative_draw_rect.max.y - self.zoom_draw_rect.max.y) / shape_y_size) as u16;
            if end_i > map_instance.y_size() {
                end_i = map_instance.y_size()
            }

            shape_x_size = shape_x_size * self.zoom;
            shape_y_size = shape_y_size * self.zoom;

            let mut previous_cell: Option<PreviousCell> = None;

            for i in start_i..end_i {
                let mut cell_color;
                for j in start_j..end_j {
                    let index = coordinate::get_cell_index_of(j, i, map_instance.x_size());
                    let state = map_instance.state();
                    let cell = state.cells().get(index).unwrap();
                    let cell_pos = emath::Rect {
                        min: Pos2 {
                            x: absolute_draw_rect.min.x + margin + (shape_x_size * (j - start_j) as f32),
                            y: absolute_draw_rect.max.y - margin - (shape_y_size * ((i - start_i) as f32 + 1.0))
                        },
                        max: Pos2 {
                            x: absolute_draw_rect.min.x + margin + (shape_x_size * ((j - start_j) as f32 + 1.0)),
                            y: absolute_draw_rect.max.y - margin - (shape_y_size * (i - start_i) as f32)
                        }
                    };
                    cell_color = Color32::BLACK;
                    if cell & WARP_MASK == WARP_MASK {
                        cell_color = Color32::BLUE;
                    } else if cell & WALKABLE_MASK == WALKABLE_MASK {
                        cell_color = Color32::WHITE;
                    }

                    if cursor_pos.is_some() {
                        let cursor_pos = cursor_pos.unwrap();
                        if cell_pos.min.x < cursor_pos.x && cell_pos.max.x > cursor_pos.x
                            && cell_pos.min.y < cursor_pos.y && cell_pos.max.y > cursor_pos.y {
                            self.cursor_pos = Some(Pos2 {
                                x: j as f32,
                                y: i as f32
                            });
                        }
                    }


                    if previous_cell.is_none() {
                        previous_cell = Some(PreviousCell {
                            color: cell_color,
                            pos: Pos2 {
                                x: (j - start_j) as f32,
                                y: (i - start_i) as f32
                            }
                        })
                    } else if previous_cell.as_ref().unwrap().color != cell_color {
                        MapInstanceView::draw_cell(&mut shapes, &mut previous_cell, &absolute_draw_rect, margin, shape_x_size, shape_y_size, start_i, i, start_j, j);
                        previous_cell = Some(PreviousCell {
                            color: cell_color,
                            pos: Pos2 {
                                x: (j - start_j) as f32,
                                y: (i - start_i) as f32
                            }
                        })
                    }
                }
                MapInstanceView::draw_cell(&mut shapes, &mut previous_cell, &absolute_draw_rect, margin, shape_x_size, shape_y_size, start_i, i, start_j, end_j);
                previous_cell = None;
            }
            for (_, map_item) in map_items.iter() {
                let map_name = map_instance.name().to_string();
                let map_instance_id = map_instance.id();
                let position = self.server.state().map_item_x_y(map_item, &map_name, map_instance_id).unwrap();
                if position.x() < start_j
                    || position.y() < start_i {
                    continue;
                }
                let mut cell_color = Default::default();
                if *map_item.object_type() == MapItemType::Mob {
                    cell_color = Color32::RED;
                } else if *map_item.object_type() == MapItemType::Character {
                    cell_color = Color32::GREEN;
                } else if *map_item.object_type() == MapItemType::Warp {
                    cell_color = Color32::BLUE;
                }
                shapes.push(epaint::Shape::Rect(RectShape {
                    rect: emath::Rect {
                        min: Pos2 {
                            x: absolute_draw_rect.min.x + margin + (shape_x_size * (position.x() - start_j) as f32),
                            y: absolute_draw_rect.max.y - margin - (shape_y_size * ((position.y() - start_i) as f32 + 1.0)),
                        },
                        max: Pos2 {
                            x: absolute_draw_rect.min.x + margin + (shape_x_size * ((position.x() - start_j) as f32 + 1.0)),
                            y: absolute_draw_rect.max.y - margin - (shape_y_size * ((position.y() - start_i) as f32)),
                        }
                    },
                    fill: cell_color,
                    stroke: Stroke::NONE,
                    rounding: Default::default(),
                }));
            }


            ui.painter().extend(shapes);
            ui.ctx().request_repaint();
        });
    }

    fn draw_cell(shapes: &mut Vec<Shape>, previous_cell: &Option<PreviousCell>, absolute_draw_rect: &emath::Rect,
                 margin: f32, shape_x_size: f32, shape_y_size: f32, start_i: u16, i: u16, start_j: u16, j: u16) {
        if previous_cell.as_ref().unwrap().color != Color32::BLACK {
            shapes.push(epaint::Shape::Rect(RectShape {
                rect: emath::Rect {
                    min: Pos2 {
                        x: absolute_draw_rect.min.x + margin + (shape_x_size * previous_cell.as_ref().unwrap().pos.x as f32),
                        y: absolute_draw_rect.max.y - margin - (shape_y_size * (previous_cell.as_ref().unwrap().pos.y as f32 + 1.0)),
                    },
                    max: Pos2 {
                        x: absolute_draw_rect.min.x + margin + (shape_x_size * (j - start_j) as f32 + 1.0),
                        y: absolute_draw_rect.max.y - margin - (shape_y_size * ((i - start_i) as f32)),
                    }
                },
                rounding: Default::default(),
                fill: previous_cell.as_ref().unwrap().color,
                stroke: Stroke::NONE
            }));
        }
    }
}
use super::brush_definition::BrushDefinition;
use anyhow::Result;
use std::collections::HashMap;

pub struct BrushManager {
    pub brushes: HashMap<String, BrushDefinition>,
    pub active_brush_name: String,
}

impl BrushManager {
    pub fn new() -> Self {
        BrushManager {
            brushes: HashMap::new(),
            active_brush_name: String::new(),
        }
    }

    pub fn load_default(&mut self) -> Result<()> {
        let circle_texture = include_bytes!("./circle-texture.png");
        let brush_circle = BrushDefinition::from_bytes("Circle", circle_texture)?;

        self.brushes.insert(brush_circle.get_name(), brush_circle);

        if let Some(name) = self.brushes.keys().next() {
            self.active_brush_name = name.clone();
        }

        Ok(())
    }

    pub fn get_active_brush_mut(&mut self) -> Option<&mut BrushDefinition> {
        self.brushes.get_mut(&self.active_brush_name)
    }
}

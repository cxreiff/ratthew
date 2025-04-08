use std::fmt::Display;

use bevy::math::IVec2;
use bevy_ecs_ldtk::{
    ldtk::{self, LayerInstance, TileInstance},
    EntityInstance,
};

#[derive(Debug, Clone)]
pub struct LayerData {
    pub _name: String,
    pub sprite_size: IVec2,
    pub variant: LayerVariant,
}

#[derive(Debug, Clone)]
pub enum LayerVariant {
    Particles(Vec<EntityInstance>),
    Walls(Vec<TileInstance>),
    Ramps(Vec<EntityInstance>),
}

impl TryFrom<&LayerInstance> for LayerData {
    type Error = ParseLayerError;

    fn try_from(value: &LayerInstance) -> Result<Self, Self::Error> {
        let (variant_str, name_str) = value
            .identifier
            .split_once('_')
            .ok_or(ParseLayerError::Title(value.iid.clone()))?;

        let name = name_str.to_string();

        let sprite_size = IVec2::new(value.c_wid, value.c_hei);

        let variant = match value.layer_instance_type {
            ldtk::Type::Entities => match variant_str {
                "Particles" => LayerVariant::Particles(value.entity_instances.clone()),
                "Ramps" => LayerVariant::Ramps(value.entity_instances.clone()),
                _ => return Err(ParseLayerError::Variant(variant_str.into())),
            },
            ldtk::Type::AutoLayer => match variant_str {
                "Walls" => LayerVariant::Walls(value.auto_layer_tiles.clone()),
                _ => return Err(ParseLayerError::Variant(variant_str.into())),
            },
            ldtk::Type::IntGrid => match variant_str {
                "Walls" => LayerVariant::Walls(value.auto_layer_tiles.clone()),
                _ => return Err(ParseLayerError::Variant(variant_str.into())),
            },
            ldtk::Type::Tiles => match variant_str {
                "Walls" => LayerVariant::Walls(value.grid_tiles.clone()),
                _ => return Err(ParseLayerError::Variant(variant_str.into())),
            },
        };

        Ok(Self {
            _name: name,
            variant,
            sprite_size,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ParseLayerError {
    Title(String),
    Variant(String),
}

impl Display for ParseLayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseLayerError::Title(str) => {
                write!(f, "failed to parse layer metadata from layer title: {str}",)
            }
            ParseLayerError::Variant(str) => {
                write!(f, "failed to parse layer variant: {str}",)
            }
        }
    }
}

impl std::error::Error for ParseLayerError {}

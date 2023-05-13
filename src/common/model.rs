// Copyright © 2018 Cormac O'Brien
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::common::{
    bsp::{BspFileError, BspModel},
    mdl::{self, AliasModel, MdlFileError},
    sprite::{self, SpriteModel},
    vfs::{Vfs, VfsError},
};

use cgmath::Vector3;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("BSP file error: {0}")]
    BspFile(#[from] BspFileError),
    #[error("MDL file error: {0}")]
    MdlFile(#[from] MdlFileError),
    #[error("SPR file error")]
    SprFile,
    #[error("Virtual filesystem error: {0}")]
    Vfs(#[from] VfsError),
}

#[derive(Debug, FromPrimitive)]
pub enum SyncType {
    Sync = 0,
    Rand = 1,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ModelFlags: u8 {
        const ROCKET  = 0b00000001;
        const GRENADE = 0b00000010;
        const GIB     = 0b00000100;
        const ROTATE  = 0b00001000;
        const TRACER  = 0b00010000;
        const ZOMGIB  = 0b00100000;
        const TRACER2 = 0b01000000;
        const TRACER3 = 0b10000000;
    }
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub kind: ModelKind,
    pub flags: ModelFlags,
}

#[derive(Debug)]
pub enum ModelKind {
    // TODO: find a more elegant way to express the null model
    None,
    Brush(BspModel),
    Alias(AliasModel),
    Sprite(SpriteModel),
}

impl Model {
    pub fn none() -> Model {
        Model {
            name: String::new(),
            kind: ModelKind::None,
            flags: ModelFlags::empty(),
        }
    }

    pub fn kind(&self) -> &ModelKind {
        &self.kind
    }

    pub fn load<S>(vfs: &Vfs, name: S) -> Result<Model, ModelError>
    where
        S: AsRef<str>,
    {
        let name = name.as_ref();
        // TODO: original engine uses the magic numbers of each format instead of the extension.
        if name.ends_with(".bsp") {
            panic!("BSP files may contain multiple models, use bsp::load for this");
        } else if name.ends_with(".mdl") {
            Ok(Model::from_alias_model(
                name.to_owned(),
                mdl::load(vfs.open(name)?)?,
            ))
        } else if name.ends_with(".spr") {
            Ok(Model::from_sprite_model(
                name.to_owned(),
                sprite::load(vfs.open(name)?),
            ))
        } else {
            panic!("Unrecognized model type: {}", name);
        }
    }

    /// Construct a new generic model from a brush model.
    pub fn from_brush_model<S>(name: S, brush_model: BspModel) -> Model
    where
        S: AsRef<str>,
    {
        Model {
            name: name.as_ref().to_owned(),
            kind: ModelKind::Brush(brush_model),
            flags: ModelFlags::empty(),
        }
    }

    /// Construct a new generic model from an alias model.
    pub fn from_alias_model<S>(name: S, alias_model: AliasModel) -> Model
    where
        S: AsRef<str>,
    {
        let flags = alias_model.flags();

        Model {
            name: name.as_ref().to_owned(),
            kind: ModelKind::Alias(alias_model),
            flags,
        }
    }

    /// Construct a new generic model from a sprite model.
    pub fn from_sprite_model<S>(name: S, sprite_model: SpriteModel) -> Model
    where
        S: AsRef<str>,
    {
        Model {
            name: name.as_ref().to_owned(),
            kind: ModelKind::Sprite(sprite_model),
            flags: ModelFlags::empty(),
        }
    }

    /// Return the name of this model.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the minimum extent of this model.
    pub fn min(&self) -> Vector3<f32> {
        debug!("Retrieving min of model {}", self.name);
        match self.kind {
            ModelKind::None => panic!("attempted to take min() of NULL model"),
            ModelKind::Brush(ref bmodel) => bmodel.min(),
            ModelKind::Sprite(ref smodel) => smodel.min(),

            // TODO: maybe change this?
            // https://github.com/id-Software/Quake/blob/master/WinQuake/gl_model.c#L1625
            ModelKind::Alias(_) => Vector3::new(-16.0, -16.0, -16.0),
        }
    }

    /// Return the maximum extent of this model.
    pub fn max(&self) -> Vector3<f32> {
        debug!("Retrieving max of model {}", self.name);
        match self.kind {
            ModelKind::None => panic!("attempted to take max() of NULL model"),
            ModelKind::Brush(ref bmodel) => bmodel.max(),
            ModelKind::Sprite(ref smodel) => smodel.max(),

            // TODO: maybe change this?
            // https://github.com/id-Software/Quake/blob/master/WinQuake/gl_model.c#L1625
            ModelKind::Alias(_) => Vector3::new(16.0, 16.0, 16.0),
        }
    }

    pub fn sync_type(&self) -> SyncType {
        match self.kind {
            ModelKind::None => panic!("Attempted to take sync_type() of NULL model"),
            ModelKind::Brush(_) => SyncType::Sync,
            // TODO: expose sync_type in Sprite and reflect it here
            ModelKind::Sprite(ref _smodel) => SyncType::Sync,
            // TODO: expose sync_type in Mdl and reflect it here
            ModelKind::Alias(ref _amodel) => SyncType::Sync,
        }
    }

    pub fn flags(&self) -> ModelFlags {
        self.flags
    }

    pub fn has_flag(&self, flag: ModelFlags) -> bool {
        self.flags.contains(flag)
    }
}

use std::{borrow::Cow, marker::PhantomData, rc::Rc};

use crate::{
    context::MapContext,
    environment::Environment,
    io::apc::{AsyncProcedureCall, Message},
    kernel::Kernel,
    sdf::{SymbolLayerData, SymbolLayersDataComponent},
    tcs::system::System,
    vector::transferables::*,
};

pub struct PopulateWorldSystem<E: Environment, T> {
    kernel: Rc<Kernel<E>>,
    phantom_t: PhantomData<T>,
}

impl<E: Environment, T> PopulateWorldSystem<E, T> {
    pub fn new(kernel: &Rc<Kernel<E>>) -> Self {
        Self {
            kernel: kernel.clone(),
            phantom_t: Default::default(),
        }
    }
}

impl<E: Environment, T: VectorTransferables> System for PopulateWorldSystem<E, T> {
    fn name(&self) -> Cow<'static, str> {
        "sdf_populate_world_system".into()
    }

    fn run(&mut self, MapContext { world, .. }: &mut MapContext) {
        for message in self.kernel.apc().receive(|message| {
            message.has_tag(T::SymbolLayerTessellated::message_tag())
                || message.has_tag(T::LayerIndexed::message_tag())
        }) {
            let message: Message = message;
            if message.has_tag(T::SymbolLayerTessellated::message_tag()) {
                let message = message.into_transferable::<T::SymbolLayerTessellated>();

                let Some(component) = world
                    .tiles
                    .query_mut::<&mut SymbolLayersDataComponent>(message.coords())
                else {
                    continue;
                };

                component
                    .layers
                    .push(SymbolLayerData::AvailableSymbolLayer(message.to_layer()));
            }
        }
    }
}

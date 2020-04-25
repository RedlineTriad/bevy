use crate::{render_resource::RenderResource, renderer::RenderContext, texture::Extent3d};
use std::sync::{Arc, Mutex};

pub enum Command {
    CopyBufferToBuffer {
        source_buffer: RenderResource,
        source_offset: u64,
        destination_buffer: RenderResource,
        destination_offset: u64,
        size: u64,
    },
    CopyBufferToTexture {
        source_buffer: RenderResource,
        source_offset: u64,
        source_bytes_per_row: u32,
        destination_texture: RenderResource,
        destination_origin: [u32; 3],
        destination_mip_level: u32,
        destination_array_layer: u32,
        size: Extent3d,
    },
    FreeBuffer(RenderResource),
}

#[derive(Default, Clone)]
pub struct CommandQueue {
    // TODO: this shouldn't really need a mutex. its just needs to be shared on whatever thread its scheduled on
    queue: Arc<Mutex<Vec<Command>>>,
}

impl CommandQueue {
    fn push(&mut self, command: Command) {
        self.queue.lock().unwrap().push(command);
    }

    pub fn copy_buffer_to_buffer(
        &mut self,
        source_buffer: RenderResource,
        source_offset: u64,
        destination_buffer: RenderResource,
        destination_offset: u64,
        size: u64,
    ) {
        self.push(Command::CopyBufferToBuffer {
            source_buffer,
            source_offset,
            destination_buffer,
            destination_offset,
            size,
        });
    }

    pub fn copy_buffer_to_texture(
        &mut self,
        source_buffer: RenderResource,
        source_offset: u64,
        source_bytes_per_row: u32,
        destination_texture: RenderResource,
        destination_origin: [u32; 3],
        destination_mip_level: u32,
        destination_array_layer: u32,
        size: Extent3d,
    ) {
        self.push(Command::CopyBufferToTexture {
            source_buffer,
            source_offset,
            source_bytes_per_row,
            destination_texture,
            destination_origin,
            destination_mip_level,
            destination_array_layer,
            size,
        });
    }

    pub fn free_buffer(&mut self, buffer: RenderResource) {
        self.push(Command::FreeBuffer(buffer));
    }

    pub fn execute(&mut self, render_context: &mut dyn RenderContext) {
        for command in self.queue.lock().unwrap().drain(..) {
            match command {
                Command::CopyBufferToBuffer {
                    source_buffer,
                    source_offset,
                    destination_buffer,
                    destination_offset,
                    size,
                } => render_context.copy_buffer_to_buffer(
                    source_buffer,
                    source_offset,
                    destination_buffer,
                    destination_offset,
                    size,
                ),
                Command::CopyBufferToTexture {
                    source_buffer,
                    source_offset,
                    source_bytes_per_row,
                    destination_texture,
                    destination_origin,
                    destination_mip_level,
                    destination_array_layer,
                    size,
                } => render_context.copy_buffer_to_texture(
                    source_buffer,
                    source_offset,
                    source_bytes_per_row,
                    destination_texture,
                    destination_origin,
                    destination_mip_level,
                    destination_array_layer,
                    size,
                ),
                Command::FreeBuffer(buffer) => render_context.resources().remove_buffer(buffer),
            }
        }
    }
}
//! Compute lib including shaders

use wgpu::{Adapter, Instance, RequestAdapterOptions};

pub async fn get_amd_vulkan(ins: &Instance) -> anyhow::Result<Option<Adapter>> {
    let prim_adap = ins.request_adapter(&RequestAdapterOptions::default()).await;
    Ok(prim_adap)
}

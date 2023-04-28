//! compute pipeline
use alonzo::compute::get_amd_vulkan;
use wgpu::{DeviceDescriptor, Features, Instance, Limits};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get the wgpu instance, and the primary AMD adapter
    let ins = Instance::default();
    let prim_adap = get_amd_vulkan(&ins).await?;
    let prim_adap = prim_adap
        .map(|a| {
            println!("{:#?}", a.get_info());
            a
        })
        .expect("Unable to retrieve an adapter from the system");

    // now get a (logical) device from the adapter
    let descriptor = DeviceDescriptor {
        label: None,
        features: Features::empty(),
        limits: Limits::downlevel_defaults(),
    };
    let (_device, _queue) = prim_adap.request_device(&descriptor, None).await?;

    Ok(())
}

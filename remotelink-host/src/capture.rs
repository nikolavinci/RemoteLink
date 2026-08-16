use std::time::{Duration, Instant};
use windows::core::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::*;

pub fn run_capture_loop() -> Result<()> {
    unsafe {
        // Create D3D11 device
        let mut d3d_device: Option<ID3D11Device> = None;
        let mut d3d_context: Option<ID3D11DeviceContext> = None;
        let feature_levels = [
            D3D_FEATURE_LEVEL_11_0,
            D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_10_0,
        ];

        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            Some(&feature_levels),
            D3D11_SDK_VERSION,
            Some(&mut d3d_device),
            None,
            Some(&mut d3d_context),
        )?;

        let d3d_device = d3d_device.unwrap();
        let dxgi_device: IDXGIDevice = d3d_device.cast()?;
        let dxgi_adapter: IDXGIAdapter = dxgi_device.GetAdapter()?;
        let dxgi_output: IDXGIOutput = dxgi_adapter.EnumOutputs(0)?;
        let dxgi_output1: IDXGIOutput1 = dxgi_output.cast()?;

        let dupl = match dxgi_output1.DuplicateOutput(&d3d_device) {
            Ok(d) => d,
            Err(e) => {
                println!(
                    "Failed to duplicate output. Is the display sleeping or running on hybrid graphics? Error: {}",
                    e
                );
                return Err(e.into());
            }
        };

        let mut frame_count = 0;
        let mut last_log = Instant::now();
        println!("Listening...");

        loop {
            let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
            let mut dxgi_resource: Option<IDXGIResource> = None;

            // Wait for up to 33ms (target 30 FPS)
            match dupl.AcquireNextFrame(33, &mut frame_info, &mut dxgi_resource) {
                Ok(_) => {
                    frame_count += 1;
                    
                    if let Err(e) = dupl.ReleaseFrame() {
                        println!("Failed to release frame: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    if e.code() == DXGI_ERROR_WAIT_TIMEOUT {
                        // Timeout is fine, just loop
                        continue;
                    } else if e.code() == DXGI_ERROR_ACCESS_LOST {
                        println!("Access lost (screen locked/resolution changed). Need to reinitialize.");
                        // For MVP, we break here. In full implementation, we'd loop back to re-init.
                        break;
                    } else {
                        println!("Error acquiring frame: {}", e);
                        break;
                    }
                }
            }

            if last_log.elapsed() >= Duration::from_secs(10) {
                println!(
                    "Captured {} frames in the last 10 seconds. FPS: {:.2}",
                    frame_count,
                    frame_count as f64 / 10.0
                );
                frame_count = 0;
                last_log = Instant::now();
            }
        }
    }

    Ok(())
}

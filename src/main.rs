use std::{
    default, error::Error, sync::Arc
};

use wgpu::wgc::device::{self, queue};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop}, window::{Window, WindowId}
};

// アプリケーションの状態を持つ構造体
struct App {
    // --- winit ----
    // ウィンドウは開く前は無いのでOption
    window:  Option<Arc<Window>>,
    
    // --- wgpu ---
    surface: Option<wgpu::Surface<'static>>,
    device:  Option<wgpu::Device>,
    queue:   Option<wgpu::Queue>,
    config:  Option<wgpu::SurfaceConfiguration>,
}

// winit
impl App {
    fn new() -> Self {
        Self {
            window:  None,
            surface: None,
            device:  None,
            queue:   None,
            config:  None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Fillstellar");

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            pollster::block_on(self.init_wgpu(window));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            // 閉じるボタンが押されたら終了
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            // リサイズでconfig更新
            WindowEvent::Resized(physical_size) => {
                if let (
                    Some(config), 
                    Some(surface), 
                    Some(device)
                ) = (
                    self.config.as_mut(), 
                    self.surface.as_ref(), 
                    self.device.as_ref()
                ) {
                    // サイズが1以上のときだけ更新
                    if physical_size.width > 0 && physical_size.height > 0 {
                        config.width = physical_size.width;
                        config.height = physical_size.height;
                        surface.configure(device, config);

                        // リサイズで再描画
                        self.window.as_ref().unwrap().request_redraw();
                    }
                }
            }

            WindowEvent::RedrawRequested => {
                // 初期化が終わってないときは無視
                let (
                    Some(surface),
                    Some(device),
                    Some(queue),
                    Some(window),
                ) = (
                    &self.surface,
                    &self.device,
                    &self.queue,
                    &self.window
                ) else { return };

                // 次のフレームを用意(ここに絵を描く)
                let frame = surface.get_current_texture().unwrap();
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

                // コマンドエンコーダー(GPUへの命令書を作る)
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                // レンダーパス(実際の描画スコープ)を開始
                {
                    let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                // 画面をこの色でクリアする（R, G, B, A）
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 1.0, // 少し暗い赤
                                    g: 0.2, // 緑
                                    b: 0.3, // 青
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                            depth_slice: None,
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                        multiview_mask: None,
                    });
                } // ここで_render_passがドロップされ、記録終了

                // コマンドをキューに積んで実行
                queue.submit(std::iter::once(encoder.finish()));
                
                // 画面に反映
                window.pre_present_notify();
                frame.present();

                // 次のフレームも描画し続けるなら
                // window.request_redraw();
            }
            _ => {}
        }
    }
}

// wgpu
impl App {
    async fn init_wgpu(&mut self, window: Arc<Window>) {
        let size = window.inner_size();

        // GraphicsAPI 自動選択
        let instance = wgpu::Instance::default();

        // Surfaceの作成(ウィンドウと紐づけ)
        let surface = instance.create_surface(window.clone()).unwrap();

        // Adapter(GPU)の取得
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        
        // DeviceとQueueの取得
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features:     wgpu::Features::empty(),
                    required_limits:       wgpu::Limits::default(),
                    memory_hints:          wgpu::MemoryHints::default(),
                    experimental_features: wgpu::ExperimentalFeatures::default(),
                    trace:                 wgpu::Trace::default(),
                },
            )
            .await
            .unwrap();

        // Surfaceの設定(解像度等)
        let surface_caps = surface.get_capabilities(&adapter);
        // スクリーンに適したフォーマットを選択
        let texture_format = surface_caps.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };
        surface.configure(&device, &config);  

        self.window = Some(window);
        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.config = Some(config);   
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new();

    // Wait: イベントが来るまで待機
    // Poll: 常に
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);    
    event_loop.run_app(&mut app)?;
    
    Ok(())
}
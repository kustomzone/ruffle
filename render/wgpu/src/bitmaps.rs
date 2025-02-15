#[derive(Debug)]
pub struct BitmapSamplers {
    pub repeat_linear: wgpu::Sampler,
    pub repeat_nearest: wgpu::Sampler,
    pub clamp_linear: wgpu::Sampler,
    pub clamp_nearest: wgpu::Sampler,
    pub clamp_u_repeat_v_linear: wgpu::Sampler,
    pub clamp_u_repeat_v_nearest: wgpu::Sampler,
    pub repeat_u_clamp_v_linear: wgpu::Sampler,
    pub repeat_u_clamp_v_nearest: wgpu::Sampler,
}

fn create_sampler(
    device: &wgpu::Device,
    address_mode_u: wgpu::AddressMode,
    address_mode_v: wgpu::AddressMode,
    filter: wgpu::FilterMode,
    sampler_label: Option<String>,
) -> wgpu::Sampler {
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: sampler_label.as_deref(),
        address_mode_u,
        address_mode_v,
        // FIXME - does this ever get used?
        address_mode_w: wgpu::AddressMode::Repeat,
        mag_filter: filter,
        min_filter: filter,
        mipmap_filter: filter,
        lod_min_clamp: 0.0,
        lod_max_clamp: 100.0,
        compare: None,
        anisotropy_clamp: None,
        border_color: None,
    });
    sampler
}

impl BitmapSamplers {
    pub fn new(device: &wgpu::Device) -> Self {
        let repeat_linear = create_sampler(
            device,
            wgpu::AddressMode::Repeat,
            wgpu::AddressMode::Repeat,
            wgpu::FilterMode::Linear,
            create_debug_label!("Repeat & Linear sampler"),
        );
        let repeat_nearest = create_sampler(
            device,
            wgpu::AddressMode::Repeat,
            wgpu::AddressMode::Repeat,
            wgpu::FilterMode::Nearest,
            create_debug_label!("Repeat & Nearest sampler"),
        );
        let clamp_linear = create_sampler(
            device,
            wgpu::AddressMode::ClampToEdge,
            wgpu::AddressMode::ClampToEdge,
            wgpu::FilterMode::Linear,
            create_debug_label!("Clamp & Linear sampler"),
        );
        let clamp_nearest = create_sampler(
            device,
            wgpu::AddressMode::ClampToEdge,
            wgpu::AddressMode::ClampToEdge,
            wgpu::FilterMode::Nearest,
            create_debug_label!("Clamp & Nearest sampler"),
        );

        let clamp_u_repeat_v_linear = create_sampler(
            device,
            wgpu::AddressMode::ClampToEdge,
            wgpu::AddressMode::Repeat,
            wgpu::FilterMode::Linear,
            create_debug_label!("Clamp U, Repeat V & Linear sampler"),
        );

        let clamp_u_repeat_v_nearest = create_sampler(
            device,
            wgpu::AddressMode::ClampToEdge,
            wgpu::AddressMode::Repeat,
            wgpu::FilterMode::Nearest,
            create_debug_label!("Clamp U, Repeat V & Nearest sampler"),
        );

        let repeat_u_clamp_v_linear = create_sampler(
            device,
            wgpu::AddressMode::Repeat,
            wgpu::AddressMode::ClampToEdge,
            wgpu::FilterMode::Linear,
            create_debug_label!("Repeat U, Clamp V & Linear sampler"),
        );

        let repeat_u_clamp_v_nearest = create_sampler(
            device,
            wgpu::AddressMode::Repeat,
            wgpu::AddressMode::ClampToEdge,
            wgpu::FilterMode::Nearest,
            create_debug_label!("Repeat U, Clamp V & Nearest sampler"),
        );

        Self {
            repeat_linear,
            repeat_nearest,
            clamp_linear,
            clamp_nearest,
            clamp_u_repeat_v_linear,
            clamp_u_repeat_v_nearest,
            repeat_u_clamp_v_linear,
            repeat_u_clamp_v_nearest,
        }
    }

    pub fn get_sampler(&self, is_repeating: bool, is_smoothed: bool) -> &wgpu::Sampler {
        match (is_repeating, is_smoothed) {
            (true, true) => &self.repeat_linear,
            (true, false) => &self.repeat_nearest,
            (false, true) => &self.clamp_linear,
            (false, false) => &self.clamp_nearest,
        }
    }
}

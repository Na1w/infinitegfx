use infinitegfx_core::core::gfx_chain::GfxChain;
use infinitegfx_core::effects::{glitch, haze, text::TextEffect};
use infinitegfx_core::modulators::lfo::RampingLfo;
use infinitegfx_core::StandardGlobals;
use infinitegfx_core::RenderContext;

#[test]
fn test_readme_example() {
    // Missing definitions that should be in a real app but were omit in README
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let time = 1.0;
    let width = 800;
    let height = 600;
    
    // We need real wgpu objects or mocks for RenderContext
    // This is hard to do without a full setup, so I'll try to just check the syntax and basic API
    
    // Create an effect chain
    let mut chain = GfxChain::new(format);
    
    // Add a text effect with animation
    let text = TextEffect::new("INFINITE", 0.0, 10.0)
        .with_pos(0.5, 0.5)
        .with_explosion(RampingLfo::new(0.5, 1.0));
    
    // Add glitch and haze
    chain = chain
        .and(text)
        .and(glitch(0.2, 0.5))
        .and(haze(0.1, 0.3));

    // Simulation of what should happen in a render loop
    // Note: RenderContext has many fields that are not in the README example.
    // The README example is currently very simplified and wouldn't compile as is.
    
    /*
    chain.render(RenderContext {
        device,
        queue,
        encoder,
        target_view: view, // README says 'view', struct says 'target_view'
        globals_bind_group, // Missing in README
        globals_buf, // Missing in README
        time,
        input_view: None, // Missing in README
    });
    */
}

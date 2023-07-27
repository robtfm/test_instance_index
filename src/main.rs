use naga;
use naga_oil::{self, compose::ComposableModuleDescriptor};

fn main() {
    let source = include_str!("shader.wgsl");

    let naga = naga::front::wgsl::parse_str(source).unwrap();
    let pp = format!("{naga:#?}");
    std::fs::write("raw_naga.txt", pp).unwrap();

    let mut composer = naga_oil::compose::Composer::default()
    // .with_capabilities(
    //     naga::valid::Capabilities {
    //         bits: todo!(), ???
    //     }
    // )
    ;
    composer.add_composable_module(ComposableModuleDescriptor{
        source: include_str!("mod.wgsl"),
        file_path: "mod.wgsl",
        ..Default::default()
    }).unwrap();
    let oily_naga = composer.make_naga_module(naga_oil::compose::NagaModuleDescriptor { 
        source: include_str!("top.wgsl"),
        file_path: "mod.wgsl",
        ..Default::default()
    }).unwrap();
    
    let opp = format!("{oily_naga:#?}");
    std::fs::write("oily_naga.txt", opp).unwrap();
}

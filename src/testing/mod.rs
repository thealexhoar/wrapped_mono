#![cfg(test)]
use rusty_fork::rusty_fork_test;
mod array;
mod benchmarks;
mod class;
mod delegate;
mod exception;
mod gc;
mod internal_call;
mod method;
mod object;
#[cfg(feature = "profiler_api")]
mod profiler;
mod reflection;
use crate as wrapped_mono;
use mstring::MString;
use wrapped_mono::*;
//use invokable::InvokePass;
use crate::assembly::Assembly;
rusty_fork_test! {
    #[test]
    fn jit_execution(){
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Jit.dll").unwrap();
        let mut args:Vec<&str> = Vec::new();
        args.push("1");
        args.push("2");
        let _res = jit::exec(&dom,&asm,args);
    }
    #[test]
    fn jit_init(){
        use wrapped_mono::jit;
        let _dom = jit::init("root",None);
    }
    #[should_panic]
    #[test]
    fn jit_init_twice(){
        use wrapped_mono::jit;
        let _dom = jit::init("root",None);
        let _dom2 = jit::init("root",None);
    }
    #[test]
    fn jit_init_version(){
        use wrapped_mono::jit;
        let _dom = jit::init("root",Some("v4.0.30319"));
    }
    #[test]
    fn multiple_domains(){
        use wrapped_mono::jit;
        use crate::domain::Domain;
        let _dom = jit::init("root",None);
        let _dom2 = Domain::create();
    }
    ///DOES NOT WORK.
    //#[test]
    fn unload_domain(){
        use wrapped_mono::jit;

        let _dom = jit::init("root",None);
    }
    #[test]
    fn assembly_loading(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        dom.assembly_open("test/dlls/Test.dll").unwrap();
    }
    #[test]
    fn metadata_acces_assembly(){
        use wrapped_mono::jit;
        use wrapped_mono::metadata::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let _asm_meta = AssemblyMetadata::from_image(img);
    }
    #[should_panic]
    #[test]
    fn metadata_acces_assembly_os(){
        use wrapped_mono::jit;
        use wrapped_mono::metadata::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let asm_meta = AssemblyOSMetadata::from_image(img).expect("No OS metadata!");
        panic!("{}",asm_meta);
    }
    #[should_panic]
    #[test]
    fn missing_assembly_loading(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        dom.assembly_open("test/dlls/Missing.dll").unwrap();
    }
    #[test]
    fn stop_jit(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        jit::cleanup(dom);
    }
    #[test]
    fn closing_image(){
        use crate as wrapped_mono;
        use wrapped_mono::jit;
        use wrapped_mono::class::Class;
        let main = jit::init("main",None);
        let asm = main.assembly_open("test/dlls/Pinvoke.dll").unwrap();
        let mut img = asm.get_image();
        let _test_class = Class::from_name(&img,"","Secondary").expect("Could not find class!");
        unsafe{img.close()};
    }
    #[test]
    fn create_mstring(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        let str_txt = "Test";
        let _ms = MString::new(&dom,str_txt);
    }
    #[test]
    fn mstring_hash(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        let s = MString::new(&dom,"Test");
        let s2 = MString::new(&dom,"Test");
        assert!(s.hash() == s2.hash());
    }
    #[test]
    fn get_mstring_content(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        let str_txt = "Test";
        let ms = MString::new(&dom,str_txt);
        assert!(str_txt == &ms.to_string());
    }
    #[test]
    fn getting_image_from_assembly(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let _img = asm.get_image();
    }
    #[test]
    fn getting_assembly_from_name(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        let _asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let _asm2 = Assembly::assembly_loaded("Test").expect("Could not get assembly!");
    }
    #[should_panic]
    #[test]
    fn getting_assembly_from_wrong_name(){
        use wrapped_mono::jit;
        let dom = jit::init("root",None);
        let _asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let _asm2 = Assembly::assembly_loaded("Tost").expect("Could not get assembly!");
    }
    #[test]
    fn gettig_class_from_image(){
        use wrapped_mono::{jit,class::Class};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let _class = Class::from_name(&img,"","TestFunctions");
    }
    #[test]
    fn bindgen(){
        use wrapped_mono::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        bindgen::BindingGenerator::create("target/test_bind.rs").unwrap().add_assembly(asm)
        .add_assembly(Assembly::assembly_loaded("mscorlib").unwrap())
        .generate().unwrap();
        panic!();
    }
    /*
    #[test]
    fn get_tuple_class(){
        use wrapped_mono::*;
        let dom = jit::init("root",None);
        let tuple_class = <(i32,u32) as InteropClass>::get_mono_class();
    }*/
}
use crate::wrapped_mono_macros::{InteropReceive, InteropSend};
#[derive(InteropReceive, InteropSend)]
struct Vec3 {
  x: f32,
  y: f32,
  z: f32,
}
use crate::binds::MonoObject;
struct CustomClass {
  object: Object,
}
impl InteropClass for CustomClass {
  fn get_mono_class() -> Class {
    let img = Assembly::assembly_loaded("MyAssembly")
      .expect("Assembly MyAssembly is not loaded, could not get CustomClass class!")
      .get_image();
    Class::from_name_case(&img, "Namespace", "CustomClass")
      .expect("Could not get Namespace.CustomClass class form MyAssembly !")
  }
}
impl ObjectTrait for CustomClass {
  fn get_ptr(&self) -> *mut MonoObject {
    self.object.get_ptr()
  }
  unsafe fn from_ptr_unchecked(ptr: *mut MonoObject) -> Self {
    let object = Object::from_ptr_unchecked(ptr);
    Self { object }
  }
}
//bindgen test
//include!("../../target/test_bind/mod.rs");

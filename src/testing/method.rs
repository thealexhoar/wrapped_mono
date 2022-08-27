use rusty_fork::rusty_fork_test;
use crate as wrapped_mono;
rusty_fork_test! {
    #[test]
    fn getting_method(){
        use wrapped_mono::{jit,class::Class,method::Method};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met = Method::get_method_from_name(&class,"GetArg",1).unwrap();
    }
    #[should_panic]
    #[test]
    fn getting_null_from_a_function(){
        use wrapped_mono::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met = Method::get_method_from_name(&class,"GetObject",0).unwrap();
        let obj = method_invoke!(met,None).expect("Got exception").expect("Got null as expected!");
        let _res = obj.unbox::<i32>();
    }
    #[test]
    fn calling_method(){
        use wrapped_mono::{jit,class::Class,method::Method};
        use crate::interop::{get_mono_rep_val,ref_to_cvoid_ptr};
        use macros::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        let mut arg1:i32 = 7;
        let obj = method_invoke!(met,None,arg1).expect("Exception").expect("Got null on a non-nullable!");
        let res = obj.unbox::<i32>();
        assert!(res == arg1);
    }
    #[test]
    fn getting_method_arg_count(){
        use wrapped_mono::*;
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        println!("method params:");
        assert!(met.get_param_count() == 1);
    }
    #[test]
    fn getting_method_arg_names(){
        use wrapped_mono::{jit,class::Class,method::Method};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let met = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        println!("method params:");
        assert!(met.get_param_count() == 1);
        for param in met.get_param_names(){
            println!("|{}|",param);
        }
    }
    #[should_panic]
    #[test]
    fn getting_missing_method(){
        use wrapped_mono::{jit,class::Class,method::Method};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met = Method::get_method_from_name(&class,"Missing",1).unwrap();
    }
    #[should_panic]
    #[test]
    fn gettig_missing_method_wrong_arg_count(){
        use wrapped_mono::{jit,class::Class,method::Method};
        let dom = jit::init("root",None);
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        let _met = Method::get_method_from_name(&class,"GetArg",3).unwrap();
    }
}

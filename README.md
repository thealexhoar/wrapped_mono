# wrapped-mono
<p align = "center">
     <a href="#About">About<a> 
     <a href="#Features">Features<a> 
     <a href="#Examples">Examples<a> 
</p>

## About
**wrapped-mono** is a wrapper around mono library, allowing for safe interreaction between rust code and mono runtime. **wrapped-mono** is WIP and does not support most of mono functionalities yet.
## Features
- [x] Mono JIT initalization  
- [x] Mono Domain creation
- [x] Loading mono assemblies
- [x] Executing code contained in mono assembly
- [ ] Getting image from assembly
- [ ] Getting classes from image
- [ ] Getting functions from image
- [ ] Getting functions from classes
- [ ] Static CLR function execution
- [ ] Constructing new class instances
- [ ] Calling CLR function on class instance
- [ ] Passing arguments to CLR functions
- [ ] Reciving data from CLR functions
- [ ] Getting accesing instance variable fields 
- [X] Exposing rust functions to CLR using P/Invoke **Works only partialy at the moment(only passing strings is working bug - free)**
- [ ] P/Invoke functions returning values to managed code
- [ ] Automplementation of InvokableArg trait using derive, supporting passing arbitrary types in P/Invoke functions
- [ ] Delegate Support
## Examples
<p align = "center">
    <a href="#Loading">Loading basic assembly<a>
    <a href="#Creating new domains">Creating new domains<a>
    <a href="#Executing manged code">Executing manged code<a>
    <a href="# Exposig rust functions using P/Invoke">Exposig rust functions using P/Invoke<a>
</p>

### Loading basic assembly
**WARNING** Mono JIT can only be initialized once, and calling Domain::init_jit more times will lead to crash.
```rust
use wraped_mono::*;
fn main(){
    //Initizlizing mono JIT and creating root domain with name "root" and no version specifincation (default runtime version)
    let domain = jit::init("root",None);
    //Loading assembly 'Test.dll'
    let assembly = domain.asembly_open("Test.dll").unwrap();
}
```
### Creating new domains
**WARNING**!<br> creating root domain and initializing JIT is a necesary step that must be done before creating other domains.
```rust
fn main(){
    let domain = jit::init("root",None);
    //creating another domain 
    let domain = Domain::create();
}
```
### Executing manged code
```rust
fn main(){
    //initalizing jit
    let domain = jit::init("root",None);
    //opening assembly
    let assemmbly = domain.assembly_open("Some.dll").unwrap();
    //creating structre containig arguments to be passed as string[] args
    let args:Vec<&str> = Vec::new();
    //calling main function in managed code
    jit::exec(dom,assembly,args);
}
```
### Exposig rust functions using P/Invoke
```cs
    class SomeClass{
        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void SomeFunction(string arg);
    }
```
```rust
    #[invokable]
    fn some_function(arg:String){
        println!("recived arg:'{}'!",arg)!
    }
    fn main(){
        /*
            jit initialization,domain creation, assembly loading, etc.
        */
        add_internal_call!("SomeClass::SomeFunction",some_function);
        /*
            executing managed code that calls SomeClass::SomeFunction
        */
    }
```
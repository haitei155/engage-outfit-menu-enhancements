//! Types and methods to manipulate the script system for events.

use unity::prelude::*;

use crate::gamedata::{unit::Unit, item::ItemData};

#[unity::from_offset("App", "ScriptSystem", "Log")]
pub fn scriptsystem_log(args: *const u8);

#[repr(C)]
#[unity::class("App", "EventScript")]
pub struct EventScript {
    pub main_process: *const u8,
    pub byte_code: *const u8,
    pub sources: *const u8,
    pub global_table: *const u8,
    pub debugger: *const u8,
    pub type_metatables: *const u8,
    pub options: *const u8,
    pub performance_stats: *const u8,
    pub registry: *const u8,
    pub name: &'static Il2CppString,
}

impl EventScript {
    pub fn get_instance() -> &'static EventScript {
        unsafe { eventscript_getinstance(None) }
    }
}

pub trait EventScriptCommand {
    fn register_action(&self, name: &str, action: extern "C" fn(&Il2CppArray<DynValue>, OptionalMethod));
}

pub trait EventResultScriptCommand {
    fn register_function(&self, name: &str, action: extern "C" fn(&Il2CppArray<DynValue>, OptionalMethod) -> &'static DynValue);
}

impl EventScriptCommand for EventScript {
    fn register_action(&self, name: &str, action: extern "C" fn(&Il2CppArray<DynValue>, OptionalMethod)) {
        unsafe {
            eventscript_registaction(self, EventScriptActionArgs::new(action).unwrap(), name.into(), None);
        }
    }
}

impl EventResultScriptCommand for EventScript {
    fn register_function(&self, name: &str, action: extern "C" fn(&Il2CppArray<DynValue>, OptionalMethod) -> &'static DynValue) {
        unsafe {
            eventscript_registfunction(self, EventScriptFunctionArgs::new(action).unwrap(), name.into(), None);
        }
    }
}

#[unity::class("", "EventScript.FunctionArgs")]

pub struct EventScriptFunctionArgs {
    pub method_ptr: extern "C" fn(&Il2CppArray<DynValue>, OptionalMethod) -> &'static DynValue,
    pub invoke_impl: *const u8,
    // Usually the ProcInst
    pub target: *const u8,
    // MethodInfo
    pub method: *const MethodInfo,
    __: [u8; 0x38],
    pub delegates: *const u8,
    // ...
}

impl EventScriptFunctionArgs {
    pub fn new(method: extern "C" fn(&Il2CppArray<DynValue>, OptionalMethod) -> &'static DynValue) -> Il2CppResult<&'static mut EventScriptFunctionArgs> {
        let function_args_class = EventScript::class().get_nested_types()[0];

        Il2CppObject::<EventScriptFunctionArgs>::from_class(function_args_class).map(|args| {
            // This is until helper methods are made to generated MethodInfos from Rust methods.
            let mut donor_method = Il2CppClass::from_name("App", "ScriptSystem").unwrap().get_method_from_name("MessIsExist", 1).unwrap().clone();

            donor_method.method_ptr = method as _;

            args.method_ptr = method;
            args.target = 0 as _;
            args.method = Box::leak(Box::new(donor_method)) as *mut MethodInfo;
            args
        })
    }
}

#[unity::class("", "EventScript.ActionArgs")]
pub struct EventScriptActionArgs {
    pub method_ptr: extern "C" fn(&Il2CppArray<DynValue>, OptionalMethod),
    pub invoke_impl: *const u8,
    // Usually the ProcInst
    pub target: *const (),
    // MethodInfo
    pub method: *const MethodInfo,
    __: [u8; 0x38],
    pub delegates: *const u8,
    // ...
}

impl EventScriptActionArgs {
    pub fn new(method: extern "C" fn(&Il2CppArray<DynValue>, OptionalMethod)) -> Il2CppResult<&'static mut EventScriptActionArgs> {
        let action_args_class = EventScript::class().get_nested_types()[1];

        Il2CppObject::<EventScriptActionArgs>::from_class(action_args_class).map(|args| {
            // This is until helper methods are made to generated MethodInfos from Rust methods.
            let mut donor_method = Il2CppClass::from_name("App", "ScriptSystem").unwrap().get_method_from_name("Log", 1).unwrap().clone();

            donor_method.method_ptr = method as _;

            args.method_ptr = method;
            args.target = 0 as _;
            args.method = Box::leak(Box::new(donor_method)) as *mut MethodInfo;
            args
        })
    }
}

#[repr(C)]
#[unity::class("System.IO", "MemoryStream")]
pub struct MemoryStream { }

#[repr(C)]
#[unity::class("MoonSharp.Interpreter", "DynValue")]
pub struct DynValue {
    pub ref_id: i32,
    pub hash_code: i32,
    pub read_only: bool,
    pub number: f64,
    // Gonna have to change this later
    pub object: &'static (),
    pub ty: i32,
}

impl DynValue {
    pub fn new_boolean(value: bool) -> &'static DynValue {
        unsafe { dynvalue_newboolean(value, None) }
    }
}

#[skyline::from_offset(0x2e200f0)]
fn dynvalue_newboolean(v: bool, method_info: OptionalMethod) -> &'static DynValue;

pub trait ScriptUtils {
    fn try_get_i32(&self, index: i32) -> i32;
    fn try_get_string(&self, index: i32) -> Option<&'static Il2CppString>;
    fn try_get_unit(&self, index: i32) -> Option<&'static Unit>;
    fn try_get_item(&self, arg_index: i32) -> Option<&'static ItemData>;
}

impl ScriptUtils for Il2CppArray<DynValue> {
    fn try_get_i32(&self, arg_index: i32) -> i32 {
        unsafe { scriptutil_trygetint(self, arg_index, i32::MAX, None) }
    }

    fn try_get_string(&self, arg_index: i32) -> Option<&'static Il2CppString> {
        unsafe { scriptutil_trygetstring(self, arg_index, "Couldn't read the value".into(), None) }
    }

    fn try_get_unit(&self, arg_index: i32) -> Option<&'static Unit> {
        unsafe { scriptutil_trygetunit(self, arg_index, true, None) }
    }

    fn try_get_item(&self, arg_index: i32) -> Option<&'static ItemData> {
        unsafe { scriptutil_trygetitem(self, arg_index, true, None) }
    }
}

#[skyline::from_offset(0x3371160)]
pub fn system_io_memorystream_ctor(this: &MemoryStream, buffer: &'static mut Il2CppArray<u8>, method_info: OptionalMethod);

#[skyline::from_offset(0x2196920)]
fn scriptutil_trygetstring(
    args: &Il2CppArray<DynValue>,
    index: i32,
    nothing: &Il2CppString,
    method_info: OptionalMethod,
) -> Option<&'static Il2CppString>;

#[skyline::from_offset(0x2196cd0)]
fn scriptutil_trygetint(args: &Il2CppArray<DynValue>, index: i32, nothing: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x21961f0)]
fn scriptutil_trygetunit(args: &Il2CppArray<DynValue>, index: i32, warning: bool, method_info: OptionalMethod) -> Option<&'static Unit>;

#[skyline::from_offset(0x21a3740)]
fn scriptutil_trygetitem(
    args: &Il2CppArray<DynValue>,
    index: i32,
    warning: bool,
    method_info: OptionalMethod,
) -> Option<&'static ItemData>;

#[skyline::from_offset(0x21994e0)]
pub fn scriptutil_yield(method_info: OptionalMethod);

#[unity::from_offset("App", "EventScript", "get_Instance")]
fn eventscript_getinstance(method_info: OptionalMethod) -> &'static EventScript;

#[unity::from_offset("MoonSharp.Interpreter", "Table", "Get")]
pub fn moonsharp_interpreter_table_get(this: *const u8, name: &Il2CppString, method_info: OptionalMethod) -> &'static DynValue;

#[unity::from_offset("MoonSharp.Interpreter", "Script", "DoStream")]
pub fn moonsharp_interpreter_script_dostream(
    this: &EventScript,
    stream: &MemoryStream,
    global_context: *const u8,
    code_friendly_name: *const u8,
    method_info: OptionalMethod,
) -> *const u8;

#[skyline::from_offset(0x24e2430)]
pub fn eventscript_registaction<T: EventScriptCommand>(
    this: &T,
    func: &EventScriptActionArgs,
    name: &Il2CppString,
    method_info: OptionalMethod,
);

#[unity::from_offset("App", "EventScript", "RegistFunction")]
pub fn eventscript_registfunction(
    this: &EventScript,
    func: &EventScriptFunctionArgs,
    name: &Il2CppString,
    method_info: OptionalMethod,
);
